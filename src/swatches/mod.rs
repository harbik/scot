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




use nalgebra::DMatrix;

use crate::observers::StandardObserver;
use crate::{Meter, SpectralDistribution, Step, Unit};
use crate::models::{CieLab,  cielab};

/**
	Traits for swatches, libraries or models for color samples, to get their spectral distributions
	and, with a specified illuminant, their color appearance coordinates.
*/
pub trait Swatch
where
	Self: SpectralDistribution,
	Self: Default,

{
	fn lab<I,C>(&self) -> CieLab<I,C> 
	where
		C: StandardObserver,
		//I: Into::<CieXYZ<C>>,
		I: Default,
		I: SpectralDistribution,
		Meter: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
//		I: <I::StepType = Self::StepType>,
		<<I as SpectralDistribution>::StepType as Step>::UnitValueType: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
	{

		let (d, s) = self.spd();
		let c = C::values(&d);
		let l = I::default().map_domain(d.clone());
		let m: DMatrix<f64>  = DMatrix::from_fn(l.nrows(), self.len(), |i, j| l[(i,0)] * s[(i,j)]);
		let xyzn = &c * l.column(0) * C::K * d.step.unitvalue(1).value();
		let xyz = c * m * C::K * d.step.unitvalue(1).value();
		CieLab::new(cielab(xyzn, xyz)) 

	}
}
/**
	Macro to define a a swatch library from static data, and implement its `Swatch` traits.

	Examples of these swatch libaries defined as static data are the color checker 
	color samples, and the color samples used in various color quality standards, such as the
	CIE CRI and IES TM30.

*/

#[allow(unused_macros)]
macro_rules! swatch {
	// a single illuminant from static slice column
	($SWATCH:ident, $N:expr, $M:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident) => {

		#[derive(Debug, Default)]
		pub struct $SWATCH<const J:usize>;

		impl<const J:usize> crate::SpectralDistribution for $SWATCH<J> {
			type MatrixType = nalgebra::SMatrixSlice<'static, f64, $N, 1>;
			type StepType = crate::WavelengthStep;

			fn len(&self) -> usize {1}

			fn spd(&self) -> (crate::Domain<Self::StepType>, Self::MatrixType) {
				assert!(J>0&&J<=$M);
				(
					$DOMAIN,
					<Self as crate::SpectralDistribution>::MatrixType::from_slice(&$DATA[(J-1)*N..J*N]),
				)
			}
			
			fn description(&self) -> Option<String> {
				Some(format!($DESC, J))
			}
		}

		impl<const J:usize> crate::swatches::Swatch for $SWATCH<J> {}

		impl<I: crate::illuminants::Illuminant, C: crate::observers::StandardObserver, const J:usize> From<$SWATCH<J>> for crate::models::CieLab<I,C> 
		where
			<<I as crate::SpectralDistribution>::StepType as crate::Step>::UnitValueType: From<crate::Meter>	
		{
			fn from(sw: $SWATCH<J>) -> Self {
				use crate::swatches::Swatch;
				sw.lab()
			}
		}
	};
	// all swatches as array with keys
	($SWATCH:ident, $N:expr, $M:expr, $DESC:literal, $DOMAIN:expr, $DATA:ident, $KEYS:ident) => {

		#[derive(Debug, Default)]
		pub struct $SWATCH;

		impl crate::SpectralDistribution for $SWATCH {
			type MatrixType = nalgebra::SMatrixSlice<'static, f64, $N, $M>;
			type StepType = crate::WavelengthStep;

			fn len(&self) -> usize {$M}

			fn spd(&self) -> (crate::Domain<Self::StepType>, Self::MatrixType) {
				(
					$DOMAIN,
					<Self as crate::SpectralDistribution>::MatrixType::from_slice(&$DATA),
				)
			}

			fn keys(&self) -> Option<Vec<String>> {
				Some($KEYS.iter().map(|s| s.to_string()).collect())
			}
			
			fn description(&self) -> Option<String> {
				Some(format!($DESC))
			}
		}

		impl crate::swatches::Swatch for $SWATCH {}

		impl<I:crate::illuminants::Illuminant, C: crate::observers::StandardObserver> From<$SWATCH> for crate::models::CieLab<I,C>
		where
			<<I as crate::SpectralDistribution>::StepType as crate::Step>::UnitValueType: From<crate::Meter>
		{
			fn from(sw: $SWATCH) -> Self {
				use crate::swatches::Swatch;
				sw.lab()
			}
		}
			
	};
			
}



/*
#[derive(Default)]
pub struct White;

impl SpectralTable for White {
    type StepType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
		where
			L: Step,
			<Self::StepType as Step>::UnitValueType: From<<L>::UnitValueType> 
			 {
        DMatrix::from_element(domain.len(), 1, 1.0)
    }

    fn domain(&self) -> Domain<Self::StepType> {
        Domain::default()
    }
}

impl Swatch for White {}

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
impl SpectralTable for Gray {
    type StepType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
		where
			L: Step,
	//		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
			 {
        DMatrix::from_element(domain.len(), 1, self.0)
    }

    fn domain(&self) -> Domain<Self::StepType> {
        Domain::default()
    }
}

impl Default for Gray {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Swatch for Gray {}

pub type Grey = Gray;


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

pub mod checker;
pub use self::checker::*;

//pub mod tcs;
//pub use self::tcs::*;

//pub mod ces;
//pub use self::ces::*;
