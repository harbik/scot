
use nalgebra::{Matrix3xX, Vector3};
use crate::spectra::{SpectralData, Illuminant};
use crate::observers::StandardObserver;

/**	
	A collection of a standard observer tristimulus values, with optional tristimulus values of a reference white point.

	Besides the tristimulus values, it also has an optional values for a white reference, and a reference to the standard observer being used.
	This to uniquely identify the observer associated with the tristimulus values, but also to be able to convert
	chromaticity coordinates between different observers, using for example transforming back to a set of reference RGB
	spectra, and calculating the tristimulus values for an other observer.

	This object's lifetime can not extend the standard observers lifetime. Most of the standard observers defined in this
	library have a static lifetime, so chromaticity values for these will be always valid.
*/
pub struct XYZ<'a, C: StandardObserver> {
	pub xyz : Matrix3xX<f64>,
	pub white: Option<Vector3<f64>>,
	pub cmf: &'a C,
}


/**
	Calculate XYZ tristimilus value using a spectral data implementer.
 */
impl<'a, C:StandardObserver, S: SpectralData> From<S> for XYZ<'a, C> {
	fn from(s: S) -> Self {
		todo!()
	}

}

/*
impl std::ops::Deref for XYZ {
    type Target = Matrix3xX<f64>;

    fn deref(&self) -> &Self::Target {
        &self.xyz
    }
}

impl XYZ {
	pub fn new(xyz: Matrix3xX<f64>, white: Option<Vector3<f64>>) -> Self {
		Self{ xyz, white}
	}
}


 */



