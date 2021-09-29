/*!
    Spectral distributions for surface illumination, from a variety of natural and artificial sources of light.

*/

//use std::ops::Mul;

//use nalgebra::Matrix3xX;


use crate::SpectralDistribution;

pub mod cct;
pub use self::cct::*;

pub mod cct_parameters;
pub use self::cct_parameters::*; // use illuminants::CCTs

pub mod blackbody;
pub use self::blackbody::*;

pub mod led;
pub use self::led::*;

/**
Represents a type with a single spectral distrution, which values can be accessed
by using its default constructor, and getting its first, and single row vector.
*/

pub trait Illuminant: SpectralDistribution + Default {}
/*
where
    Self: SpectralDistribution,
	Self: Default,
{
    // xyz data for illuminants can normalized to 100.0
    fn xyz<C>(&self) -> CieXYZ<C>
    where 
        C: StandardObserver,
        Meter: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
        Matrix3xX<f64>: Mul<Self::MatrixType>, // 
        <Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output: Mul<f64>,
        CieXYZ<C>: From< <<Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output as Mul< f64, >>::Output, >,
    {
        <Self as SpectralDistribution>::xyz(&self).normalize(100.0)
    }
}
 */

#[macro_export]
macro_rules! illuminant {
    // a single illuminant from static slice column
    ($ILL:ident, $N:expr, $M:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident) => {
        #[derive(Debug, Default)]
        pub struct $ILL<const I: usize>;

        impl<const I: usize> $crate::SpectralDistribution for $ILL<I> {
            type MatrixType = nalgebra::SMatrixSlice<'static, f64, $N, 1>;
            type StepType = $crate::WavelengthStep;

            fn shape(&self) -> (usize, usize) {
                ($N, 1)
            }

            fn spd(&self) -> ($crate::Domain<Self::StepType>, Self::MatrixType) {
                assert!(I > 0 && I <= $M);
                (
                    $DOMAIN,
                    <Self as $crate::SpectralDistribution>::MatrixType::from_slice(
                        &$DATA[(I - 1) * N..I * N],
                    ),
                )
            }

            fn description(&self) -> Option<String> {
                Some(format!($DESC, I))
            }
        }

        impl<const I: usize> $crate::illuminants::Illuminant for $ILL<I> {}

        impl<C: $crate::observers::StandardObserver, const I: usize> From<$ILL<I>>
            for $crate::models::CieXYZ<C>
        {
            fn from(ill: $ILL<I>) -> Self {
                $crate::spectra::SpectralDistribution::xyz(&ill)
            }
        }
    };
    // all illuminants as array with keys
    ($ILL:ident, $N:expr, $M:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident, $KEYS:ident) => {
        #[derive(Debug, Default)]
        pub struct $ILL;

        impl $crate::SpectralDistribution for $ILL {
            type MatrixType = nalgebra::SMatrixSlice<'static, f64, $N, $M>;
            type StepType = $crate::WavelengthStep;

            fn shape(&self) -> (usize, usize) {
                ($N, $M)
            }

            fn spd(&self) -> ($crate::Domain<Self::StepType>, Self::MatrixType) {
                (
                    $DOMAIN,
                    <Self as $crate::SpectralDistribution>::MatrixType::from_slice(&$DATA),
                )
            }

            fn keys(&self) -> Option<Vec<String>> {
                Some($KEYS.iter().map(|s| s.to_string()).collect())
            }

            fn description(&self) -> Option<String> {
                Some(format!($DESC))
            }
        }

        impl $crate::illuminants::Illuminant for $ILL {}

        impl<C: $crate::observers::StandardObserver> From<$ILL> for $crate::models::CieXYZ<C> {
            fn from(ill: $ILL) -> Self {
                use $crate::spectra::SpectralDistribution;
                ill.xyz().normalize(100.0)
            }
        }
    };
    // single data illuminant
    ($ILL:ident, $N:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident) => {
        #[derive(Debug, Default)]
        pub struct $ILL;

        impl $crate::SpectralDistribution for $ILL {
            type MatrixType = nalgebra::SMatrixSlice<'static, f64, $N, 1>;
            type StepType = $crate::WavelengthStep;

            fn shape(&self) -> (usize, usize) {
                ($N, 1)
            }

            fn spd(&self) -> ($crate::Domain<Self::StepType>, Self::MatrixType) {
                (
                    $DOMAIN,
                    <Self as $crate::SpectralDistribution>::MatrixType::from_slice(&$DATA),
                )
            }

            fn keys(&self) -> Option<Vec<String>> {
                None
            }

            fn description(&self) -> Option<String> {
                Some(format!($DESC))
            }
        }

        impl $crate::illuminants::Illuminant for $ILL {}

        impl<C: $crate::observers::StandardObserver> From<$ILL> for $crate::models::CieXYZ<C> {
            fn from(ill: $ILL) -> Self {
                use $crate::spectra::SpectralDistribution;
                ill.xyz().normalize(100.0)
            }
        }
    };
}

macro_rules! illuminant_single_test {
    ($FNAME:ident, $ILL:ident,  $X:expr, $XTOL:expr, $Y:expr, $YTOL:expr) => {
        #[test]
        fn $FNAME() {
            use crate::models;
            use crate::observers::CieObs1931;
            use approx::assert_abs_diff_eq;

            let xyz: models::CieYxy<CieObs1931> = $ILL::default().into();
            assert_abs_diff_eq!(xyz.data.column(0).y, $X, epsilon = $XTOL); // CIE 15:2004, Table T.3. D50 x value
            assert_abs_diff_eq!(xyz.data.column(0).z, $Y, epsilon = $YTOL); // CIE 15:2004, Table T.3. D50 y value - there is a slight deviation here... 50 vs 51
        }
    };
}

#[cfg(feature = "cie_fluorescent_illuminants")]
pub mod fluorescent_cie;

#[cfg(feature = "cie_fluorescent_illuminants")]
pub use self::fluorescent_cie::*;

//#[cfg(feature="ies_tm30_fluorescent_illuminants")]
//pub mod fluorescent_ies_tm30;

//#[cfg(feature="ies_tm30_fluorescent_illuminants")]
//pub use self::fluorescent_ies_tm30::*;

#[cfg(feature = "cie_hid_illuminants")]
pub mod hid_cie;

#[cfg(feature = "cie_hid_illuminants")]
pub use self::hid_cie::*;

#[cfg(feature = "cie_led_illuminants")]
pub mod led_cie;

#[cfg(feature = "cie_led_illuminants")]
pub use self::led_cie::*;

pub mod daylight;
pub use self::daylight::*;

pub use illuminant;
