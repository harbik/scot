/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.


*/
pub mod blackbody;
pub mod cct;

pub use crate::illuminants::cct::CCTs; // use illuminants::CCTs
pub use crate::illuminants::blackbody::Blackbody; // use illuminants::CCTs