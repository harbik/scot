

use nalgebra::DMatrix;

use crate::spectra::{SpectralData};
use crate::illuminants::{Illuminant};
use crate::illuminants::cct::{CCTs};
use crate::util::domain::Domain;
use crate::util::physics::planck;
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

#[derive(Debug)]
pub struct Planckian {
	pub ccts: CCTs,
}

impl Planckian {

	pub fn new(parameters: impl Into<CCTs>) -> Planckian
	{
		Planckian {
			ccts: parameters.into(),
		}
	}
}

impl SpectralData for Planckian {

	type ScaleType = WavelengthScale;

	/**
		Planckian Spectral values for multiple domain types.

		Calculates planckian spectral values for a target domain.
		This `UnitValue` item type of target domain's Unit doesn't have to be a `Meter` value, but needs to be
		able to be converted into a `Meter` value, typically done by implementing a `From<X> for Meter` trait.
	 */
	fn values<L: Scale>(&self, dom: &Domain<L>) -> DMatrix<f64>
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
		Some("Planckian Sources".to_string())
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

/**
	Generic Blackbody Rust type, with a single constant usize typed temperature.

	This type is used as a'tag' to represent the illuminant used for chromaticity calculations of color swatches,
	and use as reference white illuminant of various colorspaces, such as the CIELAB color spaces.
	
	# Examples

	Calculate the CIELAB coordinates of the 'Color checker', under a blackbody illuminant with a correlated color temperature of 2700K:
	```
		use colorado::cie::{self, Lab};
		use colorado::incandescent::BB;
		use colorado::observers::Cie1931;
		use colorado::swatches::ColorChecker;
		let checker_lab_bb2700: cie::<Lab<Cie1931,BB<2700>>> = swatches::ColorChecker::default().into();
		println!("{}", checker_lab-bb2700);
	```
	the same can also be calculated as:
	```
//		let checker_lab_bb2700  = cie::<Lab<Cie1931,BB<2700>>::from(swatches::ColorChecker::default());
	```
	And similar, to get the Colorchecker's CIELAB coordinates for a D65 illuminant
	```
//		let checker_lab: cie::<Lab<Cie1931,D65> = swatches::ColorChecker::default().into();
	```
	Both of these use the illuminant spectra to calculate the actual colors.


	Conversions between CIELAB coordinates with different illuminants, without using the Colorchecker reflectance spectra,
	is possible too, by using a chromatic adaptation transformation. By default the library uses the CIECAM02's 
	CIECAT02 chromatic adaptation model.

	```
//		use colorado::cie;
//		use colorado::observers::Cie1931;
//		use colorado::illuminants::D65;
//		let checker_lab = cie::<Lab<Cie1931,D65>>.from(checker_lab_bb2700);
	```
	although we could have calculated directly too:
	And here is a check to confirm we get the same results:
	```
		let checker_lab_bb2700  = cie::<Lab<Cie1931,BB<2700>>::from(swatches::ColorChecker::default());

		// Use chromatic adaptation to change the from a BB<2700> to a D65 illuminant:
		let checker_lab1 = cie::<Lab<Cie1931,D65>::from(checker_lab_bb2700);
	//	let checker_lab1 = cie::<Lab<Cie1931,D65>::from((checker_lab_bb2700, CAT::VonKries));

		let checker_lab2: cie::<Lab<Cie1931,D65> = swatches::ColorChecker::default().into();

		
	```
	You chan see that the chromatic adaptation transformation model is not perfect, and introduces significant errors,
	but it is the only option






	
	calculate chromaticity coordinatesin color spaces using a reference white, such as in the CIELAB color space,
	or to calculate color coordinates .
	Typically `D65` or `D50` in this color space, but using this type, also any blackbody illuminant can also be used. 


 */
struct BB <const T: usize>;

impl<const N: usize> Default for BB<N> {
	fn default() -> Self {
		Self	
	}
}

impl<const N: usize> Illuminant for BB<N> {}

impl<const N: usize> SpectralData for BB<N> {
	type ScaleType = WavelengthScale;

	fn values<L: Scale>(&self, dom: &Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<<Self as SpectralData>::ScaleType as Scale>::UnitType: From<<L>::UnitType>
	 {
		 Planckian::new(N * 100).values(dom)
	 }
	
	/// Domain which covering the visible part of the spectrum
	fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::default()
	}
}

#[test]
fn test_planckian(){
	use crate::observers::{Cie1931};
	use crate::models::{CieYxy};

	let pl_yxy = CieYxy::<Cie1931>::from(Planckian::new([2855.0, 3000.0]));
	println!("{}", pl_yxy);
}
