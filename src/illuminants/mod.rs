/*!
	Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.

*/

use std::ops::Mul;

use nalgebra::Const;
use nalgebra::Dynamic;
use nalgebra::Matrix;
use nalgebra::Matrix3xX;
use nalgebra::VecStorage;

use crate::Meter;
use crate::SpectralDistribution;
use crate::Step;
use crate::Unit;
use crate::models::CieXYZ;
use crate::observers::StandardObserver;

pub mod cct;
pub use self::cct::*;

pub mod cct_parameters;
pub use self::cct_parameters::*; // use illuminants::CCTs

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


pub trait Illuminant<C> where
	C: StandardObserver,
	Self: SpectralDistribution,
	Self: Into<CieXYZ<C>>,
	Self: Default,
	Meter: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
	Matrix3xX<f64>: Mul<Self::MatrixType>,
	<Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output: Mul<f64>,
	CieXYZ::<C>: From<<<Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output as Mul<f64>>::Output>
	{
		fn xyz(&self) -> CieXYZ<C> {
			let (d, s) = self.spd();
			let xyz = (C::values(&d) * s) * (C::K * C::domain().step.unitvalue(1).value());
			CieXYZ::<C>::from(xyz)
		}
	 }


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

