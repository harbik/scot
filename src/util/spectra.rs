use std::ops::Index;

use nalgebra::{DMatrix, Dim, Matrix, MatrixSlice, storage::Storage};
use crate::{interp_cols, sprague_cols, sprague_cols_index_based, step, util::{Domain, Step, }};

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

	fn len(&self) -> usize;

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

	fn values<S2:Step>(&self, dto: Domain<S2>) -> DMatrix<f64>
	where 
		<<Self as SpectralDistribution>::StepType as Step>::UnitValueType: From<<S2 as Step>::UnitValueType>,
	 {
		let (dfr, s) = self.spd();
	//	sprague_cols_index_based::<_, S2, _>(&dfr, &dto, s, self.len())
		sprague_cols_index_based(&dfr, &dto, s, self.len())
	}
}

