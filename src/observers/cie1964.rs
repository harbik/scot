use nalgebra::{ArrayStorage, Matrix3xX, SMatrix, convert};
use crate::util::{NM, linear_interpolate_rows_from_static_data};
use crate::{observers::StandardObserver};
use crate::util::{sprague_rows};
use crate::util::{Domain};
use crate::util::{NM5, WavelengthStep, Meter, Step};
use super::cie1964_data::{CIE1964NM1, CIE1964NM5};



#[derive(Debug,Clone,Default)]
pub struct CieObs1964Classic {}


impl StandardObserver for CieObs1964Classic {
	//const K: f64 = 683.0;
	const NAME: &'static str = "CIE1964 10º Classic";

	fn domain(&self) -> Domain<WavelengthStep> {
		Domain::new( 360/5, 830/5,  NM5)
	}

	fn cmf<L>(&self, target: &Domain<L>) -> Matrix3xX<f64>
	where
		L: Step,
		Meter: From<<L>::UnitValueType>
	 {
		let data = SMatrix::from_data(ArrayStorage(CIE1964NM5));
		convert(sprague_rows(&self.domain(), &target, &data))
	}

}
/**
CIE 1964 10º color matching functions, defined from 380 to 780nm, in steps of 1nm.


Source: ANSI/IES TM-30-18 Advanced Calculation Tool V2.0, Aug 10, 2018.

 */
#[derive(Debug,Clone,Default)]
pub struct CieObs1964 {}


impl StandardObserver for CieObs1964 {
	const K: f64 = 683.0;
	const NAME: &'static str = "CIE1964 10º 1nm";

	fn cmf<L>(&self, target: &Domain<L>) -> Matrix3xX<f64>
	where
		L: Step,
		Meter: From<<L>::UnitValueType>
	 {
		linear_interpolate_rows_from_static_data(&self.domain(), &target, &CIE1964NM1)
	}

	fn domain(&self) -> Domain<WavelengthStep> {
		Domain::new( 380, 780,  NM)
	}

}


#[test]
fn test_cmf(){
	use crate::observers::{CieObs1931};
	use crate::models::CieLab;
	use crate::swatches::ColorChecker;
	use crate::illuminants::D65;
//	let c = CieObs1964::default().cmf(&Domain::new(4,7,WavelengthScale { size: 1,  exp: -7}));

	let lab31: CieLab::<D65, CieObs1931> = ColorChecker::<13>.into();
	let lab64: CieLab::<D65, CieObs1964> = ColorChecker::<13>.into();
	println!("{} {}", lab31, lab64);
	
}
