
/*!

# Example

DE1976<Cie1931, D65>::from(CES::<ALL>::default, )
 */

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::models::CieLab;
use crate::models::lab::LabValues;
use crate::observers::{StandardObserver};
use crate::illuminants::{Illuminant};

//use super::DeltaE;

#[derive()]
pub struct CIEDE76<C,I>(Vec<f64>, PhantomData<*const C>, PhantomData<*const I>);

impl<'a, C: StandardObserver, I: Illuminant> CIEDE76<C,I>
where
	&'a C: Default
 {

    pub fn new<L1, L2>(l1: L1 , l2: L2) -> Self
	where 
		L1: Into::<CieLab<C,I>>,
		L2: Into::<CieLab<C,I>>,
	{
		let lab1: CieLab::<C,I> = l1.into();
		let lab2: CieLab::<C,I> = l2.into();

		let mut v: Vec<f64>;

		match (lab1.len(),lab2.len()) {
			(0,_) | (_,0) => panic!("Need at least one color source"),
			(1, n) => {
				v = Vec::with_capacity(n);
				let LabValues{l:l1,a:a1,b:b1} = lab1.into_iter().next().unwrap();
				for LabValues{l:l2, a:a2, b:b2} in lab2 {
					v.push(((l2-l1)*(l2-l1) + (a2-a1)*(a2-a1) + (b2-b1)*(b2-b1)).sqrt());

				}
			},
			(n, 1) => {
				v = Vec::with_capacity(n);
				let LabValues{l:l2,a:a2,b:b2} = lab2.into_iter().next().unwrap();
				for LabValues{l:l1, a:a1, b:b1} in lab1 {
					v.push(((l2-l1)*(l2-l1) + (a2-a1)*(a2-a1) + (b2-b1)*(b2-b1)).sqrt());

				}
			},
			(n1, n2) => {
				v = Vec::with_capacity(n1.min(n2));
				for (LabValues{l:l1,a:a1,b:b1}, LabValues{l:l2,a:a2, b:b2}) in  lab1.into_iter().zip(lab2.into_iter()) {
					v.push(((l2-l1)*(l2-l1) + (a2-a1)*(a2-a1) + (b2-b1)*(b2-b1)).sqrt());
				}
			},

		}
		Self(v, PhantomData, PhantomData)
    }

	pub fn colormatch(&self) -> (usize, f64) {
		let mut imin = 0usize;
		let mut vmin = f64::MAX;
		for (i, v) in self.0.iter().enumerate() {
			if v<&vmin {
				imin = i;
				vmin = *v;
			}
		}
		(imin, vmin)
	}
}

impl<C: StandardObserver, I: Illuminant> Debug for CIEDE76<C,I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       self.0.fmt(f) 
    }
}

#[test]
fn test_ciede76(){
	use crate::observers::{Cie1931};
	use crate::illuminants::{D65};
	use crate::swatches::{ColorChecker, White};
	let de = CIEDE76::<Cie1931, D65>::new(ColorChecker, White);
	println!("{:?}", de);
	println!("{:?}", de.colormatch());
}



