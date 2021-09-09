/*!
	Calculate color differences between spectral color sets, and match color for selected observers.

	In color science the color difference between two color points is expressed as a single number,
	with dimensionless unit &Delta;E, and related to a just-noticeable difference JND by:
	
	   1 &Delta;E &approx; 2.3 JND.
	
	A just noticeable difference is the amount a physiological stimulus - a color of a pixel in a display for example –
	has to change in order to be noticed by an observer. It is defined as the difference where half of the observers are
	able to see the change, and the other half not.

	The unit &Delta;E for color differences was first introduced by the CIE in 1976 with the CIE
	&Delta;E<sup>\*</sup><sub>76</sub> definition of color difference, expressed as the distance between two points in
	the CIE L<sup>\*</sup>a<sup>\*</sup>b<sup>\*</sup> color space.
	This &Delta;E unit could have been scaled differently, to be equivalent to 1 JND, by either scaling the CIE
	L<sup>\*</sup>a<sup>\*</sup>b<sup>\*</sup> – further referred to here as CIELAB color space – space differently, or by defining the &Delta;E<sup>\*</sup><sub>76</sub>
	differently.
	The CIELAB color space was designed to be perceptually uniform, so that a distance between two points in a space represents
	the same magnitude of color difference perception independent of their location.
	Further – more recent – color research has found that the uniformity of this space can be quite improved, with the introduction 
	of the CIECAM color space in 2002.

	Color differences in this library are all defined as function of color space coordinates, which 
	in turn are derived from standard observer's tristimulus values. 
	They can be calculatedjj for different observers – the perceived color difference between two patches as
	viewed by one observer, and another –  and it is also possible to calculate color
	difference perception between different observers, from a viewpoint of one observer, or the other:
	for example, how two patches which appear to have the same color for me, can appear to have different 
	colors for another observer.


	Implemented are the following color difference metrics:
	- CIE &Delta;E<sub>1976</sub>
	- CIE &Delta;E<sub>1994</sub>
	- CIE &Delta;E<sub>2000</sub>
*/
pub mod ciede76;

use nalgebra::DMatrix;

use crate::illuminants::Illuminant;
use crate::observers::{StandardObserver};
use std::collections::{BTreeMap};

pub use self::ciede76::CieDE1976;

pub mod ciede94;
pub use self::ciede94::CieDE1994;
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
		for r in m.column_iter() {
			// Using a BTreeMap to order the results on insert
			let mut btm = BTreeMap::new();
			for (i,de) in r.iter().enumerate() {
				let mut k = (de * 1E6) as usize;
				// in the very unlikely case if a match is right in the middle between two points and the key, the color
				// difference value in usize format, already exists. Insert gives back None if insert was succesful, and
				// the value, (i) in this case when the insert failed as a key already existed. This results in a dE
				// error of 1.E-6 for the second point...
				while let Some(_v) = btm.insert(k, i){
					k += 1;
				};
			}
			if btm.len()!=m.nrows() {
				panic!("Uncorrect number of matches...")
			}
			btm.values().for_each(|v| matched.push(*v));
		}
		DMatrix::<usize>::from_vec(m.nrows(), m.ncols(), matched)
	}
}
#[test]
fn test_match(){
	use crate::swatches::{ColorCheckerSwatch,Ces};
	let _de: CieDE1976 = (ColorCheckerSwatch::<3>, Ces).into();

}
