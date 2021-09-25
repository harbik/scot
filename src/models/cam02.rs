#![doc = include_str!("./cam02/README.md")]

pub mod cam; // 8 correlates
pub mod ucs; // J'a'b'
pub mod jch; //JCh


use core::panic;
use std::{marker::PhantomData};
use nalgebra::{Matrix3x1, Matrix3xX, MatrixSlice3x1, SMatrix, matrix, vector};
use crate::{DefaultObserver, illuminants::{D50, Illuminant}, linterp, observers::{StandardObserver}};
use super::{CieLab, CieXYZ};

/*
   TODO:
   - CIECAT02 transform for non-cie1931 observer?
*/

pub static MHPE: SMatrix<f64, 3, 3> = matrix![
     0.38971, 0.68898, -0.07868;
    -0.22981, 1.18340,  0.04641;
     0.00000, 0.00000,  1.00000;
];

pub static MHPEINV: SMatrix<f64, 3, 3> = matrix![
    1.910197, -1.112124,  0.201908;
    0.370950,  0.629054, -0.000008;
    0.000000,  0.000000,  1.000000;
];

pub static MCAT02: SMatrix<f64, 3, 3> = matrix![
     0.7328,  0.4296,  -0.1624;
    -0.7036,  1.6975,   0.0061;
     0.0030,  0.0136,   0.9834;
];
pub static MCAT02INV: SMatrix<f64, 3, 3> = matrix![
     1.096124, -0.278869, 0.182745;
     0.454369,  0.473533, 0.072098;
    -0.009628, -0.005698, 1.015326;
];

/**
# CieCam View conditions

- SR100 = Sr * 100, Surround Ratio
- YB: Relative background luminance, relative to YW=100.0,
  20.0 is a common number, being 'world grey'.
- LA: Absolute Luminance (in cd/m<sup>2</sup>) of the adapting field.
  If unsure, use 20% of Lw; for example 20% * 300 cd/m<sup>2</sup> = 60 cd/m<sup>2</sup>.
- D100: Discounting-the-illuminant parameter, or color constancy parameter.
  This parameter determines if chromatic adaptation is applied to calculate the ciecam values of the target.
  This compensates for the chromatic adaptation of our eyes to ambient light.

  - D_AUTO: Calculate discounting factor automatically, using the other viewconditions parameters.
    Use this if you are not sure what value to use.
  - D100=0: No dicounting, so no chromatic adaptation applied.
    This can be used in case looking at a display, in a very dark room.
  - D100=100: Full discounting, full chromatic adaptation used.
    This value is used if you watching colorsamples in full daylight for example.
  - D100=0..100: discounting factor of D/100 is used.
    Set your own value, if you want to experiment.

*/
#[derive(Default)]
pub struct ViewConditions<const LA: usize, const YB: usize, const SR1000: usize, const D100: isize>;

/**
 # CieCam View Parameters

 Parameters derived from viewconditions, needed to calculate the CieCamValues.
 */
 #[derive(Debug)]
pub struct CieCamEnv<I=D50, C=DefaultObserver> {
	// Surround
	pub s_r: f64,
	pub c: f64,
	pub f: f64,
	pub n_c: f64, 

	// Ambient
	pub l_a: f64,
	pub k: f64,
	pub f_l: f64,
	pub d: f64,


	// Background
	pub y_b: f64,

	// Illuminant dependent parameters
	pub y_w: f64,
	pub n: f64,
	pub z: f64,
	pub n_bb: f64,
	pub n_cb: f64,
	pub d_rgb: Matrix3x1<f64>,
	pub a_w: f64,

	i: PhantomData<*const I>,
	obs: PhantomData<*const C>,
}

impl<I, C: StandardObserver> CieCamEnv<I, C> {
	pub fn post_adaptation_cone_response_from_xyz(&self, xyz: CieXYZ<C>) -> Matrix3xX<f64> {
		let n_samples = xyz.len();
		let rgb = &MCAT02 * xyz.data;
		let d_rgbs = Matrix3xX::from_iterator(n_samples, self.d_rgb.as_slice().iter().cycle().take(3*n_samples).cloned()); // repeat columns
		let rgb_c = d_rgbs.component_mul(&rgb);
		let rgb_p = MHPE * MCAT02INV * rgb_c;
		let rgb_p_a = rgb_p.map(|r|cone_adaptation(self.f_l, r));
		rgb_p_a

	}

	// A: Achromatic Response
	#[inline]
	pub fn achromatic_response(&self, rgb:MatrixSlice3x1<f64>) -> f64 {
			(2.0 * rgb.x + rgb.y + rgb.z/20.0 - 0.305) * self.n_bb // achromatic response
	}  

	#[inline]
	pub fn achromatic_response_from_lightness(&self, lightness: f64) -> f64 {
		self.a_w * (lightness/100.0).powf(1.0f64/(self.c * self.z))
	}

