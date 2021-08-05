/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.


*/
pub mod incandescent;
pub mod daylight;

pub mod fluorescent;
pub mod fluorescent_data;
pub mod cct;

pub use crate::illuminants::cct::CCTs; // use illuminants::CCTs
pub use crate::illuminants::incandescent::Planckian; 
pub use crate::illuminants::daylight::{Daylight, D65, D50, D55, D75};
use crate::spectra::SpectralData;
//use crate::observers::{StandardObserver};


/**
Represents a type with a single spectral distrution, which values can be accessed 
by using its default constructor, and getting its first, and single row vector.
*/

pub trait Illuminant : SpectralData + Default {}

pub const ALL: usize = 0;
