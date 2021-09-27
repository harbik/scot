/*!
	`CieCam` color appearance model, calcualting correlates Lightness (J), Brightness (Q),
	Redness-Greenness (a), Yellow-Blueness (b), Chroma (C), Colorfulness (M), Saturation (s),
	Hue-angle (h), and Hue-composition (H).
 */

use std::{marker::PhantomData, };
use nalgebra::{Const, Dynamic, OMatrix, };
use crate::{DefaultObserver, illuminants::{D65, }, observers::{StandardObserver}};
use super::{VcAvg, CieCamEnv, };
use super::{CieLab, CieXYZ};

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
			let hue_composition = cam.hue_composition(hue_angle); // H
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
	use nalgebra::Matrix3xX;
	use super::{SR_AVG, D_AUTO, ViewConditions};
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
	use nalgebra::{Matrix3xX};
	use super::{ViewConditions, SR_AVG, D_AUTO};
	let lab: CieLab<D50> = CieLab::new(Matrix3xX::from_vec(vec![
		50.0, 0.0, 0.0,
	]));
	let cam: CieCam<ViewConditions<32, 20, SR_AVG, D_AUTO>, D50, CieObs1931> = lab.into();
//	let want = matrix![
//
	//];

	println!("{}", cam.data);
}
