//use super::physics::{ELECTRONVOLT_AS_JOULE, ELECTRONVOLT_AS_METER};

pub trait Unit {
	fn value(&self) -> f64;
	fn symbol() -> &'static str;
	fn name() -> &'static str;
}

pub struct Unitless(f64);

impl Unit for Unitless {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"-"
	}

	fn name() -> &'static str {
		"Unitless"
	}
}
pub struct Kelvin (f64);

impl Unit for Kelvin {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"K"
	}

	fn name() -> &'static str {
		"Temperature"
	}
}

pub struct Meter (f64);

impl Unit for Meter {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"m"
	}

	fn name() -> &'static str {
		"Length"
	}
}
pub struct Inch (pub f64);

impl Unit for Inch {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"in"
	}

	fn name() -> &'static str {
		"Length"
	}
}

pub struct Lumen(f64);

impl Unit for Lumen {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"lm"
	}

	fn name() -> &'static str {
		"Luminous Flux"
	}
}
pub struct Joule(f64);

impl Unit for Joule {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"J"
	}

	fn name() -> &'static str {
		"Energy"
	}
}
pub struct Electronvolt(f64);

impl Unit for Electronvolt {
	fn value(&self) ->f64 {
		self.0
	}

	fn symbol() -> &'static str {
		"eV"
	}

	fn name() -> &'static str {
		"Energy"
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


pub const A: MeterScale = MeterScale { size: 1, exp: -10}; // Angstrom
pub const A2: MeterScale = MeterScale { size: 2, exp: -10};
pub const A5: MeterScale = MeterScale { size: 5, exp: -10};

pub const NM: MeterScale = MeterScale { size: 1, exp: -9 }; // nanometer
pub const NM2: MeterScale = MeterScale { size: 2, exp: -9 };
pub const NM5: MeterScale = MeterScale { size: 5, exp: -9 };
pub const NM10: MeterScale = MeterScale { size: 1, exp: -8 };
pub const UM: MeterScale = MeterScale { size: 1, exp: -6 }; // nanometer

pub const NONE100: UnitlessScale = UnitlessScale { size: 1, exp: 2};
pub const NONE50: UnitlessScale = UnitlessScale { size: 5, exp: 1};
pub const PCT: UnitlessScale = UnitlessScale { size: 1, exp: -2};

pub const LM: LumenScale = LumenScale { size: 1, exp: 0};
pub const KLM: LumenScale = LumenScale { size: 1, exp: 3};

pub const K1: KelvinScale = KelvinScale { size: 1, exp: 0};
pub const K10: KelvinScale = KelvinScale { size: 1, exp: 1};
pub const K50: KelvinScale = KelvinScale { size: 5, exp: 1};
pub const K100: KelvinScale = KelvinScale { size: 1, exp: 2};
pub const KK: KelvinScale = KelvinScale { size: 1, exp: 3}; // kilo Kelvin, or kK

pub const DEV: ElectronvoltScale = ElectronvoltScale { size: 1, exp: -1}; // deci electronvolt, or 0.1

#[inline]
fn val(i: i32, size: u32, exp: i32 ) -> f64 {
	i as f64 * size as f64 * 10f64.powi(exp)
}

pub trait Scale : Clone + Copy + PartialEq + Eq  {
	type ValueType: Unit; // Implements Unit trait
	fn unit(&self, v: i32) -> Self::ValueType;
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitlessScale{
	pub size: u32,
	pub exp: i32,
}


impl Scale for UnitlessScale {
	type ValueType = Unitless;


	fn unit(&self, i: i32) -> Self::ValueType {
		Unitless(val(i, self.size, self.exp))
	}

}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub struct KelvinScale {
	pub size: u32,
	pub exp: i32,
}

impl Scale for KelvinScale {

	type ValueType = Kelvin;

	fn unit(&self, i: i32) -> Self::ValueType {
		Kelvin( val(i, self.size, self.exp))
	}


}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MeterScale{
	pub size: u32,
	pub exp: i32,
}

impl Scale for MeterScale {

	type ValueType = Meter;

	fn unit(&self, i: i32) -> Self::ValueType {
		Meter(val(i, self.size, self.exp))
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LumenScale {
	pub size: u32,
	pub exp: i32,
}


impl Scale for LumenScale {
	type ValueType = Lumen;

	fn unit(&self, i: i32) -> Self::ValueType {
		Lumen(val(i, self.size, self.exp))
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct JouleScale {
	pub size: u32,
	pub exp: i32,
}


impl Scale for JouleScale {
	type ValueType = Joule;

	fn unit(&self, i: i32) -> Self::ValueType {
		Joule (val(i, self.size, self.exp))
	}

}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ElectronvoltScale {
	pub size: u32,
	pub exp: i32,
}

impl Scale for ElectronvoltScale {
	type ValueType = Electronvolt;

	fn unit(&self, i: i32) -> Self::ValueType {
		Electronvolt(val(i, self.size, self.exp))
	}
}

