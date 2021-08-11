/**
	A collection of units for physics quantities used in this library.

	# Examples

	Example of a domain quantity is `Meter`, and its value can be obtained
	using its value method.
	```
	use colorado::util::units::{Meter, Unit};
	use approx::assert_abs_diff_eq;

	let m = Meter(1.2345);
	assert_abs_diff_eq!(m.value(), 1.2345);
 */

pub trait Unit {
	const SYMBOL : &'static str;
	const NAME : &'static str;
	fn value(&self) -> f64;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Unitless(pub f64);

impl Unit for Unitless {
	const SYMBOL: &'static str = "-";
	const NAME: &'static str = "Unitless";
	fn value(&self) ->f64 {
		self.0
	}
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Kelvin (pub f64);

impl Unit for Kelvin {
	const SYMBOL: &'static str = "K";
	const NAME: &'static str = "Kelvin";
	fn value(&self) ->f64 {
		self.0
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Meter (pub f64);


impl Unit for Meter {
	const SYMBOL: &'static str = "m";
	const NAME: &'static str = "Meter";
	fn value(&self) ->f64 {
		self.0
	}
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Inch (pub f64);

impl Unit for Inch {
	const SYMBOL: &'static str = "in";
	const NAME: &'static str = "Inch";
	fn value(&self) ->f64 {
		self.0
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Lumen(pub f64);

impl Unit for Lumen {
	const SYMBOL: &'static str = "lm";
	const NAME: &'static str = "Lumen";
	fn value(&self) ->f64 {
		self.0
	}
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Joule(f64);

impl Unit for Joule {
	const SYMBOL: &'static str = "J";
	const NAME: &'static str = "Joule";
	fn value(&self) ->f64 {
		self.0
	}
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Electronvolt(pub f64);

impl Unit for Electronvolt {
	const SYMBOL: &'static str = "eV";
	const NAME: &'static str = "electronvolt";
	fn value(&self) ->f64 {
		self.0
	}
}

// Conversions
const INCH_TO_METER: f64 = 0.0254;
const METER_TO_INCH: f64 = 1.0/INCH_TO_METER;

impl From<Inch> for Meter {
	fn from(inch: Inch) -> Meter {
		Meter(inch.value() * INCH_TO_METER)
	}
}

impl From<Meter> for Inch {
	fn from(meter: Meter) -> Inch {
		Inch(meter.value() * METER_TO_INCH)
	}
}


pub const A: WavelengthScale = WavelengthScale { size: 1, exp: -10}; // Angstrom
pub const A2: WavelengthScale = WavelengthScale { size: 2, exp: -10};
pub const A5: WavelengthScale = WavelengthScale { size: 5, exp: -10};

pub const NM: WavelengthScale = WavelengthScale { size: 1, exp: -9 }; // nanometer
pub const NM2: WavelengthScale = WavelengthScale { size: 2, exp: -9 };
pub const NM5: WavelengthScale = WavelengthScale { size: 5, exp: -9 };
pub const NM10: WavelengthScale = WavelengthScale { size: 1, exp: -8 };
pub const UM: WavelengthScale = WavelengthScale { size: 1, exp: -6 }; // micrometer

pub const NONE100: UnitlessScale = UnitlessScale { size: 1, exp: 2};
pub const NONE50: UnitlessScale = UnitlessScale { size: 5, exp: 1};
pub const NONE5: UnitlessScale = UnitlessScale { size: 5, exp: 0};
pub const NONE2: UnitlessScale = UnitlessScale { size: 2, exp: 0};
pub const NONE: UnitlessScale = UnitlessScale { size: 1, exp: 0};
pub const PCT: UnitlessScale = UnitlessScale { size: 1, exp: -2};

pub const LM: LuminousFluxScale = LuminousFluxScale { size: 1, exp: 0};
pub const KLM: LuminousFluxScale = LuminousFluxScale { size: 1, exp: 3};

pub const K1: CCTScale = CCTScale { size: 1, exp: 0};
pub const K10: CCTScale = CCTScale { size: 1, exp: 1};
pub const K50: CCTScale = CCTScale { size: 5, exp: 1};
pub const K100: CCTScale = CCTScale { size: 1, exp: 2};
pub const KK: CCTScale = CCTScale { size: 1, exp: 3}; // kilo Kelvin, or kK

pub const DEV: PhotonEnergyScale = PhotonEnergyScale { size: 1, exp: -1}; // deci electronvolt, or 0.1

#[inline]
fn val(i: i32, size: u32, exp: i32 ) -> f64 {
	i as f64 * size as f64 * 10f64.powi(exp)
}

/**
	
 */
pub trait Scale : Clone + Copy + PartialEq + Eq  {
	const TITLE: &'static str;
	type UnitType: Unit; // Implements Unit trait
	fn unit(&self, v: i32) -> Self::UnitType;
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitlessScale{
	pub size: u32,
	pub exp: i32,
}


impl Scale for UnitlessScale {
	const TITLE: &'static str = "-";
	type UnitType = Unitless;


	fn unit(&self, i: i32) -> Self::UnitType {
		Unitless(val(i, self.size, self.exp))
	}

}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub struct CCTScale {
	pub size: u32,
	pub exp: i32,
}

impl Scale for CCTScale {
	const TITLE: &'static str = "Correlated Color TemperatureA";
	type UnitType = Kelvin;

	fn unit(&self, i: i32) -> Self::UnitType {
		Kelvin( val(i, self.size, self.exp))
	}


}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WavelengthScale{
	pub size: u32,
	pub exp: i32,
}

impl Scale for WavelengthScale {
	const TITLE: &'static str = "Wavelength";
	type UnitType = Meter;

	fn unit(&self, i: i32) -> Self::UnitType {
		Meter(val(i, self.size, self.exp))
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LuminousFluxScale {
	pub size: u32,
	pub exp: i32,
}


impl Scale for LuminousFluxScale {
	const TITLE: &'static str = "Luminous Flux";
	type UnitType = Lumen;

	fn unit(&self, i: i32) -> Self::UnitType {
		Lumen(val(i, self.size, self.exp))
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct JouleScale {
	pub size: u32,
	pub exp: i32,
}


impl Scale for JouleScale {
	const TITLE: &'static str = "Energy";
	type UnitType = Joule;

	fn unit(&self, i: i32) -> Self::UnitType {
		Joule (val(i, self.size, self.exp))
	}

}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PhotonEnergyScale {
	pub size: u32,
	pub exp: i32,
}

impl Scale for PhotonEnergyScale {
	const TITLE: &'static str = "Photon Energy";
	type UnitType = Electronvolt;

	fn unit(&self, i: i32) -> Self::UnitType {
		Electronvolt(val(i, self.size, self.exp))
	}
}

