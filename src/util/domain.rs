

use crate::util::units::{Unit};
use std::ops::Range;
use std::iter::IntoIterator;
use std::iter::ExactSizeIterator;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Domain<U: Unit> {
	pub range: Range<i32>,
	pub unit: U, 
}

impl<U: Unit> Domain<U> {

	pub fn new(start:i32, end:i32, unit: U) -> Self {
		Self {
			range: Range {start, end: end+1},
			unit
		}
	}

	pub fn len(&self) -> usize {
		self.range.len()
	}

	pub fn iter(&self) -> IterDomain<U> {
		self.into_iter()
	}

	/*
	pub fn value(&self, i: usize) -> f64 {
		self.unit.value(i as isize + self.range.start())
	}
	 */

	/**
		Iterator to new interpolation domain values for conversion to a new domain.

		Produces f64-typed domain values, to use to obtain spectral distribution values for a new domain.
		These are typically obtained by interpolation of an existing spectral distribution dataset,
		such as linear interpolation, or Sprague interpolation.

		The value index float value in the current domain. If this value is negative, or larger than the size of the
		domain, it is out of bounds, and needs to be extrapolated instead of interpolated.
	*/
	pub fn iter_interpolate(&self, to_domain: &Domain<U>) -> IterInterpolate {
		let step = to_domain.unit.value(1) / self.unit.value(1);
		IterInterpolate {
			step,
			curr: to_domain.range.start as f64 * step - self.range.start as f64,
			n: to_domain.len(),
			next: 0,

		}
	}

}

pub struct IterDomain<U: Unit> {
	i: i32,
	end: i32,
	unit: U,
}

impl<U: Unit> Iterator for IterDomain<U> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
		let c = self.i;	
		if self.i<self.end {
			self.i += 1;
			Some(self.unit.value(c))
		} else {
			None
		}
    }
}

/**
	Iterate through all the values of a spectral domain.
*/
impl<U: Unit> IntoIterator for Domain<U> {
    type Item = f64;

    type IntoIter = IterDomain<U>;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			i: self.range.start,
			end: self.range.end,
			unit: self.unit,
		}
    }
}

/**
	Iterate through all the values for a reference to a spectral domain.
*/
impl<U: Unit> IntoIterator for &Domain<U> {
    type Item = f64;

    type IntoIter = IterDomain<U>;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			i: self.range.start,
			end: self.range.end,
			unit: self.unit,
		}
    }
}

#[test]
fn test_into_iterator_spectraldomain() {
	use crate::util::units::{K1000};
	use crate::util::units::Kelvin;
	assert_eq!(Domain::<Kelvin>::new(4, 6, K1000).into_iter().collect::<Vec<_>>(), vec![4000.0, 5000.0, 6000.0]);
}


pub struct IterInterpolate {
	step: f64,
	curr: f64,
	n: usize,
	next: usize,
}

impl Iterator for IterInterpolate {

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
		let din = from_domain.clone().into_iter().collect::<Vec<_>>();
		assert_eq!(din,vec![300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0]);
		 
		let to_domain = Domain::new(5, 21, NONE50); // 0, 1, 2, 3, 4, 5
		let dout = to_domain.clone().into_iter().collect::<Vec<_>>();
		assert_eq!(dout,vec![250.0, 300.0, 350.0, 400.0, 450.0, 500.0, 550.0, 600.0, 650.0, 700.0, 750.0, 800.0, 850.0, 900.0, 950.0, 1000.0, 1050.0]);

		assert_eq!(from_domain.iter_interpolate(&to_domain).collect::<Vec<_>>(), vec![-0.5, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5]) ;
		// values 2,3 and 4 within range
	}

}

