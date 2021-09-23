#![doc = include_str!("./cam02/README.md")]

pub mod cam;
pub mod ucs;


use core::panic;
use std::{marker::PhantomData};
use nalgebra::{Matrix3x1, Matrix3xX, MatrixSlice3x1, SMatrix, matrix};
use crate::{illuminants::{ Illuminant}, linterp, observers::{
		StandardObserver
	}};
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
pub struct CieCamEnv<I, C> {
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
	pub fn achromatic_response(&self, rgb:MatrixSlice3x1<f64>) -> f64 {
			(2.0 * rgb.x + rgb.y + rgb.z/20.0 - 0.305) * self.n_bb // achromatic response
	}  

	// a: Redness-Greenness
	pub fn red_green(&self, rgb:MatrixSlice3x1<f64>) -> f64 {
		rgb.x - 12.0 * rgb.y/11.0 + rgb.z/11.0
	}  

	// b: Blueness-Yellowness
	pub fn blue_yellow(&self, rgb:MatrixSlice3x1<f64>) -> f64 {
		(rgb.x + rgb.y - 2.0 *  rgb.z) / 9.0
	}  

	pub fn hue_angle(&self, red_green:f64, blue_yellow:f64) -> f64 {
		let theta = blue_yellow.atan2(red_green).to_degrees();
		if theta<0.0 { theta + 360.0}
		else {theta}
	}

	pub fn lightness(&self, achromatic_response: f64) -> f64 {
			100.0 * (achromatic_response/self.a_w).powf(self.c*self.z)
	}  

	pub fn brightness(&self, lightness: f64) -> f64 {
		4.0/self.c * (lightness/100.0).sqrt() * (self.a_w + 4.0) * self.f_l.powf(0.25)
	}

	pub fn chroma(&self, rgb:MatrixSlice3x1<f64>, lightness:f64, red_green:f64, blue_yellow:f64, hue_angle:f64) -> f64 {
		let eccentricity = 0.25 * ((hue_angle.to_radians() + 2.0).cos() + 3.8);
		let t = ((50_000.0/13.0 * self.n_c * self.n_cb) * eccentricity * (red_green.powi(2) + blue_yellow.powi(2)).sqrt())/(rgb.x + rgb.y + 21.0 * rgb.z/20.0);
		t.powf(0.9) * (lightness/100.0).sqrt() * (1.64 - 0.29f64.powf(self.n)).powf(0.73)
	}

	pub fn colorfulness(&self, chroma:f64) -> f64 {
		chroma * self.f_l.powf(0.25)		
	}

	pub fn saturation(&self, brightness:f64, colorfulness:f64) -> f64 {
		100.0 * (colorfulness/brightness).sqrt()
	}

	pub fn ucs_j_prime(&self, lightness:f64) -> f64 {
		0.7 * lightness / (1.0 + 0.007 * lightness)
	}

