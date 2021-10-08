#![doc = include_str!("mod.md")]
mod data;
pub use data::*;

use self::data::{M, N, TM30_CIE1931, TM30_ILLUMINANTS_DATA};
use colorado::illuminants::Illuminant;
use colorado::models::{CieXYZ, XYZValues};
use colorado::observers::StandardObserver;
use colorado::{DataSpectrumFromSlice, Domain, SpectralDistribution, WavelengthStep, NM};
use nalgebra::{Matrix3xX, SMatrixSlice, SVectorSlice};
use std::collections::HashMap;

/**
Use TM30 Sample Spectrum as illuminant.

Illuminants are used in this library to represent the, typically 'white', illumination
used to illuminate objects such as swatches, or backlight LCD pixels.
Each illuminant in the colorado library has its own type, and is constructed by its
`Default::default` method only.

The use the spectral distributions in the TM30 library as illuminant in the color models
use the `TM30Illuminant::<const K:usize>` type, where `K` can be specified as one of the
TM30 library constants.

For example, to get the CIE F1 illuminant from this library, use the `CIE_F1` constant:
```
    use colorado_tm30::samples::{TM30Illuminant, CieF1};

    use colorado::models::{CieYxy, YxyValues};
    let xy: CieYxy = CieF1::default().into();
    let YxyValues{l:_, x, y} = xy.into_iter().next().unwrap();

    use approx::assert_abs_diff_eq;
    assert_abs_diff_eq!(x,0.313100, epsilon=1E-6);
    assert_abs_diff_eq!(y,0.337279, epsilon=1E-6);

```
*/
#[derive(Default)]
pub struct TM30Illuminant<const K: usize>;

impl<const K: usize> SpectralDistribution for TM30Illuminant<K> {
    type MatrixType = SVectorSlice<'static, f64, N>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (
            Domain::new(380, 780, NM),
            Self::MatrixType::from_slice(&TM30_ILLUMINANTS_DATA[(K - 1) * N..K * N]),
        )
    }

    fn shape(&self) -> (usize, usize) {
        (N, 1)
    }
}

impl<const K: usize> Illuminant for TM30Illuminant<K> {}

impl<C: StandardObserver, const K: usize> From<TM30Illuminant<K>> for CieXYZ<C> {
    fn from(_: TM30Illuminant<K>) -> Self {
        TM30Illuminant::<K>::default().xyz().normalize(100.0)
    }
}

#[test]
fn test_tm30_ill() {
    use crate::samples::CieF1;
    use approx::assert_abs_diff_eq;
    use colorado::models::{CieYxy, YxyValues};

    let ill = CieF1::default();
    let xy: CieYxy = ill.into();
    let YxyValues { l: _, x, y } = xy.into_iter().next().unwrap();

    assert_abs_diff_eq!(x, 0.313100, epsilon = 1E-6);
    assert_abs_diff_eq!(y, 0.337279, epsilon = 1E-6);
}

pub struct TM30SampleSpectra(
    Domain<<Self as SpectralDistribution>::StepType>,
    <Self as SpectralDistribution>::MatrixType,
);

impl Default for TM30SampleSpectra {
    fn default() -> Self {
        Self(
            Domain::new(380, 780, NM),
            <Self as SpectralDistribution>::MatrixType::from_slice(&TM30_ILLUMINANTS_DATA),
        )
    }
}

impl SpectralDistribution for TM30SampleSpectra {
    type MatrixType = SMatrixSlice<'static, f64, N, M>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (self.0.clone(), self.1)
    }

    fn shape(&self) -> (usize, usize) {
        (N, M)
    }
}

impl<C: StandardObserver> From<TM30SampleSpectra> for CieXYZ<C> {
    fn from(ill: TM30SampleSpectra) -> Self {
        ill.xyz().normalize(100.0)
    }
}

#[test]
fn test_tm30_sample_spectra() {
    use colorado::models::CieYxy;
    let ill = TM30SampleSpectra::default();
    let xy: CieYxy = ill.into();
    println! {"{}", xy.data.transpose()};
}

#[derive(Clone, PartialEq, Eq)]
pub enum EmissionType {
    FluorescentBroadband = 0,
    FluorescentNarrowband = 1,
    HighIntensityDischarge = 3,
    IncandescentOrFilament = 4,
    LedHybrid = 5,
    LedMixed = 6,
    LedPhosphor = 7,
    Mathematical = 8,
    Other = 9,
}

pub enum ModelType {
    Model = 0,
    Commercial = 1,
    Experimental = 2,
    Theoretical = 3,
}

pub fn tm30_cie1931_xy() -> HashMap<&'static str, [f64; 2]> {
    TM30_CIE1931
        .iter()
        .map(|(key, _, _, x, y)| (*key, [*x, *y]))
        .collect()
}

impl From<EmissionType> for Vec<&str> {
    fn from(et: EmissionType) -> Self {
        let e = et as u32;
        let mut v: Vec<&str> = Vec::with_capacity(M);
        for (k, j, ..) in TM30_CIE1931.iter() {
            if e == *j {
                v.push(k);
            }
        }
        v
    }
}

impl<C: StandardObserver> From<EmissionType> for CieXYZ<C> {
    fn from(et: EmissionType) -> Self {
        let e = et as u32;
        let mut v: Vec<f64> = Vec::with_capacity(3 * M);
        for (i, (_, j, ..)) in TM30_CIE1931.iter().enumerate() {
            if e == *j {
                let sd = DataSpectrumFromSlice::new(
                    Domain::new(380, 780, NM),
                    &TM30_ILLUMINANTS_DATA[i * N..(i + 1) * N],
                );
                let XYZValues { x, y, z } = sd.xyz::<C>().into_iter().next().unwrap();
                v.push(x);
                v.push(y);
                v.push(z);
            }
        }
        Self::new(Matrix3xX::from_vec(v))
    }
}

#[test]
fn test_from_emission_type() {
    use approx::assert_abs_diff_eq;
    use colorado::models::{CieYxy, YxyValues};

    for emission_type in [
        EmissionType::FluorescentNarrowband,
        EmissionType::FluorescentBroadband,
        EmissionType::HighIntensityDischarge,
        EmissionType::IncandescentOrFilament,
        EmissionType::LedHybrid,
        EmissionType::LedMixed,
        EmissionType::LedPhosphor,
        EmissionType::Mathematical,
        EmissionType::Other,
    ] {
        let xyz: CieYxy = emission_type.clone().into();
        let keys: Vec<&str> = emission_type.into();
        let w = tm30_cie1931_xy();
        for (YxyValues { l: _, x, y }, k) in xyz.into_iter().zip(keys.into_iter()) {
            let [xw, yw] = w[k];
            assert_abs_diff_eq!(x, xw, epsilon = 5E-7);
            assert_abs_diff_eq!(y, yw, epsilon = 5E-7);
            println!("{} {} {} {} {}", k, x, y, xw, yw);
        }
    }
}
