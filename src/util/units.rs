pub const NM: Meter = Meter { size: 1, exp: -9 };
pub const NM2: Meter = Meter { size: 2, exp: -9 };
pub const NM5: Meter = Meter { size: 5, exp: -9 };
pub const NM10: Meter = Meter { size: 1, exp: -8 };
pub const Å: Meter = Meter { size: 1, exp: -10};
pub const Å2: Meter = Meter { size: 2, exp: -10};
pub const Å5: Meter = Meter { size: 5, exp: -10};
pub const NONE100: Unitless = Unitless { size: 1, exp: 2};
pub const NONE50: Unitless = Unitless { size: 5, exp: 1};
pub const K1: Kelvin = Kelvin { size: 1, exp: 0};
pub const K10: Kelvin = Kelvin { size: 1, exp: 1};
pub const K50: Kelvin = Kelvin { size: 5, exp: 1};
pub const K100: Kelvin = Kelvin { size: 1, exp: 2};
pub const K1000: Kelvin = Kelvin { size: 1, exp: 3};

#[inline]
fn val(i: i32, size: u32, exp: i32 ) -> f64 {
	i as f64 * size as f64 * 10f64.powi(exp)
}

pub trait Unit : Clone + Copy + PartialEq + Eq  {
	fn symbol() -> &'static str;
	fn name() -> &'static str;
	fn value(&self, v: i32) -> f64;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unitless{
	pub size: u32,
	pub exp: i32,
}

impl Unit for Unitless {
	fn symbol() -> &'static str {
		"-"
	}

	fn name() -> &'static str {
		"Unitless"
	}

	fn value(&self, i: i32) -> f64 {
		val(i, self.size, self.exp)
	}

}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub struct Kelvin{
	pub size: u32,
	pub exp: i32,
}


impl Unit for Kelvin {
	fn symbol() -> &'static str {
		"K"
	}

	fn name() -> &'static str {
		"Kelvin"
	}

	fn value(&self, i: i32) -> f64 {
		val(i, self.size, self.exp)
	}


}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Meter{
	pub size: u32,
	pub exp: i32,
}


impl Unit for Meter {
	fn symbol() -> &'static str {
		"m"
	}

	fn name() -> &'static str {
		"meter"
	}

	fn value(&self, i: i32) -> f64 {
		val(i, self.size, self.exp)
	}

}
