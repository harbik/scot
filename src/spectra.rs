
extern crate nalgebra as na;


use std::fmt;
use std::fmt::{Display, Formatter};

use na::DMatrix;

use crate::observers::StandardObserver;
use crate::xyz::XYZ;

pub trait Illuminant {}
/// trait marker for illuminant spectra

pub trait Swatch {}
/// trait marker for swatch reflection spectra, 
/// such as the Munsell color swatches.

pub trait Pixel {}
/// trait marker for display pixel spectra

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SpectralDomain {
	pub low: usize, // minimum 'unit' wavelength
	pub unit: usize,  // Angstrom
	pub size: usize, // number of data points in the spectrum
}


impl SpectralDomain {

    pub fn new(low: usize, high: usize, unit: usize) -> Self { 
		let size = (high - low) + 1;
		Self { low, unit, size } 
	}

	pub fn high(&self) -> usize {
		self.low + self.size - 1
	}

	/**
		Iterator to new interpolation domain values for conversion to a new domain.

		Produces domain values, as (usize, f64) types, for spectral distribution values for a new domain.
		These are typically obtained by interpolation of an existing spectral distribution dataset,
		such as linear interpolation, or Sprague interpolation.

		The first value is the index value for the new domain, the second an index float value in the current domain. If
		this value is negative, or larger than the size of the domain, it is out of bounds, and needs to be extrapolated
		instead of interpolated.
	*/
	pub fn iter_interpolate(&self, to_domain: SpectralDomain) -> IterInterpolate {
		let step = to_domain.unit as f64 / self.unit as f64;
		IterInterpolate {
			step,
			curr: to_domain.low as f64 * step - self.low as f64,
			n: to_domain.size,
			next: 0,

		}
	}

}

pub struct IterSpectralDomain {
	curr: usize,
	stop: usize,
	step: usize
}

impl Iterator for IterSpectralDomain {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
		let c = self.curr;
		if c <= self.stop {
			self.curr += self.step;
			Some(c)

		} else {
			None
		}
    }
}

/**
	Iterate through all the values of a spectral domain, in units of Angstrom,
	with type usize.

	These can for example be used to calculate the spectral distribution for a analytical spectral
	distribution, such as Planck's law.
*/
impl IntoIterator for SpectralDomain {
    type Item = usize;

    type IntoIter = IterSpectralDomain;

    fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			curr: self.low * self.unit,
			stop: (self.low + self.size - 1) * self.unit,
			step: self.unit,
		}
    }
}

#[test]
fn test_into_iterator_spectraldomain() {
	assert_eq!(SpectralDomain::new(4, 6, 1000).into_iter().collect::<Vec<_>>(), vec![4000, 5000, 6000]);
//	println!("{:?}", SpectralDomain::new(4, 6, 1000).into_iter().collect::<Vec<_>>());
}

impl Default for SpectralDomain {
	fn default() -> Self {
		Self {
			low: 76, // 380 (nm) * 10 (Angstrom/nm) / 50 (Angstrom)
			unit: 50, // 50 Angstrom = 5 nm
			size: 81, // (780-380)/5 + 1
		}	
	}
}

impl Display for SpectralDomain {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Spectral domain ranges from {:.1} to {:.1} nm, and has {:.1} nm intervals", (self.low * self.unit) as f64/10.0, (self.high() * self.unit) as f64/10.0, self.unit as f64/10.0)
	}
}

#[test]
fn test_domain() {
	let dom = SpectralDomain::default();
	println!("{}", dom);

}
pub struct IterInterpolate {
	step: f64,
	curr: f64,
	n: usize,
	next: usize,
}

impl Iterator for IterInterpolate {

	type Item = (usize, f64);

	fn next(&mut self) -> Option<Self::Item> {
		let i = self.next;
		if i < self.n {
			let c = self.curr;
			self.next += 1;
			self.curr += self.step;
			Some((i, c))
		} else {
			None
		}

	}
}


#[test]
fn test_iter_interpolate() {
	{
		let from_domain = SpectralDomain::new(1, 2, 2); // 2, 4
		let to_domain = SpectralDomain::new(0, 5, 1); // 0, 1, 2, 3, 4, 5
		assert_eq!(from_domain.iter_interpolate(to_domain).collect::<Vec<_>>(), vec![(0, -1.0), (1, -0.5), (2, 0.0), (3, 0.5), (4, 1.0), (5, 1.5)]) ;
		// values 2,3 and 4 within range
	}

}



/// A collection of spectral distributions, sharing a 
/// common spectral domain, and represented by an nalgebra 
/// DMatrix which can be re

pub trait SpectralDistribution {
	fn spectra(&self, domain: SpectralDomain) -> DMatrix<f64>; 
		// Returns a spectral matrix from a source, or an illuminated surface, 
	    // in form of an  nalgebra's DMatrix, with and one or more spectral 
		// data as columns.

	/// spectral's native or default spectral range
	fn domain(&self) -> SpectralDomain; 

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

	/**
		Calculates tristimulus values for a spectral data source using a standard observer.
		Depending on the specral source, it will also calculate a reference white color point,
		as for example with swatch libraries.

		A default implementation is provided, which does not provide a white reference point.
	 */ 
	fn xyz<C:'static + StandardObserver>(&self,obs: &C) -> XYZ<C> {
		XYZ::<C> {
			xyz: obs.cmf(self.domain()) *self.spectra(self.domain()),
			white: None,
			cmf: C::global()

		}

	}
}

