#![doc = include_str!("README.md")]
/*
  Copyright 2021, Harbers Bik LLC

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/


//#![allow(unused_imports)]
#![allow(clippy::approx_constant)]
pub type DefaultObserver = self::observers::CieObs1931;

pub const ALL: usize = 0;

/// Color models/spaces such as CIE XYZ, CIELAB, CIECAM, and many more
pub mod models;

/// &Delta;E Color difference formulas: CIEDE74, CIEDE94, CIEDE2000, CIECAM UCS, &hellip;
pub mod differences;

#[macro_use]
pub mod illuminants;

#[macro_use]
pub mod swatches;

/**
CIE Standard Observers
 */
pub mod observers;

pub mod domain;
pub use self::domain::*;

pub mod interpolate;
pub use self::interpolate::*;

pub mod physics;
pub use self::physics::*;

pub mod step;
pub use self::step::*;

pub mod units;
pub use self::units::*;

pub mod math;
pub use self::math::*;

pub mod spectra;
pub use self::spectra::*;
