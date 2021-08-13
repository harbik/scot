/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.


*/
pub mod incandescent;
pub mod daylight;

pub mod fluorescent;
mod fluorescent_data;
pub use self::fluorescent::*;

pub mod hid;
mod hid_data;
pub use self::hid::*;

pub mod cct;

pub use self::cct::*; // use illuminants::CCTs
pub use self::incandescent::*; 
pub use self::daylight::*;

use crate::spectra::SpectralData;



//use crate::util::{Meter, Scale};
//use crate::observers::{StandardObserver};


/**
Represents a type with a single spectral distrution, which values can be accessed 
by using its default constructor, and getting its first, and single row vector.
*/

pub trait Illuminant where
	Self:  SpectralData + Default,
//	Meter: From<<<Self as SpectralData>::ScaleType as Scale>::UnitType> 
	{}

