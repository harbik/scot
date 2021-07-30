
/// Stefan Boltzmann law, describing a blackbody radiant emittance (W m<sup>-2</sup>), as function of its absolute
/// temperature (K).
#[inline]
pub fn stefan_boltzmann(temp_kelvin: f64) -> f64 {

	SIGMA * temp_kelvin.powi(4)
}
/// Stefan-Boltzmann constant (W m<sup>-2</sup> K<sup>-4</sup>)
const SIGMA: f64 = 5.670_374_419_184E-8;

/**
Calculates spectral radiant exitance (W m<sup>-2</sup> m<sup>-1</sup> or W m<sup>-3</sup>), as function of input parameters wavelength (m), absolute
temperature (K), and radiant exitance (W m<sup>2</sup>). Integrated over the full spectral domain this function results in Stefan Boltzmann's law,
included here as a scaling factor to set the output power scale to a specified radiant power as input. 
*/
#[inline]
pub fn planck(wl: f64, temp: f64, pow: f64) -> f64 {
	pow / stefan_boltzmann(temp) * C1 / wl.powi(5) / ((C2 / (wl * temp)).exp() - 1.0)
}

/// The speed of light (m/s)
const C: f64 = 299792458.0; 

/// Boltzmann constant (m<sup>2</sup> kg s<sup>-2</sup> K<sup>-1</sup>)
const KB: f64 = 1.3806485279E-23; 

/// Planck constant (m<sup>2</sup> kg / s)
const H: f64 = 6.6260700408181E-34; 

/// First radiation constant (W m<sup>2</sup>)
const C1: f64 = 2. * std::f64::consts::PI * H * C * C; 

/// Second radiation constant (m K)
pub const C2: f64 = H * C / KB;

//pub const C2_IN_1931: f64 = 1.435E-2;


pub const ELECTRONVOLT_AS_JOULE: f64 = 1.602_176_634E-19; // Joule
pub const ELECTRONVOLT_AS_METER: f64 = 1.973_27E-7; // Meter