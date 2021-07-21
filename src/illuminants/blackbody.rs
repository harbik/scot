
use nalgebra::DMatrix;

use crate::cie::XYZ;
use crate::observers::StandardObserver;
use crate::spectra::{Illuminant, SpectralDistribution};
use crate::illuminants::cct::{CCTs};
//use crate::util::physics::planck;
use crate::util::domain::Domain;
use crate::util::physics::planck;
use crate::util::units::{Meter, MeterScale, Scale, Unit};
//use std::iter::ExactSizeIterator;


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

struct BlackbodySpectra{
	blackbody: Blackbody
}

impl BlackbodySpectra {
	pub fn new(parameters: impl Into<CCTs>) -> Self {
		Self{ 
			blackbody: Blackbody {
				ccts: parameters.into()
			}
		}
	}

}

impl SpectralDistribution for BlackbodySpectra {

	type UnitType = MeterScale;
	type UnitValue = Meter;

	/**
		Blackbody Spectral values for multiple domain types.

		Calculates Blackbody spectral values for a target domain.
		This `UnitValue` item type of target domain's Unit doesn't have to be a `Meter` value, but needs to be
		able to be converted into a `Meter` value, typically done by implementing a `From<X> for Meter` trait.
	 */
	fn values<L: Scale>(&self, dom: Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		Self::UnitValue: From<<L>::ValueType>
	 {
		let mut v : Vec<f64> = Vec::with_capacity(self.blackbody.ccts.len() * dom.len());
		for (t,p) in &self.blackbody.ccts {
			for i in dom.range.clone() {
				let meter_value: Meter = dom.scale.unit(i).into();
				v.push(planck(meter_value.value(), t, p));
			}
		}
		DMatrix::from_vec(dom.len(), self.blackbody.ccts.len(), v)

	}

	fn description(&self) -> Option<String> {
		Some("Blackbody Sources".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		self.blackbody.ccts.keys()
	}

	/// Domain which covers the total emission for all the radiators.
	fn domain(&self) -> Domain<Self::UnitType> {
		//SpectralDomain::default()
		todo!()
		}
	
}


/*

	Calculate XYZ values for Blackbody radiators.
	
	Uses the observer's domain instead of the default Blackbody
	domain, which is very broad. This would be the default option,
	if this is was not overridden here.
	
//CONFLICTING WITH GENERIC From<... SpectralDistribution> for XYZ<C>
	*/

impl<C:StandardObserver> From<Blackbody> for XYZ<C> {
	fn from(bb: Blackbody) -> Self {
		XYZ::<C> {
			xyz: C::global().cmf(C::global().domain()) * bb.values(C::global().domain()),
			white: None,
			cmf: C::global()
		}
		
	}
}


#[test]
fn test_blackbody(){
	use crate::observers::{Cie1931};
	use crate::cie::XYZ;

	let bb = XYZ::<Cie1931>::from(Blackbody::new(3000));
	println!("{}",bb);
}
