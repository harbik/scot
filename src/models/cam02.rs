#![doc = include_str!("cam02.md")]

use core::panic;
use std::{marker::PhantomData};
use nalgebra::{Const, Dynamic, Matrix3x1, Matrix3xX, OMatrix, SMatrix, matrix};
use crate::{
	DefaultObserver, 
	linterp, 
	illuminants::{
		D65, D50
	}, 
	observers::{
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

pub static HUE_ANGLE_PARAMETERS: SMatrix<f64, 3, 5> = matrix![
	/* h_i */	20.14, 	90.0, 	164.25, 237.53, 380.14;
	/* e_i */	0.8,	0.7,	1.0,	1.2,	0.8;
	/* H_i */	0.0,	100.0,	200.0,	300.0,	400.0;
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
pub struct CieCamViewParameters {
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
}

/*
		Sr		 F		  c		 Nc
	>=0.15 	1.0		0.69	1.0	Average
	0.075	0.9		0.59	0.9	Dim // this is listed as 0 < Sr < 0.15
	0.0		0.8		0.525	0.8	Dark
*/

impl<
	const LA:usize, 
	const YB:usize, 
	const SR1000:usize, 
	const D100:isize
	> From<ViewConditions<LA,YB,SR1000,D100>> for CieCamViewParameters {
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

		Self { s_r, c, f, n_c, l_a, k, f_l, d, y_b } 
    }
}



type VcAvg = ViewConditions<318, 20, SR_AVG, D_AUTO>;

#[test]
fn test_vc_avg(){
	// AppModEx.xls From MarkFairchild.org's site
	use approx::assert_abs_diff_eq;
	let p: CieCamViewParameters = ViewConditions::<32, 20, SR_AVG, D_AUTO>::default().into();
	let CieCamViewParameters{c,d,k,f_l, f,.. } = p;
	assert_abs_diff_eq!(c, 0.69);
	assert_abs_diff_eq!(k, 0.0062112, epsilon=5E-7);
	assert_abs_diff_eq!(d, 0.87573, epsilon=5E-6);
	assert_abs_diff_eq!(f_l, 0.5429, epsilon=5E-5);
	assert_abs_diff_eq!(f, 1.0, epsilon=5E-5);
}

pub const SR_AVG: usize = 0.150E3 as usize;
pub const SR_DIM: usize = 0.075E3 as usize;
pub const SR_DARK: usize = 0;

pub const D_AUTO: isize = -1;


// Illumiant reference white Lw


// Table 16.4, Mark Fairchild, Color Appearance Models
// /*X	Y	Z*/	Xw	Yw	Zw	LA	F	D	Yb	Nc	Fl	Nbb,Ncb	h	H	/*Hc*/	J	Q	S	C	M	ac	bc	am	bm	as	bs

#[doc(hidden)]
pub static CIECAM_WANT: SMatrix<f64, 4, 23> = matrix![
/*19.01, 20.0, 21.78,*/ 95.05, 100.0, 108.88, 318.31, 1.0, 0.994, 20.0, 1.0, 1.17, 1.0, 219.0, 278.1, /*"78B, 22G",*/ 41.73, 195.37, 2.36, 0.1, 0.11, -0.08, -0.07, -0.08, -0.07, -1.83, -1.49;
/*57.06, 43.06, 31.96,*/ 95.05, 100.0, 108.88, 31.83, 1.0, 0.875, 20.0, 1.0, 0.54, 1.0, 19.6, 399.6, /*100R,*/ 65.96, 152.67, 52.25, 48.57, 41.67, 45.77, 16.26, 39.27, 13.95, 49.23, 17.49;
/*3.53, 6.56, 2.14,*/ 109.85, 100.0, 35.58, 318.31, 1.0, 0.994, 20.0, 1.0, 1.17, 1.0, 177.1, 220.4, /*"80G, 20B",*/ 21.79, 141.17, 58.79, 46.94, 48.8, -46.89, 2.43, -48.74, 2.43, -58.72, 2.93;
/*19.01, 20.0, 21.78,*/ 109.85, 100.0, 35.58, 31.83, 1.0, 0.875, 20.0, 1.0, 0.54, 1.0, 248.9, 305.8, /*"94B, 6R",*/ 42.53, 122.83, 60.22, 51.92, 44.54, -18.69, -48.44, -16.03, -41.56, -21.67, -56.18;
];

pub struct CieCam<V = VcAvg, I = D65, C = DefaultObserver> {
	pub data: OMatrix<f64, Const<9>, Dynamic>, 
	v: PhantomData<*const V>,
	i: PhantomData<*const I>,
	c: PhantomData<*const C>,
}

pub enum CieCamCorrelate {
	Brightness = 0,
	Lightness = 1,
	RednessGreenness = 2,
	YellownessBlueness = 3,
	HueAngle = 4,
	HueComposition = 5,
	Chroma = 6,
	Colorfulness = 7,
	Saturation = 8,

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
	V: Default + Into<CieCamViewParameters>,
{
    fn from(samples: L) -> Self {
		let vc: CieCamViewParameters = V::default().into();
		let xyz_w: CieXYZ<C> = I::default().into()/* .normalize(100.0)*/;
		let y_w = xyz_w.data[(1,0)]; // = 100.0
		let n = vc.y_b/y_w;
		let z = n.sqrt() + 1.48;
		let n_bb = 0.725 * n.powf(-0.2);
		let n_cb = n_bb;
		let rgb_w =  &MCAT02 * &xyz_w.data;
		let nom = Matrix3x1::from_element(vc.d * y_w);
		let mut d_rgb = nom.component_div(&rgb_w);
		d_rgb.add_scalar_mut(1.0 - vc.d);
		let rgb_wc = d_rgb.component_mul(&rgb_w);
		let rgb_p_w = &MHPE * &MCAT02INV * rgb_wc;

		// RGB'<sub>a</sub> Post-adaptation cone responses
		let rgb_p_aw = rgb_p_w.map(|r|{
			let t = (vc.f_l * r/100.0).powf(0.42);
			400.0 * (t/(t+27.13)) + 0.1
		});

		// Achromatic Response for reference white
		let a_w = (2.0 * rgb_p_aw.x + rgb_p_aw.y + rgb_p_aw.z/20.0-0.305)*n_bb;

		// Calculate XYZ values from CieLab input data
		let lab: CieLab<I,C> = samples.into();
		let n_samples = lab.len();
		let xyz: CieXYZ<C> = lab.into();


		let rgb = &MCAT02 * xyz.data;
		let d_rgbs = Matrix3xX::from_fn(rgb.ncols(), |i,_j|
			match i {
				0 => d_rgb.x,
				1 => d_rgb.y,
				2 => d_rgb.z,
				_ => panic!("index error!"),
			}
		);
		let rgb_c = d_rgbs.component_mul(&rgb);
		let rgb_p = MHPE * MCAT02INV * rgb_c;
		let rgb_p_a = rgb_p.map(|r|{
			if r>=0.0 {
				let t = (vc.f_l * r/100.0).powf(0.42);
				400.0 * (t/(t+27.13)) + 0.1
			} else {
				let t = (-vc.f_l * r/100.0).powf(0.42);
				-400.0 * (t/(t+27.13)) + 0.1
			}
		});
		

		// 9xX Matrix, with 9 correlates in rows Q, J, a, b, h, H, C, M, s for the number of input samples

		let mut vdata: Vec<f64> = Vec::with_capacity(9*n_samples);
		for j in 0..n_samples {

			let rp = rgb_p_a.column(j).x;
			let gp = rgb_p_a.column(j).y;
			let bp = rgb_p_a.column(j).z;
			let achromatic_response = (2.0 * rp + gp + bp/20.0 - 0.305) * n_bb; // achromatic response

			// Lightness (J), Red-Greenness (a) and Blue-Yellowness (b)
			let lightness = 100.0 * (achromatic_response/a_w).powf(vc.c*z);
			let brightness = 4.0/vc.c * (lightness/100.0).sqrt() * (a_w + 4.0) * vc.f_l.powf(0.25);
			let red_green = rp - 12.0 * gp/11.0 + bp/11.0; // a
			let blue_yellow = (rp + gp - 2.0 *  bp) / 9.0; // b

			// Hue angle (h)
			let hue_angle = {
				let theta = blue_yellow.atan2(red_green).to_degrees();
				if theta<0.0 { theta + 360.0}
				else {theta}
			};
			//
			
			/*
			// Hue composition (H)
			let hp = if hue_angle<HUE_ANGLE_PARAMETERS[(0,0)] {
				hue_angle + 360.0
			} else {
				hue_angle
			};
			let m: usize = 
				if      hp >= HUE_ANGLE_PARAMETERS[(0,0)] && hp < HUE_ANGLE_PARAMETERS[(0,1)] { 0 }
				else if hp >= HUE_ANGLE_PARAMETERS[(0,1)] && hp < HUE_ANGLE_PARAMETERS[(0,2)] { 1 }
				else if hp >= HUE_ANGLE_PARAMETERS[(0,2)] && hp < HUE_ANGLE_PARAMETERS[(0,3)] { 2 }
				else if hp >= HUE_ANGLE_PARAMETERS[(0,3)] && hp < HUE_ANGLE_PARAMETERS[(0,4)] { 3 }
				else { panic!("Hue angle out of range");
			};
			println!("m: {}", m);
			println!("hp: {}", hp);
			let hi = HUE_ANGLE_PARAMETERS[(0,m)];
			let hl = HUE_ANGLE_PARAMETERS[(2,m)];
			let hr = HUE_ANGLE_PARAMETERS[(2,m+1)];
			let el = HUE_ANGLE_PARAMETERS[(1,m)];
			let er = HUE_ANGLE_PARAMETERS[(1,m+1)]; 
			//let hue_composition = hi + (100.0 * (hp - hl)/el) / ((hp-hl)/el + (hr-hp)/er);
			*/
			let hue_composition = hue_composition_from_hue_angle(hue_angle);

			// Chroma (C), Colorfulness (M), and saturation (S)
			let eccentricity = 0.25 * ((hp.to_radians() + 2.0).cos() + 3.8);
			println!("e_t*** {}", eccentricity);
			println!("e_t*** Fairchild {}", 12500.0/13.0*4.0*eccentricity);
			let t = ((50_000.0/13.0 * vc.n_c * n_cb) * eccentricity * (red_green.powi(2) + blue_yellow.powi(2)).sqrt())/(rp + gp + 21.0 * bp/20.0);
			println!("t*** {}", t);
			let chroma = t.powf(0.9) * (lightness/100.0).sqrt() * (1.64 - 0.29f64.powf(n)).powf(0.73);
			let colorfulness = chroma * vc.f_l.powf(0.25);
			let saturation = 100.0 * (colorfulness/brightness).sqrt();
			vdata.push(lightness); //  J
			vdata.push(brightness); // Q
			vdata.push(red_green); // a
			vdata.push(blue_yellow); // b
			vdata.push(chroma); // C
			vdata.push(colorfulness); // M
			vdata.push(saturation); // s
			vdata.push(hue_angle); // h
			vdata.push(hue_composition); // H

		}
		let data = OMatrix::<f64, Const::<9>, Dynamic>::from_vec(vdata);
		Self::new(data)
    }
}

#[test]
fn test_from_lab(){
	use crate::observers::CieObs1931;
	let lab: CieLab<D50> = CieLab::new(Matrix3xX::from_vec(vec![
	//	19.01, 20.0, 21.78, 
		71.5957, 44.2271, 18.1105,
//		3.53, 6.56, 2.14,
//		19.01, 20.0, 21.78,
	]));
	let cam: CieCam<ViewConditions<32, 20, SR_AVG, D_AUTO>, D50, CieObs1931> = lab.into();
	println!("{}", cam.data);
}

/*
pub type VcAverage = ViewConditions<AVG, 100, 690, 100, false>;
pub type VcDim= ViewConditions<DIM, 90, 590, 90, false>;
pub type VcDark = ViewConditions<DARK, 80, 525, 80, false>;
 */

/*

#[derive(Debug, Serialize, Clone)]
pub struct CIECAM {
    pub vc: ViewConditions,
    pub jch: Vec<[f64; 3]>,
    pub xyz_w: [f64; 3],
    pub obs: ObserverKey,
}

impl CIECAM {
    pub fn from(xyz: XYZ, vc: ViewConditions) -> CIECAM {
        // See Luo CIECAM02 and its recent developments Appendix A, part 1: The forward mode
        let xyz_w = xyz.xyz_w.expect("Need white point");
        let mut rgb_w = m_cat02(xyz_w);
        let vcd = vc.D();
        let fl = vc.Fl();

        let Yw = xyz_w[1];
        let dr = vcd * Yw / rgb_w[0] + 1.0 - vcd;
        let dg = vcd * Yw / rgb_w[1] + 1.0 - vcd;
        let db = vcd * Yw / rgb_w[2] + 1.0 - vcd;


        let n = vc.Yb / Yw;
        let z = n.sqrt() + 1.48;
        let Nbb = 0.725 * n.powf(-0.2);
        let Ncb = Nbb;

        // rgb_wc
        rgb_w[0] *= dr; rgb_w[1] *= dg; rgb_w[2] *= db;

        // rgb_pw
        rgb_w = m_cat02_inv(rgb_w);
        rgb_w = m_hpe(rgb_w);

        // rgb_paw
        rgb_w = vc.lum_adapt(rgb_w);

        let Aw = achromatic_rsp(rgb_w, Nbb);

        let mut jch: Vec<[f64; 3]> = Vec::with_capacity(xyz.len());

        for _xyz in xyz.xyz_vec {
            let mut rgb = m_cat02(_xyz);

            // rgb_C
            rgb[0] *= dr; rgb[1] *= dg; rgb[2] *= db;

            // rgb_p
            rgb = m_cat02_inv(rgb);
            rgb = m_hpe(rgb);

            // rgb_pa
            rgb = vc.lum_adapt(rgb);

            let ca = rgb[0] - 12.0 / 11.0 * rgb[1] + rgb[2] / 11.0;
            let cb = (rgb[0] + rgb[1] - 2. * rgb[2]) / 9.0;

            // calculate h in radians
            let mut h = cb.atan2(ca);
            if h<0.0 { h+= 2.0* PI;}


            // calculate J = jj
            let jj = 100.0 * (achromatic_rsp(rgb, Nbb) / Aw).powf(vc.c * z);

            // calculate C = cc
            let et = 0.25f64 * ((h + 2.0).cos() + 3.8);
            let t = (50000.0 / 13.0 * Ncb * vc.Nc * et * (ca * ca + cb * cb).sqrt()) / (rgb[0] + rgb[1] + 21.0 / 20.0 * rgb[2]);
            let cc = t.powf(0.9) * (jj / 100.).sqrt() * (1.64 - (0.29f64).powf(n)).powf(0.73);

            jch.push([jj, cc, h * 180.0/PI]);
        }

        CIECAM { vc, jch, xyz_w, obs: xyz.obs }
    }

    // Get J'a'b' coordinates
    pub fn jab_p(&self) -> Vec<[f64; 3]> {

        let mut jab_p_vec : Vec<[f64;3]> = Vec::with_capacity(self.jch.len());

        for [jj,cc,h] in &self.jch {
            let M = cc * self.vc.Fl().powf(0.25);
            let MPrime = 1.0 / 0.0228 * (1.0 + 0.0228 * M).ln();
            jab_p_vec.push(
                [
                    (1.0 + 100.0 * 0.007) * jj / (1.0 + 0.007 * jj),
                    MPrime * (h * PI / 180.0).cos(),
                    MPrime * (h * PI / 180.0).sin(),
                ]
            );
        }
        jab_p_vec
    }
}

pub fn m_cat02(xyz: [f64; 3]) -> [f64; 3] {
    [
        0.7328 * xyz[0] + 0.4296 * xyz[1] - 0.1624 * xyz[2],
        -0.7036 * xyz[0] + 1.6975 * xyz[1] + 0.0061 * xyz[2],
        0.0030 * xyz[0] + 0.0136 * xyz[1] + 0.9834 * xyz[2],
    ]
}

pub fn m_cat02_inv(rgb: [f64; 3]) -> [f64; 3] {
    [
        1.096124 * rgb[0] - 0.278869 * rgb[1] + 0.182745 * rgb[2],
        0.454369 * rgb[0] + 0.473533 * rgb[1] + 0.072098 * rgb[2],
        -0.009628 * rgb[0] - 0.005698 * rgb[1] + 1.015326 * rgb[2],
    ]
}

pub fn m_hpe(rgb: [f64; 3]) -> [f64; 3] {
    [
        0.38971 * rgb[0] + 0.68898 * rgb[1] - 0.07868 * rgb[2],
        -0.22981 * rgb[0] + 1.18340 * rgb[1] + 0.04641 * rgb[2],
        0.0 * rgb[0] +  0.0 * rgb[1] + 1.0 * rgb[2],
    ]
}

pub fn m_hpe_inv(rgb: [f64; 3]) -> [f64; 3] {
    [
        1.910197 * rgb[0]  - 1.112124 * rgb[1] + 0.201908 * rgb[2],
        0.370950 * rgb[0] + 0.629054 * rgb[1] - 0.000008 * rgb[2],
        0.0 * rgb[0] +  0.0 * rgb[1] + 1.0 * rgb[2],
    ]
}

pub fn achromatic_rsp(rgb: [f64; 3], Nbb: f64) -> f64 {
    (2.0 * rgb[0] + rgb[1] + rgb[2] / 20.0 - 0.305) * Nbb
}

#[test]
fn cie_cam_test() {
    use crate::cie1931::CIE1931;
    use crate::observers::ObserverKey;
    use crate::filter::grey_flt;
    use crate::sources::Spectra;
    use crate::physics::assert_vec_eq;
    use assert_approx_eq::assert_approx_eq;
    let xyz = XYZ::from(
        &Spectra::cie_d(5001.8, 1.0 / 1.8101228747229374),
        &grey_flt(380.0, 5.0, 81, vec![0.1841865]),
        &CIE1931,
    );

    // values from CIECAM02 spreadsheet
    let vcd = ViewConditions::default();
    assert_approx_eq!(vcd.D(), 0.940656466);
    assert_approx_eq!(vcd.k(), 0.00199601);
    assert_approx_eq!(vcd.Fl(), 0.79370053);
    let cam = CIECAM::from(xyz, ViewConditions::default());

    // CIECAM02 spreadsheet
    assert_approx_eq!(cam.jch[0][0], 39.74001389);
    assert_approx_eq!(cam.jch[0][1], 0.5660349);
    assert_approx_eq!(cam.jch[0][2], 112.403451);

//	println!("{:?}", cam);


    // http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS
    let tst_xyz = XYZ {
        xyz_vec: vec![
            [28.86602045, 18.41865185, 2.677891918], // Lab 50, 50, 50
            [19.69905189, 18.41865185, 6.068714129], // 50, 10, 30
            [17.75961545, 18.41865185, 2.677891918], // 50, 0, 50
            [28.86602045, 18.41865185, 15.19916323], // 50, 50, 0
        //	[0., 0., 0.], // 0,0,0
            [15.95184479, 18.41865185, 45.32717872], // 50, -10, -50
            [54.65318174, 56.68129075, 103.2566216], // 80, 0, -50
        ],
        obs: ObserverKey::CIE1931,
        xyz_w: Some([96.421908, 100.0, 82.520498]),
    };

    let cam = CIECAM::from(tst_xyz, ViewConditions::default());

//	println!("{:?}", cam.jch);

    let want = vec![
        [41.749269, 68.836414, 38.720187],
        [40.138001, 27.410128, 69.268409],
        [39.662698, 43.578235, 93.629232],
        [41.935959, 55.425303, 1.766343],
    //	[0., 0., 180.],
        [38.787623, 60.277030, 240.725076],
        [73.036617, 54.776410, 254.404297],
    ];
    for i in 0..want.len() {
        assert_vec_eq(&cam.jch[i] ,&want[i], 1.0E-5);
    }
}

 */


 fn hue_composition_from_hue_angle(h:f64) -> f64 {
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