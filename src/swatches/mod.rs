#![doc = include_str!("mod.md")]

use std::marker::PhantomData;

use nalgebra::{Const, DMatrix, OMatrix};

use crate::illuminants::Illuminant;
use crate::models::{cielab, CieLab};
use crate::observers::StandardObserver;
use crate::{Domain, Meter, SpectralDistribution, Step, Unit, WavelengthStep, DOMAIN_DEFAULT_LEN};

pub mod spline_swatch;
pub use spline_swatch::*;

pub mod data_swatch;
pub use data_swatch::*;


/**
    Traits for swatches, libraries or models for color samples, to get their spectral distributions
    and, with a specified illuminant, their color appearance coordinates.
*/
pub trait Swatch
where
    Self: SpectralDistribution,
   // Self: Default,
{
    fn lab<I, C>(&self) -> CieLab<I, C>
    where
        C: StandardObserver,
        //I: Into::<CieXYZ<C>>,
        I: Default,
        I: SpectralDistribution,
        Meter: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
        //		I: <I::StepType = Self::StepType>,
        <<I as SpectralDistribution>::StepType as Step>::UnitValueType:
            From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
    {
        let (d, s) = self.spd();
        let c = C::values(&d);
        let l = I::default().map_domain(d.clone());
        //	println!("****** {} {} {}", d.len(), self.shape().0, self.shape().1);
        let m: DMatrix<f64> =
            DMatrix::from_fn(l.nrows(), self.shape().1, |i, j| l[(i, 0)] * s[(i, j)]);
        let xyzn = &c * l.column(0) * C::K * d.step.unitvalue(1).value();
        let xyz = c * m * C::K * d.step.unitvalue(1).value();
        CieLab{ data: cielab(xyzn, xyz), cmf: PhantomData, illuminant: PhantomData}
    }
}
/**
    Macro to define a a swatch library from static data, and implement its `Swatch` traits.

    Examples of these swatch libaries defined as static data are the color checker
    color samples, and the color samples used in various color quality standards, such as the
    CIE CRI and IES TM30.

*/

