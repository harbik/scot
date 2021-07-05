
use nalgebra::{Matrix3xX, Vector3};
use crate::spectra::{Illuminant, SpectralDistribution};
use crate::observers::StandardObserver;

/**	
	A collection of a tristimulus values, an associated reference to a standard observer,
	and  an optional set of tristimulus values of a reference white point.

	
	The reference to a standard observers color matching functions is not only used to uniquely identify the observer
	associated with the tristimulus values, but also for the conversion of chromaticity coordinates between different
	observers, using for example transforming back to a set of reference RGB spectra, and calculating the tristimulus
	values for a different observer. The standard observers have global (static) scope.

*/
pub struct XYZ<C: StandardObserver> {
	pub xyz : Matrix3xX<f64>,
	pub white: Option<Vector3<f64>>,
	pub cmf: &'static C,
}


/**
	Calculate XYZ tristimilus value from spectral distributions.

	A default xyz method is available in the SpectralDistribution trait, but can be overridden by spectral distributions
	if more efficient or more appropiate methods are available.
 */
impl<C, S> From<S> for XYZ<C> 
	where 
		S: SpectralDistribution,
		C: StandardObserver
{
	fn from(s: S) -> Self {
		s.xyz(C::global())
	}

}



