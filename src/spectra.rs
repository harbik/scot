
extern crate nalgebra as na;

use na::DMatrix;

use crate::observers::StandardObserver;
use crate::cie::xyz::XYZ;
use crate::util::domain::{Domain};
use crate::util::units::{MeterScale, Scale, Meter};

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

	type UnitType: Scale;
	type UnitValue;

	/**
		Values for a set of spectral distributions.

		Returns values for any spectral representation of light, such as from a light source, or an illuminated surface,
		in form of an nalgebra's `DMatrix<f64>`, with  one or more spectral distribution data as columns. The values are
		mapped to a specified domain, typically by interpolation, or by evaluation of functions for functional
		representations.
	*/
	fn values<L:Scale>(&self, domain: Domain<L>) -> DMatrix<f64>
		where
			L: Scale,
			Self::UnitValue: From<<L>::ValueType>
			; 

	/// spectral's native or default spectral range
	fn domain(&self) -> Domain<Self::UnitType>; 

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

}

/**
	Calculate XYZ tristimilus value from spectral distributions.
 */
impl<C: StandardObserver, S: SpectralDistribution> From<S> for XYZ<C> {
	fn from(sd: S) -> Self {
		XYZ::<C> {
			xyz: C::global().cmf(sd.domain()) * sd.values(sd.domain()),
			white: None,
			cmf: C::global()
		}
	}
}
