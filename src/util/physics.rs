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

This is Planck's law for vacuum. 
*/
#[inline]
pub fn planck(wl: f64, temp: f64, pow: f64) -> f64 {
    pow / stefan_boltzmann(temp) * C1 / wl.powi(5) / ((C2 / (wl * temp)).exp() - 1.0)
}

/**
Planck with the second radiant constant as parameter. 

This to be used for planckian radiators not in vacuum, or to calculate standard illuminants defined with 
older values of this constant.

l: wavelength in meter
t: absolute temperature in Kelvin
c2: second radiative constant, in meter * Kelvin; can also be used to include refractive index, using c2 ::  c2 / n

*/
#[inline]
pub fn planck_c2(l: f64, t: f64, c2:f64) -> f64 {
    C1 / l.powi(5) / ((c2 / (l * t)).exp() - 1.0)
}

/**
	Plank's law, differentiated to temperature.

	This formula is mainly used to calculate the normal to the Planckian locus, for correlated color temperature
	purposes.

	See formula 19, Li\[2016\]: Accurate method for computing correlated color temperature, 27 Jun 2016 | Vol. 24, No.
	13 | DOI:10.1364/OE.24.014066 | OPTICS EXPRESS 14066
*/
#[inline]
pub fn planck_prime_c2(l: f64, t: f64, c2: f64) -> f64 {
    let c3 = C1 * c2 / t.powi(2);
    let e = (c2 / (l * t)).exp();
    c3 / l.powi(6) * e / (e - 1.0).powi(2)
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
/// see https://en.wikipedia.org/wiki/Planckian_locus#International_Temperature_Scale
pub const C2: f64 = H * C / KB; // Now exact 
pub const C2_NBS_1931: f64 = 1.435E-2; // A Illuminant 
pub const C2_IPTS_1948: f64 = 1.4380E-2;  // Illuminant series D
pub const C2_IPTS_1990: f64 = 1.4388E-2; 

pub const ELECTRONVOLT_AS_JOULE: f64 = 1.602_176_634E-19; // Joule
pub const ELECTRONVOLT_AS_METER: f64 = 1.973_27E-7; // Meter

// See Ohno, Spectral Design considerations for white LED Color Rendering, Optical Engineering 44(11), November 20005
// Scale by spectralWidth
pub fn led_ohno(wl: f64, peak: f64, width: f64) -> f64 {
    let t = (wl - peak) / width;
    let g = (-(t.powi(2))).exp();
    (g + 2.0 * g.powi(5)) / 3.0
}
