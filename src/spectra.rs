
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
	pub high: usize, // maximum wavelength, in 'unit'
	pub unit: usize,  // Angstrom
}


impl SpectralDomain {
    pub fn new(low: usize, high: usize, unit: usize) -> Self { Self { low, high, unit } }

	pub fn len(&self) -> usize {
		self.high - self.low + 1
	}

}

impl Default for SpectralDomain {
	fn default() -> Self {
		Self {
			low: 76, // 76 * 50 = 3800 Angstrom = 380 nm
			high: 156, // 156 * 50 = 7800 Angstrom = 780 nm
			unit: 50, // 50 Angstrom = 5 nm
		}	
	}
}

impl Display for SpectralDomain {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Spectral Range: {:.1} - {:.1} nm with {:.1} nm intervals", (self.low * self.unit) as f64/10.0, (self.high * self.unit) as f64/10.0, self.unit as f64/10.0)
	}
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

	fn domain(&self) -> SpectralDomain; 
		// spectral's native or default spectral range
}