	pub fn ucs_m_prime(&self, colorfulness:f64) -> f64 {
		43.8596 * (1.0 + 0.0228 * colorfulness).ln()
	}

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

/*

pub struct CieCam<V = VcAvg, I = D65, C = DefaultObserver> {
	pub data: OMatrix<f64, Const<9>, Dynamic>, 
	v: PhantomData<*const V>,
	i: PhantomData<*const I>,
	c: PhantomData<*const C>,
}

impl<V, I, C> CieCam<V, I, C> {

    pub fn new(data: OMatrix<f64, Const<9>, Dynamic>) -> Self { 
		Self { data, i:PhantomData, c:PhantomData, v:PhantomData } 
	}

	pub fn len(&self) -> usize {
        self.data.ncols()
    }
}

impl<V,I,C,L> From<L> for CieCam<V,I,C>
where
	I: Default + Into<CieXYZ<C>>,
	L: Into<CieLab<I,C>>,
	C: StandardObserver,
	V: Default + Into<CieCamEnv<I,C>>,
{
    fn from(samples: L) -> Self {
		let cam: CieCamEnv<I,C> = V::default().into();
		let lab: CieLab<I,C> = samples.into();
		let n_samples = lab.len();
		let rgb_pa = cam.post_adaptation_cone_response_from_xyz(lab.into());

		// 9xX Matrix, with 9 correlates in rows J, Q, a, b, C, M, s, h, H for the number of input samples
		let mut vdata: Vec<f64> = Vec::with_capacity(9*n_samples);
		for rgb in rgb_pa.column_iter() {
			let achromatic_response = cam.achromatic_response(rgb); // achromatic response
			let lightness = cam.lightness(achromatic_response); // J
			let brightness = cam.brightness(lightness); // Q
			let red_green = cam.red_green(rgb); // a
			let blue_yellow = cam.blue_yellow(rgb); // b
			let hue_angle = cam.hue_angle(red_green, blue_yellow); // h
			let hue_composition = hue_composition_from_hue_angle(hue_angle); // H
			let chroma = cam.chroma(rgb, lightness, red_green, blue_yellow, hue_angle); // /C
			let colorfulness = cam.colorfulness(chroma); // M
			let saturation = cam.saturation(brightness, colorfulness); // s
			vdata.append(&mut vec![lightness, brightness, red_green, blue_yellow, chroma, colorfulness, saturation, hue_angle, hue_composition]);
		}
		let data = OMatrix::<f64, Const::<9>, Dynamic>::from_vec(vdata);
		Self::new(data)
    }
}

#[test]
fn test_from_lab(){
	use crate::observers::CieObs1931;
	use approx::assert_relative_eq;
	use crate::illuminants::D50;
	let lab: CieLab<D50> = CieLab::new(Matrix3xX::from_vec(vec![
		50.0, 0.0, 0.0,
		50.0, -20.0, 20.0,
		50.0, 20.0, -20.0,
		50.0, -20.0, -20.0,
		0.0, 0.0, 0.0,
		100.0, 100.0, 0.0,
		100.0, 0.0, 100.0,
		100.0, 0.0, -100.0,
		100.0, 100.0, -100.0,
	]));
	let cam: CieCam<ViewConditions<32, 20, SR_AVG, D_AUTO>, D50, CieObs1931> = lab.into();
	// From ciecam02.xls by Eric Walowit and Grit O'Brien <https://web.archive.org/web/20070109143710/http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS>
	// see also cielab.xyz
	let want = OMatrix::<f64,Const::<9>, Dynamic>::from_vec(vec![
		39.614, 118.490, -0.005, 0.011, 1.104, 0.948, 8.944, 112.539, 138.373,
		38.867, 117.368, -0.271, 0.263, 28.643, 24.586, 45.769, 135.844, 169.748,
		40.378, 119.628, 0.256, -0.251, 28.455, 24.425, 45.186, 315.552, 344.608,
		38.683, 117.090, -0.338, -0.270, 37.260, 31.983, 52.263, 218.554, 277.448,
		0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 180.000, 224.729,
		106.226, 194.033, 1.952, 0.068, 100.220, 86.026, 66.585, 1.993, 382.054,
		99.751, 188.026, -0.125, 1.421, 79.525, 68.262, 60.253, 95.020, 109.386,
		98.294, 186.648, -0.620, -1.544, 97.974, 84.098, 67.125, 248.127, 305.080,
		105.470, 193.342, 1.379, -1.481, 105.655, 90.691, 68.489, 312.958, 342.808
	]);
	println!("{:.3}", cam.data.transpose());
	for (c,w) in cam.data.iter().zip(want.iter()){
		assert_relative_eq!(c, w, epsilon=1E-3, max_relative=5E-4); // abs<1.E-3 or rel<5E-4

	}
}

#[test]
fn test_from_lab2(){
	use crate::observers::CieObs1931;
	use crate::illuminants::D50;
//	use nalgebra::{dmatrix, convert};
	let lab: CieLab<D50> = CieLab::new(Matrix3xX::from_vec(vec![
		50.0, 0.0, 0.0,
	]));
	let cam: CieCam<ViewConditions<32, 20, SR_AVG, D_AUTO>, D50, CieObs1931> = lab.into();
//	let want = matrix![
//
	//];

	println!("{}", cam.data);
}
 */
/*
pub struct CieCamUcs<V = VcAvg, I = D65, C = DefaultObserver> {
	pub data: OMatrix<f64, Const<3>, Dynamic>, 
	v: PhantomData<*const V>,
	i: PhantomData<*const I>,
	c: PhantomData<*const C>,
}
impl<V, I, C> CieCamUcs<V, I, C> {

    pub fn new(data: OMatrix<f64, Const<3>, Dynamic>) -> Self { 
		Self { data, i:PhantomData, c:PhantomData, v:PhantomData } 
	}

	pub fn len(&self) -> usize {
        self.data.ncols()
    }
}

impl<V,I,C,L> From<L> for CieCamUcs<V,I,C>
where
	I: Default + Into<CieXYZ<C>>,
	L: Into<CieLab<I,C>>,
	C: StandardObserver,
	V: Default + Into<CieCamEnv<I,C>>,
{
    fn from(samples: L) -> Self {
		let cam: CieCamEnv<I,C> = V::default().into();

		// Calculate XYZ values from CieLab input data
		let lab: CieLab<I,C> = samples.into();
		let n_samples = lab.len();
		let xyz: CieXYZ<C> = lab.into();
		let rgb_pa = cam.post_adaptation_cone_response_from_xyz(xyz);

		// 3xX Matrix, CIECAM-UCS (J', a', b')
		let mut vdata: Vec<f64> = Vec::with_capacity(3*n_samples);
		for rgb in rgb_pa.column_iter() {
			let achromatic_response = cam.achromatic_response(rgb); // A
			let lightness = cam.lightness(achromatic_response); // J
			let red_green = cam.red_green(rgb); // a
			let blue_yellow = cam.blue_yellow(rgb); // b
			let hue_angle = cam.hue_angle(red_green, blue_yellow); // h
			let chroma = cam.chroma(rgb, lightness, red_green, blue_yellow, hue_angle); // C
			let colorfulness = cam.colorfulness(chroma); // M
			let j_prime = cam.ucs_j_prime(lightness); // CIECAM UCS J'
			let (a_prime, b_prime) = cam.ucs_ab_prime(colorfulness, hue_angle); // CIECAM UCS (a',b')
			vdata.append(&mut vec![j_prime, a_prime, b_prime]);
		}
		let data = OMatrix::<f64, Const::<3>, Dynamic>::from_vec(vdata);
		Self::new(data)
    }
}
 */



 #[inline]
 fn cone_adaptation(f_l:f64, x:f64) -> f64 {
	let t = (f_l * x/100.0).powf(0.42);
	400.0 * (t/(t+27.13)) + 0.1
 }
