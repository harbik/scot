
extern crate nalgebra as na;

use na::DMatrix;

use crate::observers::StandardObserver;
use crate::cie::xyz::XYZ;
use crate::util::domain::{Domain};
use crate::util::units::{Meter};

pub trait Illuminant {}
/// trait marker for illuminant spectra

pub trait Swatch {}
/// trait marker for swatch reflection spectra, 
/// such as the Munsell color swatches.

pub trait Pixel {}
/// trait marker for display pixel spectra


/**
Generic methods and operations for spectral distributions.

A collection of spectral distributions, sharing a common spectral domain, and represented by an nalgebra DMatrix which
can be re
*/

pub trait SpectralDistribution {

	/**
		Values for a set of spectral distributions.

		Returns values for any spectral representation of light, such as from a light source, or an illuminated surface,
		in form of an nalgebra's `DMatrix<f64>`, with  one or more spectral distribution data as columns. The values are
		mapped to a specified domain, typically by interpolation, or by evaluation of functions for functional
		representations.
	*/
	fn values(&self, domain: Domain<Meter>) -> DMatrix<f64>; 

	/// spectral's native or default spectral range
	fn domain(&self) -> Domain<Meter>; 

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

	/**
		Calculates tristimulus values for a set of distributions using a standard observer.

		Depending on the spectral source, it will also calculate a reference white color point.
		A default implementation is provided below, which does not provide a reference point -- this has to be added
		manually, if needed. 
		
		Spectral distributions might overwrite this method. An example of this is the blackbody radiator, which
		typically has a very wide domain, much wider than that of a color matching function. There blackbody function
		values are directly calculated using the domain of the color matching functions provided.
	 */ 
	fn xyz<C:'static + StandardObserver>(&self,obs: &C) -> XYZ<C> {
		XYZ::<C> {
			xyz: obs.cmf(self.domain()) * self.values(self.domain()),
			white: None,
			cmf: C::global()

		}
	}
}

