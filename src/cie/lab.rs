/*!
	A collection of a Lab values, associated with a standard observer and a reference white illuminant.


*/

use std::{fmt::Display, marker::PhantomData};

use nalgebra::{DMatrix, Matrix3xX};
use crate::{cie::{XYZ, Yxy}, illuminants::Illuminant, observers::StandardObserver, spectra::SpectralData, swatches::{Swatches}, util::units::{Meter, Scale, Unit}};

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

/*
Too generic - use seperate Froms 

impl<C, X, I> From<X> for Lab<C, I>
where 
	C: StandardObserver,
	X: Into::<XYZ<C>>,
	I: Illuminant,
	I: Into::<XYZ<C>>
{
	fn from(x: X) -> Self {
		let r = I::default();
		let w: XYZ<C> = r.into();
		let white = w.data.column(0);

		let m: XYZ<C> = x.into();
		let xn = white.x;
		let yn = white.y;
		let zn = white.z;

		let mut v: Vec<f64> = Vec::with_capacity(m.data.ncols() * 3);
				

		for xyz in m.data.column_iter() {
			v.push(116f64 * lab_f(xyz.y/yn) - 16f64);
			v.push(500f64 * (lab_f(xyz.x/xn) - lab_f(xyz.y/yn)));
			v.push(200f64 * (lab_f(xyz.y/yn) - lab_f(xyz.z/zn)));

		}

		Self::new(Matrix3xX::<f64>::from_vec(v))
	}
}
 */

 /**
	Calculates CIELAB values for color swatches
  */
impl<S, C, I> From<S> for Lab<C, I> 
where 
	S: Swatches,
	C: StandardObserver,
	&'static C: Default,
	I: Illuminant,
	<<S as SpectralData>::ScaleType as Scale>::UnitType: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>,
	Meter: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>
{
    fn from(swatch: S) -> Self {
		let ill = I::default();  // illuminant spectrum
		let ill_dom = ill.domain();
		let ill_data = ill.values(&ill_dom);

		let sw_data = swatch.values(&ill_dom);
		let m: DMatrix<f64>  = DMatrix::from_fn(ill_data.nrows(), sw_data.ncols(), |i, j| ill_data[(i,0)] * sw_data[(i,j)]);

		let mut xyz_n = C::into_xyz(&ill_dom, ill_data);
		xyz_n  *= 100.0 / xyz_n[(1,0)];
		println!("xyz_n {:.4}", xyz_n);
		let xyz = C::into_xyz(&ill_dom, m) * 100.0 / xyz_n[(1,0)];
		println!("xyz {:.4}", xyz.transpose());


//		let xyzn = <&C>::default().cmf(&ill_dom) * ill_data * C::K * ill_dom.scale.unit(1).value();
		Lab{ data: cielab(xyz, xyz_n), cmf: PhantomData, illuminant: PhantomData }
    }
}

impl<C: StandardObserver, I: Illuminant> Display for Lab<C, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lab<{}>: {:.5}", C::NAME, self.data)
    }
}

fn cielab(xyz: Matrix3xX<f64>, xyz_n: Matrix3xX<f64>) -> Matrix3xX<f64> {
	let s = xyz.ncols() as f64;
	let mut m: Matrix3xX<f64> = Matrix3xX::from_fn(xyz.ncols(),|i,j| xyz[(i,j)]/xyz_n[(i,0)]);
	println!("norm {}", m); // normalized matrix
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
fn test_cielab(){

		use crate::cie::Lab;
		use crate::observers::Cie1931;
		use crate::illuminants::D50;
		use crate::swatches::checker::ColorChecker;
		use crate::swatches::{White, SW_BLACK, SW_GRAY50};

		let white:  Lab<Cie1931, D50> = White::default().into();
		println!("White {:.4}", white);

		let gray:  Lab<Cie1931, D50> = SW_GRAY50.into();
		println!("Gray {:.4}", gray);

		let black:  Lab<Cie1931, D50> = SW_BLACK.into();
		println!("Black {:.4}", black);

		let checker_lab: Lab<Cie1931,D50> = ColorChecker::default().into();
		println!("{:.4}", checker_lab.data.transpose());
}
