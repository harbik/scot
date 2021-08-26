/*!
	Equidistant value ranges, used in this library for example to define wavelength ranges for spectral distributions.
 */

use crate::{Step, Unit};
use std::fmt::Debug;
use std::ops::Range;
use std::iter::IntoIterator;
use std::iter::ExactSizeIterator;
use std::marker::PhantomData;

use super::WavelengthStep;

/**

	Defines a range, with a start and end parameter, and physical units for the arguments of a varying parameter.

	In mathematics it is used to describe the independent parameter of a function along the x-axis,
	as apposed to the range of a function, describing the dependent parameters along the y-axis.
	Its main use in this library is to describe the domain of a set of spectral distribution values,
	consisting of a set of wavelength values. It is represented by a `Range`, with a start and end value,
	and a unit. The range values have an `i32` type.

	A new domain is defined by supplying a start and end value, and a scale in which the start and end values are expressed.
	The range for the domain includes the start and end value, so it is equivalent to the rust range (start..=end). 
	Examples of scales in the library are `NM5`, for a scale with 5.0 nanometer ticks, or `A`, for a 1.0 Angstrom ticks, or PCT,
	with a tick values 1.0%.
	Start and end values are signed integer values.
	

	# Examples
	Define a wavelength range from 4530 to 4550 Angstrom with 1 Angstrom steps, resulting in 21 wavelength values.
	```
	use colorado::util::{A, Unit, Domain};
	use approx::assert_abs_diff_eq;

	let d = Domain::new(4530, 4550, A);
	let v = d.iter().map(|v|v.value()).collect::<Vec<f64>>();
	let vlen = v.len();
	assert_abs_diff_eq!(v[0], 4530E-10, epsilon = 1E-16);
	assert_abs_diff_eq!(v[1] - v[0], 1E-10, epsilon = 1E-16); // 1 Angstrom distance between the points
	assert_eq!(vlen, 21);
	assert_abs_diff_eq!(v[v.len()-1], 4550E-10, epsilon = 1E-16);
	```
	Here is an example of a commonly used wavelength range, from 360 to 830 nm, with 5nm steps, the range of values as
	in the original table of CIE 1931 color matching functions. In this case there are 94 values. The unit now used is
	'NM5', and the range values are divided by 5 to get the right paramter index values.
	```
	use colorado::util::{Domain, Unit, NM5};
	use approx::assert_abs_diff_eq;

	let d = Domain::new(360/5, 830/5, NM5);
	let v = d.into_iter().map(|v|v.value()).collect::<Vec<f64>>();
	let vlen = v.len();
	assert_abs_diff_eq!(v[0], 360.0E-9);
	assert_eq!(vlen, 95);
	assert_abs_diff_eq!(v[v.len()-1], 830.0E-9);
	```
	It can also be used for other types of input parameters, such as temperature.
	Here we define a range of 5 temperature values, with unit KK, or 'kilo Kelvin',
	with values ranging from 3000.0 to 7000.0K
	```
	use colorado::util::{Domain, Unit, KK};
	use approx::assert_abs_diff_eq;

	let d = Domain::new(3, 7, KK);
	let v = d.into_iter().map(|v|v.value()).collect::<Vec<f64>>();
	let vlen = v.len();
	assert_abs_diff_eq!(v[0], 3000.0);
	assert_abs_diff_eq!(v[1], 4000.0);
	assert_abs_diff_eq!(v[2], 5000.0);
	assert_abs_diff_eq!(v[3], 6000.0);
	assert_abs_diff_eq!(v[4], 7000.0);
	assert_eq!(vlen, 5);
	```
	
	
 */
#[derive(Clone, Debug, Eq)]
pub struct Domain<S:Step> {
	pub range: Range<i32>,
	pub step: S, 
}

impl<S> Domain<S> 
where
	S: Step,
	S::UnitValueType: Unit
{

	/**
		Creates a new domain ranging from start to end, including end, and with a scale.
	 */
	pub fn new(start:i32, end:i32, scale: S) -> Self {
		Self {
			range: Range {start, end: end+1},  // this it the non-inclusive end range, so we're adding a 1. The InclusiveRange range has no support for i32!
			step: scale
		}
	}

	pub fn len(&self) -> usize {
		self.range.len()
	}

	pub fn iter(&self) -> IterDomain<S> {
		self.into_iter()
	}

	pub fn iter_interpolate<S2: Step> (&self, to_domain: &Domain<S2>) -> IterInterpolate<S, S2>
	where
		S::UnitValueType: From<<S2>::UnitValueType>, 
	{
		let div = self.step.unitvalue(1).value();
		let s = Into::<S::UnitValueType>::into(to_domain.step.unitvalue(1)).value()/div;
		let stride = if s.fract()<1E-10 && s>0.5 {
			s as usize
		} else {
			0usize
		};
		//println!("Stride: {}", stride);
		IterInterpolate {
			start: self.step.unitvalue(self.range.start).value(),
			div,
			stride,
			ito: 0usize,
			iter_to: to_domain.into_iter(),
			n: self.len()-1, // max index value
			_phd: PhantomData
		}
	}
}

pub struct IterInterpolate<S:Step, S2: Step> {
//	pub fr: Domain<S1>,
//	pub to: Domain<S2>,
	pub start: f64,
	pub div: f64,
	pub stride: usize,
	pub n: usize,
	pub ito: usize,
	pub iter_to: IterDomain<S2>,
	_phd: PhantomData::<*const S>
}

