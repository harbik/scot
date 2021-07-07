
use nalgebra::DMatrix;

use crate::spectra::{Illuminant, SpectralDistribution, SpectralDomain};
use crate::illuminants::cct::{CCTs};
use crate::util::physics::planck;


/**
	Representation of one or multiple generic blackbody illuminants. Can be constructed with various parameters
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

	fn values(&self, dom: SpectralDomain) -> DMatrix<f64> {
		DMatrix::from_fn(dom.size, self.ccts.0.nrows(),
		|r,c| { 
			let t = self.ccts.0[(c,0)];
			let p = self.ccts.0[(c,1)];
			let l = ((r + dom.low) * dom.unit) as f64 * 1E-10; // 1 Angstrom is 1E-10 m.
			planck(l, t, p)
		}
	 )

	}

	fn description(&self) -> Option<String> {
		Some("Blackbody Sources".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		self.ccts.keys()
	}

	fn domain(&self) -> SpectralDomain {
			todo!()
		}
}



#[test]
fn test_blackbody(){
	/*
	use crate::observer::{cie1931};

	let bb3000 = Blackbody::new([2700.0, 3000.0, 4000.0, 5000.0, 6500.0]);
	//println!("{:?}", bb3000.spectra(SpectralDomain::default()));
	let c31 = cie1931();
	let s = bb3000.spectra(c31.domain());
//	println!("{:?}", c31.cmf.transpose() * s);
	println!("{:?}", s.nrows());
	println!("{:?}", c31.cmf.nrows());
	println!("{:?}", c31.cmf * s);
	 */
}
