use nalgebra::{DMatrix, Dim, MatrixSlice};
use crate::util::{Domain, Step, };

pub trait Pixel {}
/// trait marker for display pixel spectra


/**
	Get spectral data from spectral models, standards, and spectral libraries, mapped to a spectral domain.

	Spectral data is represented by a `nalgebra` matrix, with the spectral distribution values as column vectors,
	allowing further processing using this libary's matrix calculation methods. T
*/

pub trait SpectralData {

	type StepType: Step;


	/*

	fn spd<'a, R, C>() -> MatrixSlice<'a, f64, R, C>
	where 
		R: Dim,
		C: Dim;
		*/

	/**
		Values for a set of spectral distributions.

		Returns values for any spectral representation of light, such as from a light source, or an illuminated surface,
		in form of an nalgebra's `DMatrix<f64>`, with  one or more spectral distribution data as columns. The values are
		mapped to a specified domain, typically by interpolation, or by evaluation of functions for functional
		representations.
	*/
	fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
		where
			L: Step,
			<<Self as SpectralData>::StepType as Step>::UnitValueType: From<<L>::UnitValueType> 
			// need to be able to map the target domain onto the native domain of the spectral data,
			// or, in other words, need to be able to convert from the target domain's unit into the
			// object's domain's unit.
			; 

	/// spectral's native or default spectral range
	fn domain(&self) -> Domain<Self::StepType>; 

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

}

pub trait SpectralFunction {

	type StepType: Step;

	/**
		Values for a set of spectral distributions.

		Returns values for any spectral representation of light, such as from a light source, or an illuminated surface,
		in form of an nalgebra's `DMatrix<f64>`, with  one or more spectral distribution data as columns. The values are
		mapped to a specified domain, typically by interpolation, or by evaluation of functions for functional
		representations.
	*/
	fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
		where
			L: Step,
			<<Self as SpectralFunction>::StepType as Step>::UnitValueType: From<<L>::UnitValueType> 
			// need to be able to map the target domain onto the native domain of the spectral data,
			// or, in other words, need to be able to convert from the target domain's unit into the
			// object's domain's unit.
			; 


	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

}



