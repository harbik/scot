
use nalgebra::{Matrix3xX};
use crate::spectra::{SpectralData, Illuminant};

pub struct Cie {
	xyz : Matrix3xX<f64>
}

impl Cie {
	pub fn new(xyz: Matrix3xX<f64>) -> Self {
		Self{ xyz}
	}
}

impl<V: SpectralData + Illuminant> From<V> for Cie {

	fn from(s: V) -> Self {
		todo!()

	}



}


impl std::ops::Deref for Cie {
    type Target = Matrix3xX<f64>;

    fn deref(&self) -> &Self::Target {
        &self.xyz
    }
}


