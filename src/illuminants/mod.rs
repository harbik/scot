/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.


*/

pub mod cct;
pub use self::cct::*; // use illuminants::CCTs

pub mod incandescent;
mod incandescent_data;
pub use self::incandescent::*; 

pub mod daylight;
pub use self::daylight::*;

pub mod fluorescent;
mod fluorescent_data;
pub use self::fluorescent::*;

pub mod hid;
mod hid_data;
pub use self::hid::*;

pub mod led;
mod led_data;
pub use self::led::*;

use crate::spectra::SpectralData;



/**
Represents a type with a single spectral distrution, which values can be accessed 
by using its default constructor, and getting its first, and single row vector.
*/

pub trait Illuminant where
	Self:  SpectralData + Default,
//	Meter: From<<<Self as SpectralData>::ScaleType as Scale>::UnitType> 
	{}

