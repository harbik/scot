
use std::{fmt::Display, marker::PhantomData};

use nalgebra::{Matrix3xX};
use crate::{DefaultObserver, observers::StandardObserver};

use super::{CieXYZ, XYZValues};

#[derive(Debug)]
pub struct CieYuv1960<C: StandardObserver = DefaultObserver> {
	pub data : Matrix3xX<f64>,
	_cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieYuv1960<C> {
	pub fn new(data: Matrix3xX<f64>) -> Self {
		Self { data, _cmf: PhantomData}
	}
}

impl<C, X> From<X> for CieYuv1960<C>
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
			v.push(6.0 * y / den);
		}
		Self::new(Matrix3xX::<f64>::from_vec(v))
	}
}

impl<C: StandardObserver> Display for CieYuv1960<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Yuv 1960<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct Yuv1960Iter<C: StandardObserver> {
	yuv: CieYuv1960<C>,
	i: usize,
}

pub struct Yuv1960IterRef<'a, C: StandardObserver> {
	yuv: &'a CieYuv1960<C>,
	i: usize,
}

#[derive(Debug)]
pub struct CieYuv1960Values {
	pub y: f64,
	pub u: f64,
	pub v: f64,
}

impl<C: StandardObserver> Iterator for Yuv1960Iter<C> {
	type Item = CieYuv1960Values;
	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.yuv.data.ncols() {
			let y = self.yuv.data[(0, self.i)];
			let u = self.yuv.data[(1, self.i)];
			let v = self.yuv.data[(2, self.i)];
			self.i += 1;
			Some(CieYuv1960Values {y, u, v})
		} else {
			None
		}
	}
}

impl<'a, C: StandardObserver> Iterator for Yuv1960IterRef<'a, C> {
	type Item = CieYuv1960Values;
	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.yuv.data.ncols() {
			let y = self.yuv.data[(0, self.i)];
			let u = self.yuv.data[(1, self.i)];
			let v = self.yuv.data[(2, self.i)];
			self.i += 1;
			Some(CieYuv1960Values {y, u, v})
		} else {
			None
		}
	}
}

impl<C: StandardObserver> IntoIterator for CieYuv1960<C> {
	type Item = CieYuv1960Values;
	type IntoIter = Yuv1960Iter<C>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			yuv: self,
			i: 0,
		}
	}
}

impl<'a, C: StandardObserver> IntoIterator for &'a CieYuv1960<C> {
	type Item = CieYuv1960Values;
	type IntoIter = Yuv1960IterRef<'a, C>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			yuv: self,
			i: 0,
		}
	}
}

#[test]
fn test_lab_iter(){
	use crate::observers::CieObs1931;
	use crate::illuminants::IesTm30Fluorescent;
	use crate::ALL;
	for CieYuv1960Values {y, u, v}  in CieYuv1960::<CieObs1931>::from(IesTm30Fluorescent::<ALL>){
		println!("{}, {}, {}", y, u, v);
	}
}