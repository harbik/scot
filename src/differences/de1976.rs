
/*!

# Example

DE1976<Cie1931, D65>::from(CES::<ALL>::default, )
 */

use std::marker::PhantomData;

use crate::models::CieLab;
use crate::models::lab::LabValues;
use crate::observers::{StandardObserver};
use crate::illuminants::{Illuminant};

//use super::DeltaE;

#[derive(Debug)]
pub struct CIEDE76<C,I>(Vec<f64>, PhantomData<C>, PhantomData<I>);

impl<'a, C: StandardObserver, I: Illuminant> CIEDE76<C,I>
where
	&'a C: Default
 {

    fn new<L1, L2>(l1: L1 , l2: L2) -> Vec<f64>
	where 
		L1: Into::<CieLab<C,I>>,
		L2: Into::<CieLab<C,I>>,
	{
		let lab1: CieLab::<C,I> = l1.into();
		let lab2: CieLab::<C,I> = l2.into();
		let mut v: Vec<f64> = Vec::with_capacity(lab1.data.ncols().min(lab2.data.ncols()));
		for (LabValues{l:l1,a:a1,b:b1}, LabValues{l:l2,a:a2, b:b2}) in  lab1.into_iter().zip(lab2.into_iter()) {
			v.push(((l2-l1)*(l2-l1) + (a2-a1)*(a2-a1) + (b2-b1)*(b2-b1)).sqrt());
		};
		v
    }
}

#[test]
fn test_ciede76(){
	use crate::observers::{Cie1931};
	use crate::illuminants::{D65};
	use crate::swatches::{ColorChecker, White};
	let de = CIEDE76::<Cie1931, D65>::new(ColorChecker, White);
	println!("{:?}", de);
}



