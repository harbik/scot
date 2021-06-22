
extern crate nalgebra as na;


use std::fmt;
use std::fmt::{Display, Formatter};

use na::DMatrix;

pub trait Illuminant {}
/// trait marker for illuminant spectra

pub trait Swatch {}
/// trait marker for swatch reflection spectra, 
/// such as the Munsell color swatches.

pub trait Pixel {}
/// trait marker for display pixel spectra

pub trait StandardObserver {}
/// trait marker for a Standard observer

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

pub type Spectra = DMatrix<f64>;

/// A collection of spectral distributions, sharing a 
/// common spectral domain, represented by an nalgebra 
/// DMatrix.

pub trait SpectralData {
	fn spectra(&self, domain: SpectralDomain) -> Spectra; 
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
}