	// a: Redness-Greenness
	#[inline]
	pub fn red_green(&self, rgb:MatrixSlice3x1<f64>) -> f64 {
		rgb.x - 12.0 * rgb.y/11.0 + rgb.z/11.0
	}  

	// b: Blueness-Yellowness
	#[inline]
	pub fn blue_yellow(&self, rgb:MatrixSlice3x1<f64>) -> f64 {
		(rgb.x + rgb.y - 2.0 *  rgb.z) / 9.0
	}  

	#[inline]
	pub fn hue_angle(&self, red_green:f64, blue_yellow:f64) -> f64 {
		let theta = blue_yellow.atan2(red_green).to_degrees();
		if theta<0.0 { theta + 360.0}
		else {theta}
	}

	#[inline]
	pub fn lightness(&self, achromatic_response: f64) -> f64 {
			100.0 * (achromatic_response/self.a_w).powf(self.c*self.z)
	}  

	#[inline]
	pub fn brightness(&self, lightness: f64) -> f64 {
		4.0/self.c * (lightness/100.0).sqrt() * (self.a_w + 4.0) * self.f_l.powf(0.25)
	}

	#[inline]
	pub fn eccentricity(&self, hue_angle:f64) -> f64 {
		0.25 * ((hue_angle.to_radians() + 2.0).cos() + 3.8)

	}

	#[inline]
	pub fn chroma(&self, rgb:MatrixSlice3x1<f64>, lightness:f64, red_green:f64, blue_yellow:f64, hue_angle:f64) -> f64 {
	//	let eccentricity = 0.25 * ((hue_angle.to_radians() + 2.0).cos() + 3.8);
		let t = ((50_000.0/13.0 * self.n_c * self.n_cb) * self.eccentricity(hue_angle) * (red_green.powi(2) + blue_yellow.powi(2)).sqrt())/(rgb.x + rgb.y + 21.0 * rgb.z/20.0);
		t.powf(0.9) * (lightness/100.0).sqrt() * (1.64 - 0.29f64.powf(self.n)).powf(0.73)
	}

	#[inline]
	pub fn colorfulness(&self, chroma:f64) -> f64 {
		chroma * self.f_l.powf(0.25)		
	}

	#[inline]
	pub fn saturation(&self, brightness:f64, colorfulness:f64) -> f64 {
		100.0 * (colorfulness/brightness).sqrt()
	}

	#[inline]
	pub fn ucs_j_prime(&self, lightness:f64) -> f64 {
		0.7 * lightness / (1.0 + 0.007 * lightness)
	}

	#[inline]
	pub fn ucs_m_prime(&self, colorfulness:f64) -> f64 {
		43.8596 * (1.0 + 0.0228 * colorfulness).ln()
	}

	#[inline]
	pub fn ucs_ab_prime(&self, colorfulness:f64, hue_angle:f64) -> (f64,f64) {
		let (s,c) = hue_angle.sin_cos();
		(colorfulness*c, colorfulness*s)
	}

	fn hue_composition(&self, hue_angle:f64) -> f64 {
		let h = hue_angle;
		if h>=20.14 && h<=90.0 {
			(100.0*(h-20.14)/0.8)/(((h-20.14)/0.8)+(90.0-h)/0.7)
		} else if h>=90.0 && h<=164.25 {
			100.0+(100.0*(h-90.0)/0.7)/(((h-90.0)/0.7)+(164.25-h))
		} else if h>=164.25 && h<=237.53 {
			200.0+(100.0*(h-164.25))/((h-164.25)+((237.53-h)/1.2))
		} else if h>=237.53 && h<=380.14 {
			300.0+(100.0*(h-237.53) /1.2)/(((h-237.53)/1.2)+(380.14-h)/0.8)
		} else if h<20.14 {
			300.0+(100.0*((h+360.0)-237.53) /1.2)/((((h+360.0)-237.53)/1.2)+(380.14-(h+360.0))/0.8)
		} else {
			panic!("wrong hue angle")
		} 
	}


	const P1C: f64 = 50_000.0/13.0;
	const P3:f64 = 21.0/20.0;
	const NOM:f64 = (2.0+Self::P3)*460.0/1403.0; // this is 1!!
	const DEN1:f64 = (2.0+Self::P3)*220.0/1403.0;
	const DEN2:f64 = 27.0/1403.0 - Self::P3 * 6300.0/1403.0;

