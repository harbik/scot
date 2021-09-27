/*!

# Spectral Distribution Representations

A spectral distribution represents the composition of electromagnetic radiation
– called light when visible – for a domain of wavelengths (waves)
or a domain of photon energies (photons),
and is probably most familiar for you in form spectral plot,
such as the spectral distribution of daylight, shown below.

    Add D65 spectral plot, and how it is generated

In this case it has a domain, shown along the horizontal axis, with a wavelength scale,
with values from 380 to 780 nanometer,
and a range from 0.0 to approximately xxx W·m<sup>2</sup>nm<sup>2</sup>.
This plot is scaled such that the surface under the curve equals a value of 1000 W·m<sup>-2</sup>,
which is approximately the maximum daylight irradiance at earth, at sea level, at a clear day,
at normal incidence ('midsummer noon'), resulting in an illuminance of approximately 120_000 lm·m<sup>-2</sup>

# Quantities and Units of Spectral Distributions

There are many different types of spectral distributions, depending on the application,
and depending on the type of spectral domains.

Spectral domains can have a wavelength scale, or a photon energy scale, depending if you consider
electromagnetic radiation to be composed of electromagnetic waves, of photons.
In colorimetry the most common –and the scale used in every colorimetric standard– is the wavelength scale,
with as most common wavelength unit a nanometer, or *nm*, typically ranging from 380 to 780nm.

Although for historic reasons wavelength domains are almost exclusively used in colorimetry,
from a physics and physiology perspective (the cones in our eyes are sophisticated photon counters),
spectral distributions representations with photonenergy domains, and photon flux based range scales,
would be more appropiate.
Changing the scales for domain and range of a spectral distibution from wavelength/power to photon-energy/photon-flux
would not change the results of all the developed color models,
as they are developed to model experimental data,
but it could simplify, or improve the colorimetry models.

## Wavelength/Radiant Flux Scales

- *Spectral Irradiance*, with unit W·m<sup>-2</sup>m<sup>-1</sup>, or W·m<sup>-2</sup>nm<sup>-1</sup>, used for illuminants.
Used to characterize the illuminance and color rendering of surface colors, textiles, and transparent colored materials.

- *Spectral Luminance*, with unit cd·m<sup>-2</sup>m<sup>-1</sup>, or cd·m<sup>-2</sup>nm<sup>-1</sup>, mostly used for displays.

- *Spectral Flux*, with unit W·m<sup>-1</sup>, or W·nm<sup>-1</sup>, for the total output of a lamp, as measured in an integrating sphere,
or as measured with a spectro-photogoniometer.

- *Spectral Reflectivity* and *Spectral Transmissivity*, with unit m<sup>-2</sup>m<sup>-1</sup>, or m<sup>-2</sup>nm<sup>-1</sup>,
for surfaces and transparent media.


## Photon Energy/Photon Flux Scales


# Sources for Spectral Distribution Data

Spectral distribution data can be generated by mathematical models,
such as a blackbody emitter using Planck's law,
or can be originating from spectrometer measurements.

## Mathematical models

## International Standards

## Spectral Data Libraries

## Measurements



# Roadmap Ideas

- [ ] Add general introduction spectral distributions
- [ ] Support varying data formats: f32 and u16, scale value
- [ ] Use data from files (zero-copy, rkyv?)
- [ ] Explore existing spectral data file formats
- [ ] Support phothonenergy domains?
- [ ] Add spectral plot capabilities (SVG?)
- [ ] DataSpectraFromSlice, with new(domain, number_of_spectra, slice)
- [ ] map_domain -> interpolate, producing a generic DataSpectra type

 */
use std::ops::{Index, Mul};

use crate::{
    lin_interp_mat_col, models::CieXYZ, observers::StandardObserver, Domain, Meter, Step, Unit,
    WavelengthStep,
};
use nalgebra::{DMatrix, DVectorSlice, Matrix3xX};

pub trait SpectralDistribution {
    // type ValueType: num::ToPrimitive = f64;  // mag waarschijnlijk niet hier...
    // could use num::Float as a first step
    // type MatrixType: Index<(usize,usize), Output = Self::ValueType>;
    type MatrixType: Index<(usize, usize), Output = f64>;
    type StepType: Step;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType);

    fn shape(&self) -> (usize, usize);

    /// Optional keys for each of the spectral distribution in the collection.
    fn keys(&self) -> Option<Vec<String>> {
        None
    }
    //  here implemented as a default method, to be overridden if applicable

    /// Optional description of spectral collection.
    fn description(&self) -> Option<String> {
        None
    }

    fn map_domain<S2: Step>(&self, dto: Domain<S2>) -> DMatrix<f64>
    where
        <<Self as SpectralDistribution>::StepType as Step>::UnitValueType:
            From<<S2 as Step>::UnitValueType>,
    {
        let (dfr, s) = self.spd();
        //	sprague_cols_index_based::<_, S2, _>(&dfr, &dto, s, self.len())
        //		sprague_cols_index_based(&dfr, &dto, s, self.len())
        lin_interp_mat_col(&dfr, &dto, self.shape().1, s)
    }

    fn xyz<C>(&self) -> CieXYZ<C>
    where
        C: StandardObserver,
        Meter: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
        Matrix3xX<f64>: Mul<Self::MatrixType>,
        <Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output: Mul<f64>,
        CieXYZ<C>: From<
            <<Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output as Mul<
                f64,
            >>::Output,
        >,
    {
        let (d, s) = self.spd();
        let xyz = (C::values(&d) * s) * (C::K * C::domain().step.unitvalue(1).value());
        CieXYZ::<C>::from(xyz)
    }
}

/**
   Use SpectralDistribution trait methods on a data slice, using a matrix slice
   without allocation. Only one spectrum here.

*/
pub struct DataSpectrumFromSlice<'a> {
    d: Domain<WavelengthStep>,
    m: &'a [f64],
}

impl<'a> DataSpectrumFromSlice<'a> {
    pub fn new(d: Domain<WavelengthStep>, m: &'a [f64]) -> Self {
        assert_eq!(d.len(), m.len());
        Self { d, m }
    }
}

impl<'a> SpectralDistribution for DataSpectrumFromSlice<'a> {
    type MatrixType = DVectorSlice<'a, f64>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (self.d.clone(), <Self::MatrixType>::from(self.m))
    }

    fn shape(&self) -> (usize, usize) {
        (self.d.len(), 1)
    }
}
