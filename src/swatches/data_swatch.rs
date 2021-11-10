use nalgebra::DVector;

use crate::{Domain, Meter, SpectralDistribution, Step, WavelengthStep, illuminants::Illuminant, models::cielab::CieLab, observers::StandardObserver};

use super::Swatch;

#[derive(Debug)]
pub struct DataSwatch {
    values: Vec<f64>,
    domain: Domain<WavelengthStep>,
}

impl DataSwatch {
    pub fn new(domain: Domain<WavelengthStep>, values: Vec<f64>) -> Self { 
        assert!(domain.len() == values.len());
        Self { 
            domain,
            values,
         } 
    }
}

impl SpectralDistribution for DataSwatch {
    type MatrixType = DVector<f64>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (self.domain.clone(), Self::MatrixType::from_vec(self.values.clone()))
    }

    fn shape(&self) -> (usize, usize) {
        (self.domain.len(), 1)
    }
}

impl Swatch for DataSwatch {}

impl<I, C> From<DataSwatch> for CieLab<I,C> 
  where 
    <<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<Meter>, 
    I: Illuminant, 
    C: StandardObserver 
  {
    fn from(sw: DataSwatch) -> Self {
        sw.lab()
    }
}


#[test]
fn test_spline_swatch(){
    use crate::models::CieLab;
    let sc5y6_6: SplineCurve<3,1> = SplineCurve::new(vec![380.0, 380.0, 380.0, 380.0, 407.0, 433.0, 485.0, 512.0, 525.0, 538.0, 590.0, 800.0, 800.0, 800.0, 800.0], vec![0.06820, 0.06820, 0.09133, 0.08235, 0.08207, 0.10139, 0.25423, 0.30611, 0.29466, 0.28540, 0.28540]);
    let lab: CieLab<crate::illuminants::CieIllC, crate::observers::CieObs1931> = sc5y6_6.into();
    println!("{}",lab.data);
}