/*!
	Equidistant value ranges, used in this library for example to define wavelength ranges for spectral distributions.
 */

use crate::util::units::{Scale, Unit};
use std::ops::Range;
use std::iter::IntoIterator;
use std::iter::ExactSizeIterator;

/**

	Defines a range, with a start and end parameter, and physical units for the arguments of a varying parameter.

	In mathematics it is used to describe the independent parameter of a function along the x-axis,
	as apposed to the range of a function, describing the dependent parameters along the y-axis.
	Its main use in this library is to describe the domain of a set of spectral distribution values,
	consisting of a set of wavelength values. It is represented by a `Range`, with a start and end value,
	and a unit. The range values have an `i32` type.

	A new domain is defined by supplying a start and end value, and a unit in which the start and end values are expressed.
	The range for the domain includes the start and end value, so it is equivalent to the rust range (start..=end). 
	Examples of units in the library are NM5, for a unit of 5.0 nanometer, A, for a unit of 1.0 Angstrom, or PCT,
	with a unit value of 1.0%.
	Start and end values are signed integer values.
	

	# Examples
	Define a wavelength range from 4530 to 4550 Angstrom with 1 Angstrom steps, resulting in 21 wavelength values.
	```
	use colorado::util::domain::Domain;
	use colorado::util::units::A;
	use approx::assert_abs_diff_eq;

	let d = Domain::new(4530, 4550, A);
	let v = d.iter().collect::<Vec<f64>>();
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
	use colorado::util::domain::Domain;
	use colorado::util::units::NM5;
	use approx::assert_abs_diff_eq;

	let d = Domain::new(360/5, 830/5, NM5);
	let v = d.into_iter().collect::<Vec<f64>>();
	let vlen = v.len();
	assert_abs_diff_eq!(v[0], 360.0E-9);
	assert_eq!(vlen, 95);
	assert_abs_diff_eq!(v[v.len()-1], 830.0E-9);
	```
	It can also be used for other types of input parameters, such as temperature.
	Here we define a range of 5 temperature values, with unit KK, or 'kilo Kelvin',
	with values ranging from 3000.0 to 7000.0K
	```
	use colorado::util::domain::Domain;
	use colorado::util::units::KK;
	use approx::assert_abs_diff_eq;

	let d = Domain::new(3, 7, KK);
	let v = d.into_iter().collect::<Vec<f64>>();
	let vlen = v.len();
	assert_abs_diff_eq!(v[0], 3000.0);
	assert_abs_diff_eq!(v[1], 4000.0);
	assert_abs_diff_eq!(v[2], 5000.0);
	assert_abs_diff_eq!(v[3], 6000.0);
	assert_abs_diff_eq!(v[4], 7000.0);
	assert_eq!(vlen, 5);
	```
	
	
 */
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Domain<S: Scale> {
	pub range: Range<i32>,
	pub scale: S, 
}

impl<S: Scale> Domain<S> {

	pub fn new(start:i32, end:i32, scale: S) -> Self {
		Self {
			range: Range {start, end: end+1},
			scale
		}
	}

	pub fn len(&self) -> usize {
		self.range.len()
	}

	pub fn iter(&self) -> IterDomain<S> {
		self.into_iter()
	}


	/**
		Iterator to produce domain values for a new domain mapped to a current domain.

		These are typically obtained by interpolation of an existing spectral distribution dataset,
		such as linear interpolation, or Sprague interpolation.
		The produced value is an index float value in the current domain. 
		If this value is negative, or larger than the size of the domain, it is out of bounds, and needs to be extrapolated instead of interpolated.

	*/
	pub fn interpolate(&self, to_domain: &Domain<S>) -> InterpolationIterator {
		let step = to_domain.scale.unit(1).value() / self.scale.unit(1).value();
		InterpolationIterator {
			step,
			curr: to_domain.range.start as f64 * step - self.range.start as f64,
			n: to_domain.len(),
			next: 0,

		}
	}

}

/**
	Domain iterator, generating all domain values as `f64` values.

	The generated values are float values, expressed in base unit values.

	# Examples
	An iterator starting at a value 4530 Angstrom, as a value 4530.0E-10m, and making steps of of 1 Angstrom, or
	1.0E-10m.
	```	
	use colorado::util::domain::Domain;
	use colorado::util::units::A;
	use approx::assert_abs_diff_eq;

	let mut it = Domain::new(4530, 4550, A).into_iter();
	assert_abs_diff_eq!(it.next().unwrap(), 4530E-10);
	assert_abs_diff_eq!(it.next().unwrap(), 4531E-10);
	```
	And here is an example for use in a for loop. 
	This produces two values, 3000 and 4000K, and thier sum is supposed to be 7000.0 K.
	```	
	use colorado::util::domain::Domain;
	use colorado::util::units::KK;
	use approx::assert_abs_diff_eq;

	let mut sum = 0.0;
	for t in Domain::new(3, 4, KK){
		sum += t;
	}
	assert_abs_diff_eq!(sum, 7000.0);
	```
 */
pub struct IterDomain<U: Scale> {
	i: i32,
	end: i32,
	unit: U,
}

impl<U: Scale> Iterator for IterDomain<U> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
		let c = self.i;	
		if self.i<self.end {
			self.i += 1;
			Some(self.unit.unit(c).value())
		} else {
			None
		}
    }
}


/**
	Iterate through all the values of a spectral domain, as `f64` values.
*/
impl<U: Scale> IntoIterator for Domain<U> {
    type Item = f64;

    type IntoIter = IterDomain<U>;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			i: self.range.start,
			end: self.range.end,
			unit: self.scale,
		}
    }
}


/**
	Iterate through all the values for a reference to a spectral domain.
*/
impl<U: Scale> IntoIterator for &Domain<U> {
    type Item = f64;

    type IntoIter = IterDomain<U>;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			i: self.range.start,
			end: self.range.end,
			unit: self.scale,
		}
    }
}

#[test]
fn test_into_iterator_spectraldomain() {
	use crate::util::units::KK;
	assert_eq!(Domain::new(4, 6, KK).into_iter().collect::<Vec<_>>(), vec![4000.0, 5000.0, 6000.0]);
}


/**
	Interpolation iterator. 
	Maps a new domain onto an existing domain.

	This iterator produces the locations of a new value domain, as floating values mapped on the old domain.




 */
pub struct InterpolationIterator {
	step: f64,
	curr: f64,
	n: usize,
	next: usize,
}

impl Iterator for InterpolationIterator {

	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.next < self.n {
			let c = self.curr;
			self.next += 1;
			self.curr += self.step;
			Some(c)
		} else {
			None
		}

	}
}


#[test]
fn test_iter_interpolate() {
	{
		use crate::util::units::{NONE100, NONE50};

		let from_domain = Domain::new(3, 10, NONE100); // 2, 4
		let din = from_domain.iter().collect::<Vec<_>>();
		assert_eq!(din,vec![300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0]);
		 
		let to_domain = Domain::new(5, 21, NONE50); // 0, 1, 2, 3, 4, 5
		let dout = to_domain.clone().into_iter().collect::<Vec<_>>();
		assert_eq!(dout,vec![250.0, 300.0, 350.0, 400.0, 450.0, 500.0, 550.0, 600.0, 650.0, 700.0, 750.0, 800.0, 850.0, 900.0, 950.0, 1000.0, 1050.0]);

		assert_eq!(from_domain.interpolate(&to_domain).collect::<Vec<_>>(), vec![-0.5, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5]) ;
		// values 2,3 and 4 within range
	}

}

