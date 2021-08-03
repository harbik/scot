/*!
Spectral distributions of color samples, or color swatches, as found in color swatch books and guides. 

Swatches are typically used to specify, or check, the color of products, and objects, such as the color of fabrics.
To see their color, they need to be illuminated, typically using ambient light, such as daylight, or artificial light, provided by 
LED or fluorescent lights. 
The collections in this library represent spectral reflectivity distributions, normalized to a peak reflectivity of 1.0.
They all implement the `SpectralData`, and `Swatch` traits.

Currently, this library has the following collections:

- X-Rites's ColorChecker chart samples, measured by BabelColor's Danny Pascale, averaged over 30 sample charts.


*/

//use std::{marker::PhantomData, vec::from_elem};

use nalgebra::{DMatrix};


use crate::{spectra::SpectralData, util::{domain::Domain, units::{Scale, WavelengthScale}}};

pub use crate::swatches::checker::ColorChecker;

pub trait Swatches: SpectralData {}
/// trait marker for swatch reflection spectra, 
/// such as the Munsell color swatches.


pub struct White ();

impl SpectralData for White {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
		where
			L: Scale,
			<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
			 {
        DMatrix::from_element(domain.len(), 1, 1.0)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
        Domain::default()
    }
}

impl Default for White {
    fn default() -> Self {
        Self()
    }
}

impl Swatches for White {}

pub struct Gray (pub f64);

pub const SW_WHITE: Gray = Gray(1.0);
pub const SW_GRAY90: Gray = Gray(0.9);
pub const SW_GRAY80: Gray = Gray(0.8);
pub const SW_GRAY70: Gray = Gray(0.7);
pub const SW_GRAY60: Gray = Gray(0.6);
pub const SW_GRAY50: Gray = Gray(0.5);
pub const SW_GRAY40: Gray = Gray(0.4);
pub const SW_GRAY30: Gray = Gray(0.3);
pub const SW_GRAY20: Gray = Gray(0.2);
pub const SW_GRAY10: Gray = Gray(0.1);
pub const SW_BLACK: Gray = Gray(0.0);

impl Gray {
	pub fn new(r: f64) -> Self {
		Gray(r)
	}
}
impl SpectralData for Gray {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
		where
			L: Scale,
	//		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
			 {
        DMatrix::from_element(domain.len(), 1, self.0)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
        Domain::default()
    }
}

impl Default for Gray {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Swatches for Gray {}

pub type Grey = Gray;

pub mod checker;

/*
	Represents the spectral distributions of a swatch collection, 
	illuminated with an illuminant.

	# Examples

	```
		use colorado::swatches::{SwatchView, ColorChecker};
		use colorado::illuminants::D65;
		use colorado::cie::{self, Lab};
		use colorado::observers::Cie1931;
		let cc = ColorChecker::default();
		let cc_d65 = SwatchView::<D65, _>::new(&cc);
		let lab_cc_d65 = cie::Lab::<Cie1931, D65>::from(cc_d65);
		println!("{}", lab_cc_d65);
	```

	
*/
/*
pub struct SwatchView<'a, I,S> {
	sd: &'a S,
	ill: PhantomData<I>
}

impl<'a, I, S> SwatchView<'a, I, S> 
where 
	S: Swatches,
	I: Illuminant
{
    pub fn new(sd: &'a S) -> Self { Self { sd, ill: PhantomData } }
}


impl<'a, I, S> SpectralData for SwatchView<'a, I, S>
where
	S: Swatches,
	I: Illuminant,
	<<S as SpectralData>::ScaleType as Scale>::UnitType: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>,

{
    type ScaleType = <I as SpectralData>::ScaleType; // choose illuminant domain as domain basis

    fn values<L>(&self, domain: Domain<L>) -> DMatrix<f64>
		where
			L: Scale,
			<<Self as SpectralData>::ScaleType as Scale>::UnitType: From<<L>::UnitType>
			 {

				let ill  = I::default();
				for vc in self.sd.values::<I::ScaleType>(ill.domain()). {
					
				}
//				let v = ill.values::<I::ScaleType>(ill.domain()).column(0) * self.sd.values::<I::ScaleType>(ill.domain());
				sprague_cols(&ill.domain(), &domain, &v)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
		I::default().domain()
    }
}
 */