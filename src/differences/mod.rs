pub mod de1976;

use nalgebra::DMatrix;

use crate::illuminants::Illuminant;
use crate::observers::{StandardObserver};
use std::collections::{BTreeMap};

pub use self::de1976::CieDE1976;

pub mod de1994;
pub use self::de1994::CieDE1994;
//use crate::{illuminants::Illuminant, observers::StandardObserver, spectra::SpectralData, swatches::Swatches, util::units::{Meter, Scale}};

pub trait DeltaEValues<I,C>
where 
	I: Illuminant,
	C: StandardObserver,
	Self: AsRef<DMatrix<f64>>
{

	/**
	 Match indices ordered by color difference.

	 # Example

	 ```
	let de: CieDE1976 = (ColorChecker::<3>, IesTm30Ces).into();
	 ```

	 */
	fn matches(&self) -> DMatrix<usize> {
		let m:  &DMatrix<f64> = self.as_ref();
		let mut matched: Vec<usize> = Vec::with_capacity(m.len());
		for r in m.row_iter() {
			// Using a BTreeMap to order the results on insert
			let mut btm = BTreeMap::new();
			for (i,c) in r.iter().enumerate() {
				let mut k = (c * 1E6) as usize;
				// in the very unlikely case if a match is right in the middle between two points and the key, the color
				// difference value in usize format, already exists. Insert gives back None if insert was succesful, and
				// the value, (i) in this case when the insert failed as a key already existed. This results in a dE
				// error of 1.E-6 for the second point...
				while let Some(_v) = btm.insert(k, i){
					k += 1;
				};
			}
			if btm.len()!=m.ncols() {
				panic!("Uncorrect number of matches...")
			}
			btm.values().for_each(|v| matched.push(*v));
		}
		DMatrix::<usize>::from_vec(m.nrows(), m.ncols(), matched)
	}
}
#[test]
fn test_match(){
	use crate::swatches::{ColorChecker,IesTm30Ces};
	let _de: CieDE1976 = (ColorChecker::<3>, IesTm30Ces).into();

}
