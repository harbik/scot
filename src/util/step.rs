use super::{Electronvolt, Joule, Kelvin, Lumen, Meter, Unit, Unitless};


/**
	Step – or interval – type and size for a set of equidistant data points.

 */

pub const A: WavelengthStep = WavelengthStep { size: 1, exp: -10}; // Angstrom
pub const A2: WavelengthStep = WavelengthStep { size: 2, exp: -10};
pub const A5: WavelengthStep = WavelengthStep { size: 5, exp: -10};

pub const NM: WavelengthStep = WavelengthStep { size: 1, exp: -9 }; // nanometer
pub const NM2: WavelengthStep = WavelengthStep { size: 2, exp: -9 };
pub const NM5: WavelengthStep = WavelengthStep { size: 5, exp: -9 };
pub const NM10: WavelengthStep = WavelengthStep { size: 1, exp: -8 };
pub const UM: WavelengthStep = WavelengthStep { size: 1, exp: -6 }; // micrometer

pub const NONE100: UnitlessStep = UnitlessStep { size: 1, exp: 2};
pub const NONE50: UnitlessStep = UnitlessStep { size: 5, exp: 1};
pub const NONE10: UnitlessStep = UnitlessStep { size: 1, exp: 1};
pub const NONE5: UnitlessStep = UnitlessStep { size: 5, exp: 0};
pub const NONE2: UnitlessStep = UnitlessStep { size: 2, exp: 0};
pub const NONE: UnitlessStep = UnitlessStep { size: 1, exp: 0};
pub const PCT: UnitlessStep = UnitlessStep { size: 1, exp: -2};

pub const LM: LuminousFluxStep = LuminousFluxStep { size: 1, exp: 0};
pub const KLM: LuminousFluxStep = LuminousFluxStep { size: 1, exp: 3};

pub const K1: CctStep = CctStep { size: 1, exp: 0};
pub const K10: CctStep = CctStep { size: 1, exp: 1};
pub const K50: CctStep = CctStep { size: 5, exp: 1};
pub const K100: CctStep = CctStep { size: 1, exp: 2};
pub const KK: CctStep = CctStep { size: 1, exp: 3}; // kilo Kelvin, or kK

pub const DEV: PhotonEnergyStep = PhotonEnergyStep { size: 1, exp: -1}; // deci electronvolt, or 0.1 eV

#[inline]
fn val(i: i32, size: u32, exp: i32 ) -> f64 {
	i as f64 * size as f64 * 10f64.powi(exp)
}

/**
	
 */
pub trait Step : Clone + Copy + PartialEq + Eq{
	const NAME: &'static str;
	type UnitValueType: Unit; // Implements Unit trait
	fn unitvalue(&self, v: i32) -> Self::UnitValueType;
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitlessStep{
	pub size: u32,
	pub exp: i32,
}


impl Step for UnitlessStep {
	const NAME: &'static str = "-";
	type UnitValueType = Unitless;


	fn unitvalue(&self, i: i32) -> Self::UnitValueType {
		Unitless(val(i, self.size, self.exp))
	}

}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub struct CctStep {
	pub size: u32,
	pub exp: i32,
}

impl Step for CctStep {
	const NAME: &'static str = "Correlated Color Temperature";
	type UnitValueType = Kelvin;

	fn unitvalue(&self, i: i32) -> Self::UnitValueType {
		Kelvin( val(i, self.size, self.exp))
	}


}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WavelengthStep{
	pub size: u32,
	pub exp: i32,
}

impl Step for WavelengthStep {
	const NAME: &'static str = "Wavelength";
	type UnitValueType = Meter;

	fn unitvalue(&self, i: i32) -> Self::UnitValueType {
		Meter(val(i, self.size, self.exp))
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LuminousFluxStep {
	pub size: u32,
	pub exp: i32,
}


impl Step for LuminousFluxStep {
	const NAME: &'static str = "Luminous Flux";
	type UnitValueType = Lumen;

	fn unitvalue(&self, i: i32) -> Self::UnitValueType {
		Lumen(val(i, self.size, self.exp))
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct JouleScale {
	pub size: u32,
	pub exp: i32,
}


impl Step for JouleScale {
	const NAME: &'static str = "Energy";
	type UnitValueType = Joule;

	fn unitvalue(&self, i: i32) -> Self::UnitValueType {
		Joule (val(i, self.size, self.exp))
	}

}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PhotonEnergyStep {
	pub size: u32,
	pub exp: i32,
}

impl Step for PhotonEnergyStep {
	const NAME: &'static str = "Photon Energy";
	type UnitValueType = Electronvolt;

	fn unitvalue(&self, i: i32) -> Self::UnitValueType {
		Electronvolt(val(i, self.size, self.exp))
	}
}

