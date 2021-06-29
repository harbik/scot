
use nalgebra::DMatrix;

use crate::spectra::{Illuminant, SpectralDistribution, SpectralDomain};
use crate::cct::{CCTs};

/// Stefan Boltzmann law, describing a blackbody radiant emittance (W m<sup>-2</sup>), as function of its absolute
/// temperature (K).
pub fn stefan_boltzmann(temp_kelvin: f64) -> f64 {

	SIGMA * temp_kelvin.powi(4)
}
/// Stefan-Boltzmann constant (W m<sup>-2</sup> K<sup>-4</sup>)
pub const SIGMA: f64 = 5.670_374_419_184E-8;

/**
Calculates spectral radiant exitance (W m<sup>-2</sup> m<sup>-1</sup> or W m<sup>-3</sup>), as function of input parameters wavelength (m), absolute
temperature (K), and radiant exitance (W m<sup>2</sup>). Integrated over the full spectral domain this function results in Stefan Boltzmann's law,
included here as a scaling factor to set the output power scale to a specified radiant power as input. 
*/
pub fn planck(wl: f64, temp: f64, pow: f64) -> f64 {
	pow / stefan_boltzmann(temp) * C1 / wl.powi(5) / ((C2 / (wl * temp)).exp() - 1.0)
}

/// The speed of light (m/s)
pub const C: f64 = 299792458.0; 

/// Boltzmann constant (m<sup>2</sup> kg s<sup>-2</sup> K<sup>-1</sup>)
pub const KB: f64 = 1.3806485279E-23; 

/// Planck constant (m<sup>2</sup> kg / s)
pub const H: f64 = 6.6260700408181E-34; 

/// First radiation constant (W m<sup>2</sup>)
pub const C1: f64 = 2. * std::f64::consts::PI * H * C * C; 

/// Second radiation constant (m K)
pub const C2: f64 = H * C / KB;


/**
	Representation of one or multiple generic blackbody illuminants. Can be constructed with various parameters
	as illustrated in the examples below.

	# Examples
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

impl SpectralDistribution for Blackbody {

	fn spectra(&self, dom: SpectralDomain) -> DMatrix<f64> {
		DMatrix::from_fn(dom.size, self.ccts.0.nrows(),
		|r,c| { 
			let t = self.ccts.0[(c,0)];
			let p = self.ccts.0[(c,1)];
			let l = ((r + dom.low) * dom.unit) as f64 * 1E-10; // 1 Angstrom is 1E-10 m.
			planck(l, t, p)
		}
	 )

	}

	fn description(&self) -> Option<String> {
		Some("Blackbody Sources".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		self.ccts.keys()
	}

	fn domain(&self) -> SpectralDomain {
			todo!()
		}
}



#[test]
fn test_blackbody(){
	/*
	use crate::observer::{cie1931};

	let bb3000 = Blackbody::new([2700.0, 3000.0, 4000.0, 5000.0, 6500.0]);
	//println!("{:?}", bb3000.spectra(SpectralDomain::default()));
	let c31 = cie1931();
	let s = bb3000.spectra(c31.domain());
//	println!("{:?}", c31.cmf.transpose() * s);
	println!("{:?}", s.nrows());
	println!("{:?}", c31.cmf.nrows());
	println!("{:?}", c31.cmf * s);
	 */
}
