/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.

*/

use crate::spectra::SpectralData;

pub mod cct;
pub use self::cct::*; // use illuminants::CCTs

pub mod daylight;
pub use self::daylight::*;

pub mod blackbody;
pub use self::blackbody::*; 

pub mod led;
pub use self::led::*;


/**
Represents a type with a single spectral distrution, which values can be accessed 
by using its default constructor, and getting its first, and single row vector.
*/

pub trait Illuminant where
	Self:  SpectralData + Default,
	{}

/*
	Optional illuminant data libraries, which can be excluded by feature flags.
	This libraries are big in size, so a user might want to exlude them if small packages are 
	required. They are included by default, to make it easy to use the library for general 
	color science work.
*/

#[cfg(feature="ies_tm30_incandescent_illuminants")]
pub mod incandescent_ies_tm30;

#[cfg(feature="ies_tm30_incandescent_illuminants")]
pub use self::incandescent_ies_tm30::*; 

#[cfg(feature="cie_fluorescent_illuminants")]
pub mod fluorescent_cie;

#[cfg(feature="cie_fluorescent_illuminants")]
pub use self::fluorescent_cie::*;

#[cfg(feature="ies_tm30_fluorescent_illuminants")]
pub mod fluorescent_ies_tm30;

#[cfg(feature="ies_tm30_fluorescent_illuminants")]
pub use self::fluorescent_ies_tm30::*;

#[cfg(feature="cie_hid_illuminants")]
pub mod hid_cie;

#[cfg(feature="cie_hid_illuminants")]
pub use self::hid_cie::*;

#[cfg(feature="ies_tm30_hid_illuminants")]
pub mod hid_ies_tm30;

#[cfg(feature="ies_tm30_hid_illuminants")]
pub use self::hid_ies_tm30::*;

#[cfg(feature="cie_led_illuminants")]
pub mod led_cie;

#[cfg(feature="cie_led_illuminants")]
pub use self::led_cie::*;

#[cfg(feature="ies_tm30_led_illuminants")]
pub mod led_ies_tm30;

#[cfg(feature="ies_tm30_led_illuminants")]
pub use self::led_ies_tm30::*;

