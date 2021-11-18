use nalgebra::DVector;
use spliny::SplineCurve;

use crate::{Domain, Meter, SpectralDistribution, Step, Unit, WavelengthStep, illuminants::Illuminant, models::cielab::CieLab, observers::StandardObserver};

use super::Swatch;

#[derive(Debug)]
pub struct SplineSwatch<const K: usize> {
    spline_curve: SplineCurve<K,1>,
    domain: Domain<WavelengthStep>,
}

impl< const K: usize> SplineSwatch<K> {
    /// Construct a `SplineSwatch` from a 1D `SplineCurve`.
    /// 
    /// The spline curve should represent the reflection spectrum of a swatch,
    /// normalized to a reference white reflection value of 1.0. as function of 
    /// an array of spline knot wavelength values.
    pub fn new(spline_curve:  SplineCurve<K, 1>) -> Self { 
        Self { 
            spline_curve,
            domain: Domain::default()
         } 
    }

    pub fn domain(mut self, domain: Domain<WavelengthStep>) -> Self {
        self.domain = domain;
        self
    }
}

impl<const K: usize> SpectralDistribution for SplineSwatch<K> {
    type MatrixType = DVector<f64>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        let n = self.domain.len();
        let w: Vec<f64> = self.domain.iter().map(|v|v.value()).collect();
        let v = self.spline_curve.evaluate(&w).unwrap_or(vec![f64::NAN;n]);
        (self.domain.clone(), Self::MatrixType::from_vec(v))
    }

    fn shape(&self) -> (usize, usize) {
        (self.domain.len(), 1)
    }
}

impl<const K:usize> Swatch for SplineSwatch<K> {}

impl<I, C, const K: usize> From<SplineSwatch<K>> for CieLab<I,C> 
  where 
    <<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<Meter>, 
    I: Illuminant, 
    C: StandardObserver 
  {
    fn from(sw: SplineSwatch<K>) -> Self {
        sw.lab()
    }
}

impl <const K: usize> From<SplineCurve<K,1>> for SplineSwatch<K> {
    fn from(sc:  SplineCurve<K,1>) -> Self {
        SplineSwatch::new(sc)
    }
}

impl<I, C, const K: usize> From<SplineCurve<K,1>> for CieLab<I,C> 
  where 
    <<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<Meter>, 
    I: Illuminant, 
    C: StandardObserver 
  {
    fn from(sc: SplineCurve<K,1>) -> Self {
        let sw: SplineSwatch<K> = sc.into();
        sw.lab()
    }
}


#[test]
fn test_spline_swatch(){
    use crate::models::CieLab;
    let sc5y6_6: SplineCurve<3,1> = SplineCurve::new(vec![380E-9, 380E-9, 380E-9, 380E-9, 407E-9, 433E-9, 485E-9, 512E-9, 525E-9, 538E-9, 590E-9, 800E-9, 800E-9, 800E-9, 800E-9], vec![0.06820, 0.06820, 0.09133, 0.08235, 0.08207, 0.10139, 0.25423, 0.30611, 0.29466, 0.28540, 0.28540]);
    let lab: CieLab<crate::illuminants::CieIllC, crate::observers::CieObs1931> = sc5y6_6.into();
    println!("{}",lab.data);
}