#[derive(Debug)]
pub enum IterInterpolateType { // index target domain, left index interval from domain, and fraction
	ExtrapolateLow(usize, isize, f64),
	ExtrapolateHigh(usize, usize, f64),
	Interpolate(usize, usize, f64),
	RangeEnd(usize, usize),
}

impl<S: Step, S2: Step> Iterator for IterInterpolate<S,S2> 
where
    S::UnitValueType: From<<S2>::UnitValueType>, 
{
    type Item = IterInterpolateType;

    fn next(&mut self) -> Option<Self::Item> {
		match self.iter_to.next(){
			Some(ut) => {
				let j = self.ito;
				self.ito += 1;
				let from_domain_interval = (Into::<S::UnitValueType>::into(ut).value() - self.start) / self.div;
				let i = from_domain_interval.floor() as isize;
				match (i, i as usize, from_domain_interval - i as f64) {
					(i, _, h) if i<0 => Some(IterInterpolateType::ExtrapolateLow(j, i, h)),
					(_, u, h) if u <self.n =>  Some(IterInterpolateType::Interpolate(j, u, h)), // i>=0
					(_, u, h) if u==self.n && h<1E-6 => Some(IterInterpolateType::RangeEnd(j, u)),
					(_, u, h) => Some(IterInterpolateType::ExtrapolateHigh(j, u-self.n, h)),
				}
			}
			None => None,
		}
    }
}

/**
	380 to 780 nm wavelength scale, with 1 nm steps
 */
impl Default for Domain<WavelengthStep> {
    fn default() -> Self {
        Domain::new(380, 780, WavelengthStep { size: 1, exp: -9 })
    }
}

impl<S1:Step,  S2:Step> PartialEq<Domain<S2>> for Domain<S1> {
    fn eq(&self, other: &Domain<S2>) -> bool {
		/*
        if self.range.start == other.range.start &&
			self.range.end == other.range.end &&
			<S1 as Step>::UnitValueType::NAME == <S2 as Step>::UnitValueType::NAME &&
			self.scale.unitvalue(1).value() == other.scale.unitvalue(1).value()
		{
				true
			} else {
				false
			}
		 */
		self.range.start == other.range.start &&
		self.range.end == other.range.end &&
		<S1 as Step>::UnitValueType::NAME == <S2 as Step>::UnitValueType::NAME &&
		self.step.unitvalue(1).value() == other.step.unitvalue(1).value()}
}

/**
	Domain iterator, generating all domain values as `f64` values.

	The generated values are float values, expressed in base unit values.

	# Examples
	An iterator starting at a value 4530 Angstrom, as a value 4530.0E-10m, and making steps of of 1 Angstrom, or
	1.0E-10m.
	```	
	use colorado::util::Domain;
	use colorado::util::{A, Unit};
	use approx::assert_abs_diff_eq;

	let mut it = Domain::new(4530, 4550, A).into_iter();
	assert_abs_diff_eq!(it.next().unwrap().value(), 4530E-10);
	assert_abs_diff_eq!(it.next().unwrap().value(), 4531E-10);
	```
	And here is an example for use in a for loop. 
	This produces two values, 3000 and 4000K, and thier sum is supposed to be 7000.0 K.
	```	
	use colorado::util::Domain;
	use colorado::util::{KK, Unit};
	use approx::assert_abs_diff_eq;

	let mut sum = 0.0;
	for t in Domain::new(3, 4, KK){
		sum += t.value();
	}
	assert_abs_diff_eq!(sum, 7000.0);
	```
 */
pub struct IterDomain<S> {
	i: i32,
	end: i32,
	scale: S,
}


impl<S> Iterator for IterDomain<S> 
where 
	S: Step,
	S::UnitValueType: Unit
{
    type Item = S::UnitValueType;

    fn next(&mut self) -> Option<Self::Item> {
		let c = self.i;	
		if self.i<self.end {
			self.i += 1;
			Some(self.scale.unitvalue(c))
		} else {
			None
		}
    }
}


/**
	Iterate through all the values of a spectral domain, as `Unit` values.
*/
impl<S> IntoIterator for Domain<S>
where 
	S: Step,
	S::UnitValueType: Unit
 {
    type Item = S::UnitValueType;

    type IntoIter = IterDomain<S>;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			i: self.range.start,
			end: self.range.end,
			scale: self.step,
		}
    }
}


/**
	Iterate through all the values for a reference to a spectral domain.
*/
impl<S: Step> IntoIterator for &Domain<S> {
    type Item = S::UnitValueType;

    type IntoIter = IterDomain<S>;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			i: self.range.start,
			end: self.range.end,
			scale: self.step,
		}
    }
}

#[test]
fn test_into_iterator_spectraldomain() {
	use crate::util::KK;
	assert_eq!(Domain::new(4, 6, KK).into_iter().map(|u|u.value()).collect::<Vec<_>>(), vec![4000.0, 5000.0, 6000.0]);
}


#[test]
fn test_iter_interpolate() {
	{
		use crate::util::{NONE5, NONE};

		let from_domain = Domain::new(1, 2, NONE5); // 2, 4
		let din = from_domain.iter().map(|u|u.value()).collect::<Vec<_>>();
		assert_eq!(din,vec![5.0, 10.0]);
		 
		let to_domain = Domain::new(4, 11, NONE); // 0, 1, 2, 3, 4, 5
		let dout = to_domain.clone().into_iter().map(|u|u.value()).collect::<Vec<_>>();
		assert_eq!(dout,vec![4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,]);

		let val = from_domain.iter_interpolate(&to_domain).into_iter().collect::<Vec<_>>();
		println!("{:?}", val)
	}

}

