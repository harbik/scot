
use nalgebra::{Matrix3xX, Vector3};
use crate::spectra::{SpectralData, Illuminant};

/**	
	A collection of a standard observer tristimulus values, with optional tristimulus values of a reference white point.
*/
pub struct XYZ {
	xyz : Matrix3xX<f64>,
	white: Option<Vector3<f64>>,
}

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





