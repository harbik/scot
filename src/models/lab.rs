/*!
	A collection of a Lab values, associated with a standard observer and a reference white illuminant.


*/

use std::{fmt::Display, marker::PhantomData};

use nalgebra::{Matrix3x1, Matrix3xX};
use crate::illuminants::Illuminant;
use crate::observers::StandardObserver;
use crate::spectra::SpectralData;
use crate::swatches::Swatches;
use crate::util::units::{Meter, Scale};

#[derive(Debug)]
pub struct Lab<C: StandardObserver, I: Illuminant> {
	pub data : Matrix3xX<f64>,
	cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
	illuminant: PhantomData<*const I>, // only used through I::Default(), but needed to mark the type
}

impl<C: StandardObserver, I: Illuminant> Lab<C,I> {
	pub fn new(data: Matrix3xX<f64>) -> Self {
		Self { data, cmf: PhantomData, illuminant: PhantomData}
	}
}


const DELTA: f64 = 24f64/116f64;
const DELTA_POW3: f64 = DELTA * DELTA * DELTA;
const LABPOW: f64 = 1f64/3f64;
const LABC1: f64 = 841f64/108f64;
const LABC2: f64 = 16f64/116f64;

fn lab_f(v: f64) -> f64 {
	if v > DELTA_POW3 {
		v.powf(LABPOW)
	} else {
		LABC1 * v + LABC2
	}
}


 /**
	Calculates CIELAB values for color swatches
  */
impl<'a, S, C, I> From<S> for Lab<C, I> 
where 
	S: Swatches,
	C: StandardObserver,
	&'a C: Default,
	I: Illuminant,
	<<S as SpectralData>::ScaleType as Scale>::UnitType: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>,
	Meter: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>
{
    fn from(swatch: S) -> Self {
		let ill = I::default();  // illuminant spectrum
		let ill_dom = ill.domain();
		let ill_data = ill.values(&ill_dom);
		let sw_data = swatch.values(&ill_dom);

		let (xyz_n, xyz) = C::xyz_from_dom_ill_mat(ill_dom, ill_data, sw_data);

		Lab{ data: cielab(xyz_n, xyz), cmf: PhantomData, illuminant: PhantomData }
    }
}

impl<C: StandardObserver, I: Illuminant> Display for Lab<C, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lab<{}>: {:.5}", C::NAME, self.data)
    }
}

fn cielab(xyz_n: Matrix3x1<f64>, xyz: Matrix3xX<f64>) -> Matrix3xX<f64> {
	let mut m: Matrix3xX<f64> = Matrix3xX::from_fn(xyz.ncols(),|i,j| xyz[(i,j)]/xyz_n[(i,0)]);
	for mut xyz in m.column_iter_mut() {
			let x = xyz.x;
			let y = xyz.y;
			let z = xyz.z;
			xyz.x = 116f64 * lab_f(y) - 16f64;
			xyz.y = 500f64 * (lab_f(x) - lab_f(y));
			xyz.z = 200f64 * (lab_f(y) - lab_f(z));
	}
	m
}

#[test]
/**
	ColorChecker CieLab values calculation.

	The test values are from the Babel color spreadsheet, with spectral values defined at a domain from 380 to 730nm,
	with 10nm steps.  The values here use the illuminant D50 domain, which uses 5nm steps. This results in small deviations
	in the order of 0.1% in CieLab values.
*/ 
fn test_cielab_colorchecker(){

		use crate::models::Lab;
		use crate::observers::Cie1931;
		use crate::illuminants::D50;
		use crate::swatches::checker::ColorChecker;
		use crate::swatches::{White, Gray};
		use approx::{assert_abs_diff_eq};
		use nalgebra::{matrix};

		let white:  Lab<Cie1931, D50> = White::default().into();
		assert_abs_diff_eq!(white.data[(0,0)], 100.0, epsilon = 0.00001);
		assert_abs_diff_eq!(white.data[(1,0)], 0.0, epsilon = 0.00001);
		assert_abs_diff_eq!(white.data[(2,0)], 0.0, epsilon = 0.00001);
	//	println!("White {:.4}", white);

		let gray:  Lab<Cie1931, D50> = Gray(0.18418651851244416).into();
		assert_abs_diff_eq!(gray.data[(0,0)], 50.0, epsilon = 0.00001);
		assert_abs_diff_eq!(gray.data[(1,0)], 0.0, epsilon = 0.00001);
		assert_abs_diff_eq!(gray.data[(2,0)], 0.0, epsilon = 0.00001);
	//	println!("Gray {:.4}", gray);

		let checker_lab: Lab<Cie1931,D50> = ColorChecker::default().into();

		let babel = matrix![
			38.44, 13.61, 14.53;
			65.95, 17.91, 17.87;
			50.06, -4.52, -22.25;
			43.28, -13.21, 21.94;
			55.31, 8.82, -24.60;
			70.69, -33.03, -0.11;
			62.65, 35.35, 57.86;
			40.24, 9.74, -44.35;
			51.60, 47.80, 16.90;
			30.50, 21.07, -20.02;
			72.46, -23.30, 57.00;
			71.95, 19.46, 68.12;
			28.87, 14.81, -50.15;
			55.15, -37.80, 31.64;
			42.28, 54.12, 28.67;
			82.27, 4.02, 79.99;
			51.91, 49.80, -13.82;
			50.72, -28.11, -27.95;
			96.53, -0.47, 2.42;
			81.21, -0.64, 0.27;
			66.48, -0.53, 0.00;
			50.83, -0.64, -0.14;
			35.85, -0.54, -0.49;
			20.81, 0.03, -0.39
		];

		for (i,cc) in checker_lab.data.column_iter().enumerate() {
			assert_abs_diff_eq!(cc.x, babel[(i,0)], epsilon = 0.05);
			assert_abs_diff_eq!(cc.y, babel[(i,1)], epsilon = 0.05);
			assert_abs_diff_eq!(cc.z, babel[(i,2)], epsilon = 0.05);

		}
}
