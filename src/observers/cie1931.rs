use nalgebra::{ArrayStorage, Dynamic, Matrix, Matrix3xX, SMatrix, U3, VecStorage, convert};
use crate::util::linear_interpolate_rows_from_static_data;
use crate::{observers::StandardObserver};
use crate::util::{sprague_rows};
use crate::util::{Domain};
use crate::util::{NM5, NM, WavelengthStep, Meter, Step};
use super::cie1931_data::{CIE1931NM1, CIE1931NM5};



#[derive(Debug,Clone,Default)]
pub struct CieObs1931Classic {}

impl StandardObserver for CieObs1931Classic {
	//const K: f64 = 683.0;
	const NAME: &'static str = "CIE1931 2º";

	fn domain(&self) -> Domain<WavelengthStep> {
		Domain::new( 360/5, 830/5,  NM5)
	}

	fn cmf<L>(&self, target: &Domain<L>) -> Matrix3xX<f64>
	where
		L: Step,
		Meter: From<<L>::UnitValueType>
	 {
//		calculate row interpolated values, and convert to Matrix3xX matrix... 
		let data = SMatrix::from_data(ArrayStorage(CIE1931NM5));
		convert(sprague_rows(&self.domain(), &target, &data))
	}

}

/**
CIE 1931 2º color matching functions, defined from 380 to 780nm, in steps of 1nm.

Source: ANSI/IES TM-30-18 Advanced Calculation Tool V2.0, Aug 10, 2018.

*/
#[derive(Debug,Clone,Default)]
pub struct CieObs1931 {}

impl StandardObserver for CieObs1931 {
	const K: f64 = 683.0;
	const NAME: &'static str = "CIE1931 2º (TM30)";

	fn domain(&self) -> Domain<WavelengthStep> {
		Domain::new( 380, 780,  NM)
	}

	fn cmf<L>(&self, target: &Domain<L>) -> Matrix<f64, U3,Dynamic,VecStorage<f64,U3,Dynamic>>
	where
		L: Step,
		Meter: From<<L>::UnitValueType>
	 {
		linear_interpolate_rows_from_static_data(&self.domain(), &target, &CIE1931NM1)
	}

}

