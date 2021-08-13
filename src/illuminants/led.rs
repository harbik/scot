

use nalgebra::{ArrayStorage, DMatrix, SMatrix, SVectorSlice};

use crate::ALL;
use crate::spectra::{SpectralData};
use crate::illuminants::{Illuminant};
use crate::illuminants::cct::{CCTs};
use crate::util::domain::Domain;
use crate::util::physics::planck;
use crate::util::{Meter, NM, Step, Unit, WavelengthStep, sprague_cols};

use super::led_data::{LED_IES_DATA, LED_IES_KEYS};


/**
	Spectral distributions of one or multiple generic blackbody illuminants.
	
	Each of the blackbody sources is characterized by a temperature, in units of Kelvin, and radiant exitance
	with unit W/m<sup>2</sup>. Through a `CCTs` helper class, it accepts multiple ways to specify the 
	temperatures and exitance you want &mdash; see this class for examples.

	The spectral power distribution for blackbody radiators is calculated using Planck's law.
	The `values` method of the `SpectralDistribution` trait produces spectral radiant exitance values
	over the range of the input domain, and at equidistant spacing. Besides the usual wavelength domains,
	you can also use other domains with units which implement the Wavelength trait
	

	
	

	# Examples
	A blackbody radiator, with a temperature of 3000K, and a irradiance of 1W/m<sup>2</sup>.
	Here a single integer valued argument is used to specify a blackbody's temperature.

	```
	use colorado::illuminants::Planckian;
	use colorado::observers::Cie1931;
	use colorado::cie::XYZ;
	use approx::assert_abs_diff_eq;

	let pl = Planckian::new(3000);
	let xyz = XYZ::<Cie1931>::from(pl);
	```

	# Examples
	Y
	A blackbody radiator, with a temperature of 3000K, and an illuminance of 0.1W/m<sup>2</sup>.
	Here a single integer valued argument is used to specify a blackbody's temperature.

	```
	use colorado::illuminants::Planckian;
	use colorado::observers::Cie1931;
	use colorado::util::domain::Domain;
	use colorado::spectra::SpectralDistribution;
	use colorado::cie::XYZ;
	use colorado::util::units::DEV; // dEv 
	use approx::assert_abs_diff_eq;

	let sd = Planckian::new([[6500.0,0.1]]);
	let v = sd.values(Domain::new(15, 33, DEV)); // values for Planckian radiator from 1.5 (826.56nm) to 3.3 eV (375.709)
	let val : Vec<f64> = v.into_iter().cloned().collect();
	assert_eq!(val, vec![]);
	```
 */

#[derive(Debug, Clone, Copy)]
pub struct LedModelParameters {
	pub center: f64,
	pub fwhm: f64,
	pub power: f64,
}

pub struct LedOhnoModel(Vec<LedModelParameters>);


impl LedOhnoModel {

	pub fn new(parameters: impl Into<Vec<LedModelParameters>>) -> Self
	{
		Self(parameters.into()) 
	}
}

impl SpectralData for LedOhnoModel {

	type ScaleType = WavelengthStep;

	/**
		Spectral values for Ohno Model LEDs.

	 */
	fn values<L: Step>(&self, dom: &Domain<L>) -> DMatrix<f64>
	where
		L: Step,
		<<Self as SpectralData>::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType>
	 {
		let mut v : Vec<f64> = Vec::with_capacity(self.0.len() * dom.len());
		for LedModelParameters{center, fwhm, power} in &self.0 {
			for i in dom.range {
				let meter_value: Meter = dom.scale.unitvalue(i).into();
				v.push(ohno(meter_value.value(), center, fwhm, power));
			}
		}
		DMatrix::from_vec(dom.len(), self.0.len(), v)

	}

	fn description(&self) -> Option<String> {
		Some("Planckian Sources".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		let v: Vec<String> = Vec::with_capacity(self.0.len());
		for LedModelParameters { center , fwhm, power: _ } in self.0 {
			v.push(format!("Ohno LED Model {:.1}/{:.1}",center, fwhm));
		}
		Some(v)
	}

	/// Domain covering the visible part of the spectrum
	fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::default()
	}
	
}


#[derive(Debug, Default)]
pub struct IesTm30Led<const I:usize>;

impl<const I:usize> SpectralData for IesTm30Led<I> {
    type ScaleType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Step,
		<Self::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(LED_IES_DATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=14 => {
				let data = SVectorSlice::<f64, 401>::from_slice(&LED_IES_DATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in IES LED Data")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380, 780, NM)
    }

	fn keys(&self) -> Option<Vec<String>> {
		Some(LED_IES_KEYS.iter().map(|s| s.to_string()).collect())
	

	}


	fn description(&self) -> Option<String> {
		Some("IES TM30 Commercial LED Spectra".to_string())
	}
}