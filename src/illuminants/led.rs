

use nalgebra::DMatrix;
use num::ToPrimitive;

use crate::spectra::{SpectralData};
use crate::util::{Meter, Step, Unit, WavelengthStep, led_ohno, simpson, Domain};



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
pub struct LedParameters {
	pub peak: f64,
	pub fwhm: f64,
	pub power: f64,
}

/**
Ohno LED Model

LED Model as published in "Spectral design considerations for white LED color rendering", Yoshi Ohno, in Optical
Engineering 44(11), 111302 (November 2005).
 */
pub struct LedOhno2005(pub Vec<LedParameters>);

impl<T: ToPrimitive> From<[T;2]> for LedOhno2005 {
    fn from([c,w]: [T;2]) -> Self {
		let mut peak = c.to_f64().unwrap();	
		let mut fwhm = w.to_f64().unwrap();
		if peak>1.0 {  // assume parameters are in nm
			peak *= 1E-9;
			fwhm *= 1E-9;
		}
        LedOhno2005 (
			vec![
				LedParameters{peak, fwhm, power: 0.0}
			]
		)
    }
}

#[test]
pub fn test_from_array(){
	use crate::models::CieYxy;

	let led = LedOhno2005::from([630,25]);
	let y_xy : CieYxy =  led.into();
	println!("{}", y_xy);
}


impl LedOhno2005 {

	pub fn new(parameters: impl Into<Vec<LedParameters>>) -> Self
	{
		Self(parameters.into()) 
	}
}

impl SpectralData for LedOhno2005 {

	type StepType = WavelengthStep;

	/**
		Spectral values for Ohno Model LEDs.

	 */
	fn values<L: Step>(&self, dom: &Domain<L>) -> DMatrix<f64>
	where
		L: Step,
		<<Self as SpectralData>::StepType as Step>::UnitValueType: From<<L>::UnitValueType>
	 {
		let mut v : Vec<f64> = Vec::with_capacity(self.0.len() * dom.len());
		for &LedParameters{peak, fwhm, power} in self.0.iter() {
			let scale = if power>0.0 {
				power / simpson(|l|led_ohno(l,peak, fwhm), peak - 3.0 * fwhm, peak + 3.0 * fwhm, 100)
			} else {
				1.0
			};
			for i in dom.range.clone() {
				let meter_value: Meter = dom.scale.unitvalue(i).into();
				v.push(scale * led_ohno(meter_value.value(), peak, fwhm));
			}
		}
		DMatrix::from_vec(dom.len(), self.0.len(), v)

	}

	fn description(&self) -> Option<String> {
		Some("Ohno 2005 LED model spectra ".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		let mut v: Vec<String> = Vec::with_capacity(self.0.len());
		for LedParameters { peak: center , fwhm, power: _ } in &self.0 {
			v.push(format!("Ohno LED Model {:.1}/{:.1}",center, fwhm));
		}
		Some(v)
	}

	/// Domain covering the visible part of the spectrum
	fn domain(&self) -> Domain<Self::StepType> {
		Domain::default()
	}
	
}

