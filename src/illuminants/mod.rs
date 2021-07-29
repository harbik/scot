/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.


*/
pub mod incandescent;
pub mod daylight;
pub mod cct;

pub use crate::illuminants::cct::CCTs; // use illuminants::CCTs
pub use crate::illuminants::incandescent::Planckian; 
pub use crate::illuminants::daylight::Daylight;
//use crate::observers::{StandardObserver};
pub trait Illuminant {}