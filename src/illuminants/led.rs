use nalgebra::{Dynamic, OMatrix};
use num::ToPrimitive;

use crate::{
    led_ohno, models::CieXYZ, observers::StandardObserver, Domain, SpectralDistribution, Step,
    Unit, WavelengthStep, NM,
};

//use super::Illuminant;

/**
    Input parameters for single, direct LED emission models.

    This covers only simple models, for non-white or monochrome LEDs.
    The input parameters consist of a peak wavelength – the wavelength where emission is maximal – and
    a full-width-half-maximum spectral width, both in units of meter, and a power value.
    A power value of 0.0 indicates a 'don't care' value, to be used if you're only interested in chromaticity, or it is
    scaled in later step in the calculation anyway. There can be a calculation speed advantage, depending on the type of
    model, if you decide not to specify an input power.

*/

#[derive(Debug, Clone, Copy)]
pub struct LedPar {
    pub peak_wavelength: f64,
    pub fwhm: f64, // full width at half maximum
}

impl Default for LedPar {
    fn default() -> Self {
        Self {
            peak_wavelength: 550.0,
            fwhm: 50.0,
        }
    }
}

/**
Ohno LED Model

LED Model as published in "Spectral design considerations for white LED color rendering", Yoshi Ohno, in Optical
Engineering 44(11), 111302 (November 2005).
 */
#[derive(Debug)]
pub struct LedOhno2005 {
    pub parameters: Vec<LedPar>,
    pub domain: Domain<WavelengthStep>,
}

impl LedOhno2005 {
    pub fn new(lp: impl Into<Vec<LedPar>>) -> Self {
        Self {
            parameters: lp.into(),
            ..Default::default()
        }
    }

    pub fn set_domain(mut self, domain: Domain<WavelengthStep>) -> Self {
        self.domain = domain;
        self
    }

    pub fn keys(&self) -> Option<Vec<String>> {
        let mut v: Vec<String> = Vec::with_capacity(self.shape().1);
        for LedPar {
            peak_wavelength: center,
            fwhm,
        } in &self.parameters
        {
            v.push(format!("Ohno LED Model {:.1}/{:.1}", center, fwhm));
        }
        Some(v)
    }
}

impl Default for LedOhno2005 {
    fn default() -> Self {
        Self {
            parameters: vec![LedPar::default()],
            ..Default::default()
        }
    }
}

impl<T: ToPrimitive> From<[T; 2]> for LedOhno2005 {
    fn from([c, w]: [T; 2]) -> Self {
        let mut peak_wavelength = c.to_f64().unwrap();
        let mut fwhm = w.to_f64().unwrap();
        if peak_wavelength > 1.0 {
            // assume parameters are in nm
            peak_wavelength *= 1E-9;
            fwhm *= 1E-9;
        }
        let uval = NM.unitvalue(1).value();
        LedOhno2005 {
            parameters: vec![LedPar {
                peak_wavelength,
                fwhm,
            }],
            domain: Domain::new(
                ((peak_wavelength - 3.0 * fwhm) / uval) as i32,
                ((peak_wavelength + 3.0 * fwhm) / uval) as i32,
                NM,
            ),
        }
    }
}

#[test]
pub fn test_from_array() {
    use crate::models::CieYxy;

    let led = LedOhno2005::from([630, 25]);
    println!("{:?}", led);
    let y_xy: CieYxy = led.into();
    println!("{}", y_xy);
}

impl SpectralDistribution for LedOhno2005 {
    type StepType = WavelengthStep;
    type MatrixType = OMatrix<f64, Dynamic, Dynamic>;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (self.domain.clone(), self.map_domain(self.domain.clone()))
    }

    fn shape(&self) -> (usize, usize) {
        (self.domain.len(), self.parameters.len())
    }

    fn map_domain<S2: Step>(&self, dto: Domain<S2>) -> OMatrix<f64, Dynamic, Dynamic>
    where
        <<Self as SpectralDistribution>::StepType as Step>::UnitValueType:
            From<<S2 as Step>::UnitValueType>,
    {
        let m = Self::MatrixType::from_iterator(
            dto.len(),
            self.shape().1,
            self.parameters.iter().flat_map(|lp| {
                dto.iter()
                    .map(move |l| led_ohno(l.value(), lp.peak_wavelength, lp.fwhm))
            }),
        );
        m
    }

    fn description(&self) -> Option<String> {
        Some("Ohno 2005 LED model spectra ".to_string())
    }

    fn xyz<C>(&self) -> CieXYZ<C>
    where
        C: StandardObserver,
    {
        let xyz =
            C::cmf() * self.map_domain(C::domain()) * C::K * C::domain().step.unitvalue(1).value();
        CieXYZ::<C>::new(xyz).normalize(100.0)
    }
}

impl<C: StandardObserver> From<LedOhno2005> for CieXYZ<C> {
    fn from(l: LedOhno2005) -> Self {
        l.xyz()
    }
}
