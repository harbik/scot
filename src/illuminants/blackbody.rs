

use nalgebra::DMatrix;

use crate::spectra::{Illuminant, SpectralData};
use crate::illuminants::cct::{CCTs};
use crate::util::domain::Domain;
use crate::util::physics::planck_cie as planck;
use crate::util::units::{Meter, WavelengthScale, Scale, Unit};


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
	use colorado::illuminants::Blackbody;
	use colorado::observers::Cie1931;
	use colorado::cie::XYZ;
	use approx::assert_abs_diff_eq;

	let bb = Blackbody::new(3000);
	let xyz = XYZ::<Cie1931>::from(bb);
	```

	# Examples
	Y
	A blackbody radiator, with a temperature of 3000K, and an illuminance of 0.1W/m<sup>2</sup>.
	Here a single integer valued argument is used to specify a blackbody's temperature.

	```
	use colorado::illuminants::Blackbody;
	use colorado::observers::Cie1931;
	use colorado::util::domain::Domain;
	use colorado::spectra::SpectralDistribution;
	use colorado::cie::XYZ;
	use colorado::util::units::DEV; // dEv 
	use approx::assert_abs_diff_eq;

	let sdbb = Blackbody::new([[6500.0,0.1]]);
	let v = sdbb.values(Domain::new(15, 33, DEV)); // values for blackbody radiator from 1.5 (826.56nm) to 3.3 eV (375.709)
	let val : Vec<f64> = v.into_iter().cloned().collect();
	assert_eq!(val, vec![]);
	```
 */

#[derive(Debug)]
pub struct Blackbody {
	pub ccts: CCTs,
}

impl Illuminant for Blackbody {}

impl Blackbody {

	pub fn new(parameters: impl Into<CCTs>) -> Blackbody
	{
		Blackbody {
			ccts: parameters.into(),
		}
	}
}

impl SpectralData for Blackbody {

	type ScaleType = WavelengthScale;

	/**
		Blackbody Spectral values for multiple domain types.

		Calculates Blackbody spectral values for a target domain.
		This `UnitValue` item type of target domain's Unit doesn't have to be a `Meter` value, but needs to be
		able to be converted into a `Meter` value, typically done by implementing a `From<X> for Meter` trait.
	 */
	fn values<L: Scale>(&self, dom: Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<<Self as SpectralData>::ScaleType as Scale>::UnitType: From<<L>::UnitType>
	 {
		let mut v : Vec<f64> = Vec::with_capacity(self.ccts.len() * dom.len());
		for (t,p) in &self.ccts {
			for i in dom.range.clone() {
				let meter_value: Meter = dom.scale.unit(i).into();
				v.push(planck(meter_value.value(), t, p));
			}
		}
		DMatrix::from_vec(dom.len(), self.ccts.len(), v)

	}

	fn description(&self) -> Option<String> {
		Some("Blackbody Sources".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		self.ccts.keys()
	}

	/// Domain which covering the visible part of the spectrum
	fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::default()
		}
	
}




#[test]
fn test_blackbody(){
	use crate::observers::{Cie1931};
	use crate::cie::{Yxy};
	use crate::util::physics::{C2, C2_CIE};

	let bb_yxy = Yxy::<Cie1931>::from(Blackbody::new([2855.0, 3000.0]));
	println!("{}", bb_yxy);
	println!("{:?} {:?}", C2, C2_CIE);
}
