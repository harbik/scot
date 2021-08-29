

use approx::assert_abs_diff_eq;
use nalgebra::{DMatrix, DVector, Vector, Vector3};

use crate::models::uv60;
use crate::observers::StandardObserver;
use crate::{C2_IPTS_1948, C2_NBS_1931, NM, NM5, SpectralData, planck_c2, planck_prime_c2};
use crate::illuminants::{Illuminant};
use crate::illuminants::cct_parameters::{CctParameters};
use crate::util::{Domain, planck, Meter, Step, Unit, WavelengthStep, };



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
	use colorado::observers::CieObs1931;
	use colorado::cie::XYZ;
	use approx::assert_abs_diff_eq;

	let pl = Planckian::new(3000);
	let xyz = XYZ::<CieObs1931>::from(pl);
	```

	# Examples
	
	A blackbody radiator, with a temperature of 3000K, and an illuminance of 0.1W/m<sup>2</sup>.
	Here a single integer valued argument is used to specify a blackbody's temperature.

	```
	use colorado::illuminants::Planckian;
	use colorado::observers::CieObs1931;
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
	pub ccts: CctParameters,
}

impl Planckian {

	pub fn new(parameters: impl Into<CctParameters>) -> Planckian
	{
		Planckian {
			ccts: parameters.into(),
		}
	}
}

impl SpectralData for Planckian {

	type StepType = WavelengthStep;

	/**
		Planckian Spectral values for multiple domain types.

		Calculates planckian spectral values for a target domain.
		This `UnitValue` item type of target domain's Unit doesn't have to be a `Meter` value, but needs to be
		able to be converted into a `Meter` value, typically done by implementing a `From<X> for Meter` trait.
	 */
	fn values<L: Step>(&self, dom: &Domain<L>) -> DMatrix<f64>
	where
		L: Step,
		<<Self as SpectralData>::StepType as Step>::UnitValueType: From<<L>::UnitValueType>
	 {
		let mut v : Vec<f64> = Vec::with_capacity(self.ccts.len() * dom.len());
		for (t,p) in &self.ccts {
			for i in dom.range.clone() {
				let meter_value: Meter = dom.step.unitvalue(i).into();
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
	fn domain(&self) -> Domain<Self::StepType> {
		Domain::default()
	}
	
}
pub struct BB <const T: usize>;

impl<const T: usize> Default for BB<T> {
	fn default() -> Self {
		Self	
	}
}

impl<const N: usize> Illuminant for BB<N> {}

impl<const N: usize> SpectralData for BB<N> {
	type StepType = WavelengthStep;

	fn values<L: Step>(&self, dom: &Domain<L>) -> DMatrix<f64>
	where
		L: Step,
		<<Self as SpectralData>::StepType as Step>::UnitValueType: From<<L>::UnitValueType>
	 {
		 Planckian::new(N).values(dom)
	 }
	
	fn domain(&self) -> Domain<Self::StepType> {
		Domain::default()
	}
}

pub fn planck_xyz<C:StandardObserver>(t: f64, c2: f64) -> [f64;3] {
	let d = C::default().domain();
	let n = d.len();
	let cmf = C::default().cmf(&d);
	let pl = DVector::<f64>::from_iterator(n, (&d).into_iter().map(|p|planck_c2(p.value(),t, c2)));
	let xyz = cmf * pl;
	[xyz.x, xyz.y, xyz.z]
}

pub fn planck_xyz_dxyz<C:StandardObserver>(t: f64, c2: f64) -> [[f64;3];2]{
//	let d = Domain::new(380, 780, NM);
//	let d = Domain::new(360/5, 830/5, NM5);
	let d = C::default().domain();
	let n = d.len();
	let cmf = C::default().cmf(&d);
	let pl = DVector::<f64>::from_iterator(n, (&d).into_iter().map(|p|planck_c2(p.value(),t, c2)));
	let pl_prime = DVector::<f64>::from_iterator(n, d.into_iter().map(|p|planck_prime_c2(p.value(),t, c2)));
	let xyz = &cmf * pl;
	let dxyz = cmf * pl_prime;
	[[xyz.x, xyz.y, xyz.z], [dxyz.x, dxyz.y, dxyz.z]]
}

pub fn planck_du_dv<C:StandardObserver>(t: f64, c2: f64) -> [f64;3] {
	let [[x,y, z], [xp, yp, zp]] = planck_xyz_dxyz::<C>(t, c2);
	let den = x + 15.0 * y + 3.0 * z;
	let denp = xp + 15.0 * yp + 3.0 * zp;
	let du = (4.0 * xp * den - 4.0 * x * denp) / den.powi(2);
	let dv = (6.0 * yp * den - 6.0 * y * denp) / den.powi(2);
	let [_, u,v] = uv60(x, y, z);
	[u,v,-du/dv]
}

#[test]
/**
	Robertson's (Robertson \[1968\]A) "Computation of Correlated Color Temperature" Table II,
	"Thirty one isotemperature lines".
	This uses the original CIE 1931 color matching functions, spaced at 5nm intervals and defined over a
	range from 360 to 830 nm, as represented with `Cie1931Classic` standard observer, and uses an older value
	for the second radiometric constant.
	Current practice is to use a range from 380 to 780nm, and with 1 nm steps.
*/
fn test_planck_robertson_table(){
	use crate::observers::CieObs1931Classic;
	let [u,v,t] = planck_du_dv::<CieObs1931Classic>(1_000_000_000.0, C2_IPTS_1948);
	assert_abs_diff_eq!(u, 0.180_06, epsilon = 0.000_005);
	assert_abs_diff_eq!(v, 0.263_52, epsilon = 0.000_005);
	assert_abs_diff_eq!(t, -0.243_4, epsilon = 0.000_05);

	let [u,v,t] = planck_du_dv::<CieObs1931Classic>(5_000.0, C2_IPTS_1948);
	assert_abs_diff_eq!(u, 0.211_40, epsilon = 0.000_005);
	assert_abs_diff_eq!(v, 0.323_09, epsilon = 0.000_005);
	assert_abs_diff_eq!(t, -1.017, epsilon = 0.000_5);

	let [u,v,t] = planck_du_dv::<CieObs1931Classic>(2_000.0, C2_IPTS_1948);
	assert_abs_diff_eq!(u, 0.304_96, epsilon = 0.000_005);
	assert_abs_diff_eq!(v, 0.359_06, epsilon = 0.000_005);
	assert_abs_diff_eq!(t, -11.29, epsilon = 0.005);

	// vertical iso-temperature line
	println!("{:?}", planck_du_dv::<CieObs1931Classic>(1_624.911_5, C2_IPTS_1948));
}