#[macro_export]
macro_rules! swatch {
    // a single illuminant from static slice column
    ($SWATCH:ident, $N:expr, $M:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident) => {
        #[derive(Debug, Default)]
        pub struct $SWATCH<const J: usize>;

        impl<const J: usize> $crate::SpectralDistribution for $SWATCH<J> {
            type MatrixType = nalgebra::SMatrixSlice<'static, f64, $N, 1>;
            type StepType = $crate::WavelengthStep;

            fn shape(&self) -> (usize, usize) {
                ($N, 1)
            }

            fn spd(&self) -> ($crate::Domain<Self::StepType>, Self::MatrixType) {
                assert!(J > 0 && J <= $M);
                (
                    $DOMAIN,
                    <Self as $crate::SpectralDistribution>::MatrixType::from_slice(
                        &$DATA[(J - 1) * N..J * N],
                    ),
                )
            }

            fn description(&self) -> Option<String> {
                Some(format!($DESC, J))
            }
        }

        impl<const J: usize> $crate::swatches::Swatch for $SWATCH<J> {}

        impl<
                I: $crate::illuminants::Illuminant,
                C: $crate::observers::StandardObserver,
                const J: usize,
            > From<$SWATCH<J>> for $crate::models::CieLab<I, C>
        where
            <<I as $crate::SpectralDistribution>::StepType as $crate::Step>::UnitValueType:
                From<$crate::Meter>,
        {
            fn from(sw: $SWATCH<J>) -> Self {
                use $crate::swatches::Swatch;
                sw.lab()
            }
        }
    };
    // all swatches as array with keys
    ($SWATCH:ident, $N:expr, $M:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident, $KEYS:ident) => {
        #[derive(Debug, Default)]
        pub struct $SWATCH;

        impl $crate::SpectralDistribution for $SWATCH {
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

        impl $crate::swatches::Swatch for $SWATCH {}

        impl<I: $crate::illuminants::Illuminant, C: $crate::observers::StandardObserver>
            From<$SWATCH> for $crate::models::CieLab<I, C>
        where
            <<I as $crate::SpectralDistribution>::StepType as $crate::Step>::UnitValueType:
                From<$crate::Meter>,
        {
            fn from(sw: $SWATCH) -> Self {
                use $crate::swatches::Swatch;
                sw.lab()
            }
        }
    };
}

macro_rules! impl_greys {
	($ ($I:ident=$R:literal),*) => {
		$(
			pub type $I = Gray::<$R>;
		 )*
	};
}

impl_greys!(
    Black = 0,
    G0 = 0, G1 = 1, G2 = 2, G3 = 3, G4 = 4, G5 = 5, G6 = 6, G7 = 7, G8 = 8, G9 = 9,
    G10 = 10, G11 = 11, G12 = 12, G13 = 13, G14 = 14, G15 = 15, G16 = 16, G17 = 17, G18 = 18, G19 = 19,
    G20 = 20, G21 = 21, G22 = 22, G23 = 23, G24 = 24, G25 = 25, G26 = 26, G27 = 27, G28 = 28, G29 = 29,
    G30 = 30, G31 = 31, G32 = 32, G33 = 33, G34 = 34, G35 = 35, G36 = 36, G37 = 37, G38 = 38, G39 = 39,
    G40 = 40, G41 = 41, G42 = 42, G43 = 43, G44 = 44, G45 = 45, G46 = 46, G47 = 47, G48 = 48, G49 = 49,
    G50 = 50, G51 = 51, G52 = 52, G53 = 53, G54 = 54, G55 = 55, G56 = 56, G57 = 57, G58 = 58, G59 = 59,
    G60 = 60, G61 = 61, G62 = 62, G63 = 63, G64 = 64, G65 = 65, G66 = 66, G67 = 67, G68 = 68, G69 = 69,
    G70 = 70, G71 = 71, G72 = 72, G73 = 73, G74 = 74, G75 = 75, G76 = 76, G77 = 77, G78 = 78, G79 = 79,
    G80 = 80, G81 = 81, G82 = 82, G83 = 83, G84 = 84, G85 = 85, G86 = 86, G87 = 87, G88 = 88, G89 = 89,
    G90 = 90, G91 = 91, G92 = 92, G93 = 93, G94 = 94, G95 = 95, G96 = 96, G97 = 97, G98 = 98, G99 = 99,
    G100 = 100,
    White = 100
);

#[derive(Default)]
pub struct Gray<const R: usize>;

impl<const R: usize> SpectralDistribution for Gray<R> {
    type MatrixType = OMatrix<f64, Const<DOMAIN_DEFAULT_LEN>, Const<1>>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (
            Domain::default(),
            Self::MatrixType::from_element(R as f64 / 100.0),
        )
    }

    fn shape(&self) -> (usize, usize) {
        (DOMAIN_DEFAULT_LEN, 1)
    }

    fn map_domain<S2: Step>(&self, dto: Domain<S2>) -> DMatrix<f64>
    where
        <<Self as SpectralDistribution>::StepType as Step>::UnitValueType: From<S2::UnitValueType>,
    {
        DMatrix::from_element(dto.len(), 1, R as f64 / 100.0)
    }
}

impl<I: Illuminant, C: StandardObserver, const R: usize> From<Gray<R>> for CieLab<I, C>
where
    <<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<Meter>,
{
    fn from(g: Gray<R>) -> Self {
        g.lab()
    }
}

impl<I: Illuminant, C: StandardObserver, const R: usize> From<&Gray<R>> for CieLab<I, C>
where
    <<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<Meter>,
{
    fn from(g: &Gray<R>) -> Self {
        g.lab()
    }
}

impl<const R: usize> Swatch for Gray<R> {}


pub use swatch;
