use std::ops::{Index, Mul};

use nalgebra::{DMatrix, DVectorSlice, Matrix3xX};
use crate::{Domain, Meter, Step, Unit, WavelengthStep, lin_interp_mat_col, models::CieXYZ, observers::StandardObserver};

pub trait Pixel {}
/// trait marker for display pixel spectra


/**
	Get spectral data from spectral models, standards, and spectral libraries, mapped to a spectral domain.

	Spectral data is represented by a `nalgebra` matrix, with the spectral distribution values as column vectors,
	allowing further processing using this libary's matrix calculation methods. T
*/


pub trait SpectralDistribution {
	type MatrixType: Index<(usize,usize), Output = f64>;
	type StepType: Step; 

	fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType);

	fn shape(&self) -> (usize, usize);

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

	fn map_domain<S2:Step>(&self, dto: Domain<S2>) -> DMatrix<f64>
	where 
		<<Self as SpectralDistribution>::StepType as Step>::UnitValueType: From<<S2 as Step>::UnitValueType>,
	 {
		let (dfr, s) = self.spd();
	//	sprague_cols_index_based::<_, S2, _>(&dfr, &dto, s, self.len())
//		sprague_cols_index_based(&dfr, &dto, s, self.len())
		lin_interp_mat_col(&dfr, &dto, self.shape().1, s)
	}

	fn xyz<C>(&self) -> CieXYZ<C> 
	where 
		C: StandardObserver,
		Meter: From<<<Self as SpectralDistribution>::StepType as Step>::UnitValueType>,
		Matrix3xX<f64>: Mul<Self::MatrixType>,
		<Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output: Mul<f64>,
		CieXYZ::<C>: From<<<Matrix3xX<f64> as Mul<<Self as SpectralDistribution>::MatrixType>>::Output as Mul<f64>>::Output>
	{
		let (d, s) = self.spd();
		let xyz = (C::values(&d) * s) * (C::K * C::domain().step.unitvalue(1).value());
		CieXYZ::<C>::from(xyz)
	}
}

pub struct DataSpectrumFromSlice<'a> {
	d: Domain<WavelengthStep>,
	m: &'a [f64],
}

impl<'a> DataSpectrumFromSlice<'a> {
    pub fn new(d: Domain<WavelengthStep>, m: &'a [f64]) -> Self { 
		assert_eq!(d.len(), m.len());
		Self { d, m } 
	}
}

impl<'a> SpectralDistribution for DataSpectrumFromSlice<'a> {
	type MatrixType = DVectorSlice<'a, f64>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
		(self.d.clone(), <Self::MatrixType>::from(self.m))
    }

    fn shape(&self) -> (usize, usize) {
       (self.d.len(), 1)
    }
}