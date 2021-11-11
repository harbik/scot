/*!

Munsell Color System

From [Paul Centore]'s article:

> Albert Munsell originally defined the Munsell system conceptually. A colour is specified by its hue, value, and chroma. Hue is notated by a number between 0 and 10,
> which prefixes one of ten hue names: red (R), yellow-red (YR), yellow (Y), greenyellow (GY), green (G), blue-green (BG), blue (B), purple-blue (PB), purple (P),
> and red-purple (RP). There are a total of 100 hues with integer prefixes. Value, indicating how light a colour is, is a number between 0 (signifying black) and 10 (white).
> Chroma extends from 0 (grey) to a positive number, which increases to a varying
> perceptual limit as a colour’s difference from a grey, of the same Munsell hue and
> value, increases. The 100 hues with integer prefixes are evenly spaced perceptually,
> as are values and chromas. In addition to Munsell’s abstract definition, the 1929
> Munsell Book of Color contained physical exemplifications of Munsell specifications.
> This book became a physical standard for the system.

[Paul Centore]:  https://www.munsellcolourscienceforpainters.com/ColourSciencePapers/OpenSourceInverseRenotationArticle.pdf "An Open-Source Inversion Algorithm for the Munsell Renotation"

 */

 #![allow(clippy::approx_constant)]

use colorado::{Domain, Meter, SpectralDistribution, Step, Unit, WavelengthStep, illuminants::Illuminant, models::CieLab, observers::StandardObserver, swatches::Swatch};
use nalgebra::DMatrix;
use matt_splines::MUNSELL_MATT;


pub mod gloss;
pub use gloss::*;

pub mod matt;
pub use matt::*;

pub mod matt_splines;
//pub use matt_splines::*;

pub mod renotation;
pub use renotation::*;


#[derive(Debug,Default)]
pub struct MunsellMattSpline {
    pub keys: Vec<String>,
    pub domain: Domain<WavelengthStep>,
}

impl MunsellMattSpline {
    pub fn new(hvl: impl IntoIterator<Item = impl AsRef<str>>) -> Self { 
        let keys: Vec<String> = hvl.into_iter().map(|s| s.as_ref().to_string()).collect();
        Self { 
            keys,
            ..Default::default()
         } 
    }
}

impl SpectralDistribution for MunsellMattSpline {
    type MatrixType = DMatrix<f64>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        let n = self.domain.len();
        let w: Vec<f64> = self.domain.iter().map(|v|v.value()*1E9).collect();
        let mut v:Vec<f64> = Vec::with_capacity(w.len() * self.keys.len());
        for k in self.keys.iter() {
            let spd = MUNSELL_MATT.evaluate(&k.as_str(), &w).unwrap_or_else(|_|vec![f64::NAN; n]);
            v.extend(spd);
        }
        (self.domain.clone(), DMatrix::from_vec(w.len(), self.keys.len(), v))
    }

    fn shape(&self) -> (usize, usize) {
        (self.domain.len(), self.keys.len())
    }
}

impl colorado::swatches::Swatch for MunsellMattSpline {}

impl<I, C> From<MunsellMattSpline> for CieLab<I,C> 
  where 
    <<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<Meter>, 
    I: Illuminant, 
    C: StandardObserver 
  {
    fn from(mms: MunsellMattSpline) -> Self {
        mms.lab()
    }
}


#[test]
fn test_munsell_matt(){
    use colorado::models::CieLab;
    let m = MunsellMattSpline::new(["7.5RP5/12", "5Y6/6", "2.5BG9/2"]);
    let (_dom, spd) = m.spd();
    println!("{}", spd);
    let lab: CieLab<colorado::illuminants::CieIllC, colorado::observers::CieObs1931> = CieLab::from(m);
    println!("{}",lab.data);
}
 