
use std::{fmt::Display, marker::PhantomData};

use nalgebra::{Matrix3xX};
use crate::{DefaultObserver, observers::StandardObserver};

use super::{CieXYZ, XYZValues};

#[derive(Debug)]
pub struct CieYuv<C: StandardObserver = DefaultObserver> {
	pub data : Matrix3xX<f64>,
	cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieYuv<C> {
	pub fn new(data: Matrix3xX<f64>) -> Self {
		Self { data, cmf: PhantomData}
	}
}

impl<C, X> From<X> for CieYuv<C>
where 
	C: StandardObserver,
	X: Into::<CieXYZ<C>>,
{
	fn from(x: X) -> Self {
		let m: CieXYZ<C> = x.into();

		let mut v: Vec<f64> = Vec::with_capacity(m.data.len());
		for XYZValues {x, y, z} in m {
			let den = x + 15.0 * y + 3.0 * z;
			v.push(y);
			v.push(4.0 * x / den);
			v.push(9.0 * y / den);
		}
		Self::new(Matrix3xX::<f64>::from_vec(v))
	}
}

impl<C: StandardObserver> Display for CieYuv<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Yu'v'<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct YuvIter<C: StandardObserver> {
	yuv: CieYuv<C>,
	i: usize,
}

#[derive(Debug)]
pub struct CieYuvValues {
	pub y: f64,
	pub u: f64,
	pub v: f64,
}

impl<C: StandardObserver> Iterator for YuvIter<C> {
	type Item = CieYuvValues;
	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.yuv.data.ncols() {
			let y = self.yuv.data[(0, self.i)];
			let u = self.yuv.data[(1, self.i)];
			let v = self.yuv.data[(2, self.i)];
			self.i += 1;
			Some(CieYuvValues {y, u, v})
		} else {
			None
		}
	}
}

impl<C: StandardObserver> IntoIterator for CieYuv<C> {
	type Item = CieYuvValues;
	type IntoIter = YuvIter<C>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			yuv: self,
			i: 0,
		}
	}
}

