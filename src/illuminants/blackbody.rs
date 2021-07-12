
use nalgebra::DMatrix;

use crate::cie::XYZ;
use crate::observers::StandardObserver;
use crate::spectra::{Illuminant, SpectralDistribution};
use crate::illuminants::cct::{CCTs};
//use crate::util::physics::planck;
use crate::util::domain::Domain;
use crate::util::units::{Meter};
//use std::iter::ExactSizeIterator;


/**
	Representation of one or multiple generic blackbody illuminants. 
	
	Can be constructed with various parameters
	as illustrated in the examples below.

	# Examples
 */
#[derive(Debug)]
pub struct Blackbody {
	pub ccts: CCTs,
}

impl Illuminant for Blackbody {}

impl Blackbody {

	pub fn new(parameters: impl Into<CCTs>) -> Blackbody
	{
		Blackbody {
			ccts: parameters.into(),
		}
	}
}

impl SpectralDistribution for Blackbody {


	fn values(&self, dom: Domain<Meter>) -> DMatrix<f64> {
		todo!()

	}


	/* 
	fn values(&self, dom: Domain<Meter>) -> DMatrix<f64> {
		DMatrix::from_fn(dom.range.len(), self.ccts.0.nrows(),
		|r,c| { 
			let t = self.ccts.0[(c,0)];
			let p = self.ccts.0[(c,1)];
			//let l = ((r + dom.low) * dom.unit) as f64 * 1E-10; // 1 Angstrom is 1E-10 m.
			planck(dom.value(r), t, p)
		}
	 )

	}
	*/

	fn description(&self) -> Option<String> {
		Some("Blackbody Sources".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		self.ccts.keys()
	}

	/// Domain which covers the total emission for all the radiators.
	fn domain(&self) -> Domain<Meter> {
		//SpectralDomain::default()
		todo!()
		}

	/**
		Calculate XYZ values for Blackbody radiators.
		
		Uses the observer's domain instead of the default Blackbody
		domain, which is very broad. This would be the default option,
		if this is was not overridden here.
		*/
	fn xyz<C:'static + StandardObserver>(&self, obs: &C) -> XYZ<C> {
		XYZ::<C> {
			xyz: obs.cmf(obs.domain()) * self.values(obs.domain()),
			white: None,
			cmf: C::global()

		}
	}
	
}



#[test]
fn test_blackbody(){
	use crate::observers::{Cie1931};
	use crate::cie::XYZ;

	let bb = XYZ::<Cie1931>::from(Blackbody::new(3000));
	println!("{}",bb);
}
