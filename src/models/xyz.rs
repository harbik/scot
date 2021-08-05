
use std::{fmt::Display, marker::PhantomData};

use nalgebra::{Matrix3xX};
use crate::{observers::StandardObserver};

/**	
	A collection of a tristimulus values, associated with a standard observer,
	and an optional set of tristimulus values of a reference white point.

	The reference to a standard observers color matching functions is not only used to uniquely identify the observer
	associated with the tristimulus values, but also for the conversion of chromaticity coordinates between different
	observers, using for example transforming back to a set of reference RGB spectra, and calculating the tristimulus
	values for a different observer. The standard observers have global (static) scope.

*/
#[derive(Debug)]
pub struct XYZ<C: StandardObserver> {
	pub data : Matrix3xX<f64>,
	cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> XYZ<C> {
	pub fn new(xyz: Matrix3xX<f64>) -> Self {
		Self { data: xyz, cmf: PhantomData}
	}
}

impl<C: StandardObserver> Display for XYZ<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "XYZ<{}>: {:.5}", C::NAME, self.data)
    }
}

#[derive(Debug)]
pub struct Yxy<C: StandardObserver> {
	pub data : Matrix3xX<f64>,
	cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> Yxy<C> {
	pub fn new(yxy: Matrix3xX<f64>) -> Self {
		Self { data: yxy, cmf: PhantomData}
	}

	pub fn yxy(&self, i: usize) -> [f64;3] {
		let v = self.data.column(i);
		[v.x, v.y, v.z]
	}
}

impl<C, X> From<X> for Yxy<C>
where 
	C: StandardObserver,
	X: Into::<XYZ<C>>,
{
	fn from(x: X) -> Self {
		let m: XYZ<C> = x.into();

		let mut v: Vec<f64> = Vec::with_capacity(m.data.len());
		for xyz in m.data.column_iter(){
			let s = xyz.sum();
			v.push(xyz.y);
			v.push(xyz.x/s);
			v.push(xyz.y/s);
		}
		Self::new(Matrix3xX::<f64>::from_vec(v))
	}
}

impl<C: StandardObserver> Display for Yxy<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Yxy<{}>: {:.5}", C::NAME, self.data)
    }
}
