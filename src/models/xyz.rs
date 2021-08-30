
use std::{fmt::Display, marker::PhantomData};

use nalgebra::{Matrix3xX};
use crate::{DefaultObserver, Meter, SpectralData, Step, Unit, observers::StandardObserver};

/**	
	A collection of a tristimulus values, associated with a standard observer,
	and an optional set of tristimulus values of a reference white point.

	The reference to a standard observers color matching functions is not only used to uniquely identify the observer
	associated with the tristimulus values, but also for the conversion of chromaticity coordinates between different
	observers, using for example transforming back to a set of reference RGB spectra, and calculating the tristimulus
	values for a different observer. The standard observers have global (static) scope.

*/
#[derive(Debug)]
pub struct CieXYZ<C: StandardObserver = DefaultObserver> {
	pub data : Matrix3xX<f64>,
	cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieXYZ<C> {
	pub fn new(xyz: Matrix3xX<f64>) -> Self {
		Self { data: xyz, cmf: PhantomData}
	}
}

/**
	Calculate XYZ tristimilus value from spectral distributions.

	This is a generic implementation for calculation of XYZ values. 
	It interpolates the color matching functions values onto the 
	spectral distribution's domain.

	# Examples
	Calculate Tristimulus values for a Blackbody radiator
	```
	use colorado::illuminants::Blackbody;
	use colorado::observers::Cie1931;
	use colorado::cie::XYZ;

	let bb = XYZ::<Cie1931>::from(Blackbody::new(3000));
	println!("{}",bb);
	```
 */

impl<C, S> From<S> for CieXYZ<C>
where 
	C: StandardObserver,
//	&'static C: Default,
	S: SpectralData,
	Meter: From<<<S as SpectralData>::StepType as Step>::UnitValueType>,
//	<Matrix<f64, Const, Dynamic, VecStorage<f64, Const, Dynamic>> as Mul<<S as SpectralData>::MatrixType>>::Output
 {
	fn from(sd: S) -> Self {
		let xyz = 
			<C>::default().values(&sd.domain()) * sd.values(&sd.domain()) * C::K * sd.domain().step.unitvalue(1).value();
		//let xyz = C::xyz_from_dom_mat(sd.domain(), sd.values(&sd.domain()));
		CieXYZ::<C>::new(xyz)
	}
}

impl<C: StandardObserver> Display for CieXYZ<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "XYZ<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct XYZIter<C: StandardObserver> {
	xyz: CieXYZ<C>,
	i: usize,
}

impl<C: StandardObserver> Iterator for XYZIter<C> {
	type Item = XYZValues;
	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.xyz.data.ncols() {
			let x = self.xyz.data[(0, self.i)];
			let y = self.xyz.data[(1, self.i)];
			let z = self.xyz.data[(2, self.i)];
			self.i += 1;
			Some(XYZValues {x, y, z})
		} else {
			None
		}
	}
}

#[derive(Debug)]
pub struct XYZValues {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl<C: StandardObserver> IntoIterator for CieXYZ<C> {
	type Item = XYZValues;

	type IntoIter = XYZIter<C>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			xyz: self,
			i: 0,
		}
	}
}

#[test]
fn test_lab_iter(){
	use crate::observers::CieObs1931;
	use crate::illuminants::IesTm30Fluorescent;
	use crate::ALL;
	for XYZValues {x, y, z}  in CieXYZ::<CieObs1931>::from(IesTm30Fluorescent::<ALL>){
		println!("{}, {}, {}", x, y, z);
	}
}


#[derive(Debug)]
pub struct CieYxy<C: StandardObserver = DefaultObserver> {
	pub data : Matrix3xX<f64>,
	cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieYxy<C> {
	pub fn new(yxy: Matrix3xX<f64>) -> Self {
		Self { data: yxy, cmf: PhantomData}
	}

	pub fn yxy(&self, i: usize) -> [f64;3] {
		let v = self.data.column(i);
		[v.x, v.y, v.z]
	}
}

impl<C, X> From<X> for CieYxy<C>
where 
	C: StandardObserver,
	X: Into::<CieXYZ<C>>,
{
	fn from(x: X) -> Self {
		let m: CieXYZ<C> = x.into();

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

impl<C: StandardObserver> Display for CieYxy<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Yxy<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct YxyIter<C: StandardObserver> {
	lxy: CieYxy<C>,
	i: usize,
}

impl<C: StandardObserver> Iterator for YxyIter<C> {
	type Item = YxyValues;
	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.lxy.data.ncols() {
			let l = self.lxy.data[(0, self.i)];
			let x = self.lxy.data[(1, self.i)];
			let y = self.lxy.data[(2, self.i)];
			self.i += 1;
			Some(YxyValues {l, x, y})
		} else {
			None
		}
	}
}

#[derive(Debug)]
pub struct YxyValues {
	pub l: f64,
	pub x: f64,
	pub y: f64,
}

impl<C: StandardObserver> IntoIterator for CieYxy<C> {
	type Item = YxyValues;

	type IntoIter = YxyIter<C>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			lxy: self,
			i: 0,
		}
	}
}

#[test]
fn test_yxy_iter(){
	use crate::observers::CieObs1931;
	use crate::illuminants::IesTm30Fluorescent;
	use crate::ALL;
	for YxyValues {l, x	, y}  in CieYxy::<CieObs1931>::from(IesTm30Fluorescent::<ALL>){
		println!("{}, {}, {}", l, x, y);
	}
}