	pub fn xyz_from_jch(&self, jch: MatrixSlice3x1<f64>) -> Matrix3x1<f64> { // XYZ
		let &[lightness, chroma, hue_angle]:&[f64;3] = jch.as_ref();
		let t = ( chroma/( ( lightness/100.0).sqrt()*(1.64-0.29f64.powf(self.n)).powf(0.73))).powf(1.0/9.0);
		let p1 = Self::P1C * self.n_c * self.n_cb * self.eccentricity(hue_angle)/t;
		let p2 = self.achromatic_response_from_lightness(lightness)/self.n_bb + 0.305;
		let (hs, hc) = hue_angle.to_radians().sin_cos();
		let (a,b) = if t==0.0 {
			(0.0, 0.0)
		} else if hs.abs()>=hc.abs() {
			let b = p2 * Self::NOM/(p1/hs + Self::DEN1 * hc/hs - Self::DEN2);
			(b * hc/hs, b)
		} else {
			let a = p2 * Self::NOM / (p1/hc + Self::DEN1 - Self::DEN2 * hs/hc);
			(a, a * hs/hc)
		};
		let m =  matrix![ 460.0, 451.0, 288.0; 460.0, -891.0, -261.0; 460.0, -220.0, -6_300.0; ]/1_403.0;
		let mut rgb = m * vector![p2,a,b];  // rbg_pa
		rgb.apply(|x|inv_cone_adaptation(self.f_l, x)); // rgb_p
		let rgb_c = (&MCAT02 * MHPEINV * rgb).component_div(&self.d_rgb);
		MCAT02INV * rgb_c
	}

}
#[test]
fn test_nom(){
	println!("{}", CieCamEnv::<D50,DefaultObserver>::NOM);
}


/*
		Sr		 F		  c		 Nc
	>=0.15 	1.0		0.69	1.0	Average
	0.075	0.9		0.59	0.9	Dim // this is listed as 0 < Sr < 0.15
	0.0		0.8		0.525	0.8	Dark
*/

impl< C, I, const LA:usize, const YB:usize, const SR1000:usize, const D100:isize > 
	From<ViewConditions<LA,YB,SR1000,D100>> for CieCamEnv<I, C> 
	where 
		C: StandardObserver,
		I: Illuminant + Default + Into<CieXYZ<C>> 
	 {
    fn from(_: ViewConditions<LA,YB,SR1000,D100>) -> Self {
			// Surround dependent parameters
			let s_r = SR1000 as f64/1000.0;
			let c = 
				if s_r >= 0.15 {
					0.69
				} else if s_r > 0.075 {
					linterp(s_r, 0.075, 0.59, 0.15, 0.69)
				} else {
					linterp(s_r, 0.0, 0.525, 0.075, 0.59)
				};
			let f = 
				if c > 0.59 {
					linterp(c, 0.59, 0.9, 0.69, 1.0)
				} else {
					linterp(c, 0.525, 0.8, 0.59, 0.9)
				};
			let n_c = f;

			// Ambient parameters
			let l_a = LA as f64;
    		let k = 1. / (5. * l_a + 1.);
			let f_l = k.powi(4) * l_a + (1. - k.powi(4)).powi(2) / 10. * (5.0 * l_a).powf(1. / 3.);
			let d = match D100 {
					D_AUTO => { (f * (1.0 - (1.0 / 3.6) * ((-l_a - 42.0) / 92.0).exp())).clamp(0.0, 1.0) }
					_ => ((D100 as f64)/100.0).clamp(0.0, 1.0),
				};

			// Background
			let y_b = YB as f64;

			// Further Illuminant and Viewing Environment derived parameters
			let xyz_w: CieXYZ<C> = I::default().into()/* .normalize(100.0)*/;
			let y_w = xyz_w.data[(1,0)]; // = 100.0
			let n = y_b/y_w;
			let z = n.sqrt() + 1.48;
			let n_bb = 0.725 * n.powf(-0.2);
			let n_cb = n_bb;
			let rgb_w =  &MCAT02 * &xyz_w.data;
			let nom = Matrix3x1::from_element(d * y_w);
			let mut d_rgb = nom.component_div(&rgb_w);
			d_rgb.add_scalar_mut(1.0 - d);
			let rgb_wc = d_rgb.component_mul(&rgb_w);
			let rgb_p_w = &MHPE * &MCAT02INV * rgb_wc;
			let rgb_p_aw = rgb_p_w.map(|r| cone_adaptation(f_l,r));
			let a_w = (2.0 * rgb_p_aw.x + rgb_p_aw.y + rgb_p_aw.z/20.0-0.305)*n_bb;

		Self { s_r, c, f, n_c, l_a, k, f_l, d, y_b, y_w, n, z, n_bb, n_cb, d_rgb, a_w, i: PhantomData, obs:PhantomData} 
    }
}



type VcAvg = ViewConditions<318, 20, SR_AVG, D_AUTO>;

pub const SR_AVG: usize = 0.150E3 as usize;
pub const SR_DIM: usize = 0.075E3 as usize;
pub const SR_DARK: usize = 0;

pub const D_AUTO: isize = -1;


 #[inline]
 fn cone_adaptation(f_l:f64, x:f64) -> f64 {
	let t = (f_l * x/100.0).powf(0.42);
	400.0 * (t/(t+27.13)) + 0.1
 }

 #[inline]
 fn inv_cone_adaptation(f_l:f64, x:f64) -> f64 {
	let x = x - 0.1;
	let t = 27.13 * x.abs() / (400.0 - x.abs());
	x.signum()* 100.0/f_l * t.powf(1.0/0.42)
 }