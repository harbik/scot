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

pub trait Unit: PartialEq {
    const SYMBOL: &'static str;
    const NAME: &'static str;
    fn value(&self) -> f64;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Unitless(pub f64);

impl Unit for Unitless {
    const SYMBOL: &'static str = "-";
    const NAME: &'static str = "Unitless";
    fn value(&self) -> f64 {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Kelvin(pub f64);

impl Unit for Kelvin {
    const SYMBOL: &'static str = "K";
    const NAME: &'static str = "Kelvin";
    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Meter(pub f64);

impl Unit for Meter {
    const SYMBOL: &'static str = "m";
    const NAME: &'static str = "Meter";
    fn value(&self) -> f64 {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Inch(pub f64);

impl Unit for Inch {
    const SYMBOL: &'static str = "in";
    const NAME: &'static str = "Inch";
    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Lumen(pub f64);

impl Unit for Lumen {
    const SYMBOL: &'static str = "lm";
    const NAME: &'static str = "Lumen";
    fn value(&self) -> f64 {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Joule(pub f64);

impl Unit for Joule {
    const SYMBOL: &'static str = "J";
    const NAME: &'static str = "Joule";
    fn value(&self) -> f64 {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Electronvolt(pub f64);

impl Unit for Electronvolt {
    const SYMBOL: &'static str = "eV";
    const NAME: &'static str = "electronvolt";
    fn value(&self) -> f64 {
        self.0
    }
}

// Conversions
const INCH_TO_METER: f64 = 0.0254;
const METER_TO_INCH: f64 = 1.0 / INCH_TO_METER;

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
