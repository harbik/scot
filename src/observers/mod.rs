/*!
Color matching functions from Standard Observers, such as the CIE 1931 standard observer.

These observers, and their color matching functions, play a key role in Colorimetry, and are used througout this
library.  In particular, references to standard observers are used in many color models and collections of
chromaticity coordinates, to maintain consistency between all the different models and datasets, and to implement 
automatic transformations between the different mathematical representations of color.
*/


pub mod cie1931;
pub use cie1931::*;

pub mod cie1964;
pub use cie1964::*;

pub mod cie_f2;
pub use cie_f2::*;

pub mod cie_f10;
pub use cie_f10::*;

use nalgebra::{DMatrix, Matrix3x1, Matrix3xX, MatrixSlice3xX,};
use crate::{Domain, Meter, WavelengthStep, Step, Unit};


/**
	Color matching functions mapped to a spectral data domain.

	A trait to get a standard observer chromatic responses – generally referred to as color matching functions
	x&#772;(&lambda;), y&#772;(&lambda;) z&#772;(&lambda;) – as a matrix over target domain.
	The mapping is done using an interpolation algorithm: typically a linear interpolation is used 
	if the base data set is finely spaces (1nm).
 */
pub trait StandardObserver : Default
{
	const K: f64 = 683.0;
	const NAME: &'static str;	
//	const N: usize; 

	fn cmf<'a>() -> MatrixSlice3xX<'a, f64>;

	/**
		Chromatic response mapped to a spectral domain, as a matrix with the x,y, and z color matching fuctions 
		as row vectors, with their length being dynamic, and determined by the standard's wavelength domain.
		The target domain does not have to use unit `Meter`, but needs be be able to be converted into a `Meter-unit.
	*/
	fn values<L>(&self, target: &Domain<L>) -> Matrix3xX<f64>
		where 
			L: Step,
			Meter: From<<L>::UnitValueType>
		;



	/**
		Calculate tri-stimulus values from spectral data, represented by a domain `d`,
		and a `DMatrix<f64>` data array.

		Typically this function is not used directly: use functions like
		`cie::XYZ::<Cie1931>::from(D65)`.
		 or `cie::Lab::<Cie1931, D50>::from(cc: ColorChecker)` 
		instead.
	*/
	fn xyz_from_dom_mat<'a, L>(d: Domain<L>, m: DMatrix<f64>) -> Matrix3xX<f64>
	where 
		L: Step,
		Meter: From<<L>::UnitValueType>,
	//	&'a Self : Default
	{
		let c = <Self>::default();
		c.values(&d) * m * Self::K * d.step.unitvalue(1).value()
	}

	/**
		Calculate tri-stimulus values from reflection or transmission spectral data `m` `DMatrix<f64>` data array, and
		an illuminant `l`, both defined on domain `d`. 
		
		The illuminant matrix is represented by a `DMatrix<f64>`, but only its first spectral distribution is used: any 
		other data is ignored.  Typically this function is not used directly: use functions like `cie::Lab::<Cie1931,
		D50>::from(cc/*: ColorChecker*/)` instead.
	*/
	fn xyz_from_dom_ill_mat<'a, L>(d: Domain<L>, l: DMatrix<f64>, m: DMatrix<f64>) -> (Matrix3x1<f64>, Matrix3xX<f64>)
	where 
		L: Step,
		Meter: From<<L>::UnitValueType>,
	{
		assert!(l.nrows()==m.nrows());
		let c = <Self>::default().values(&d);
		let m: DMatrix<f64>  = DMatrix::from_fn(l.nrows(), m.ncols(), |i, j| l[(i,0)] * m[(i,j)]);
		(
			c.clone() * l.column(0) * Self::K * d.step.unitvalue(1).value(),
			c * m * Self::K * d.step.unitvalue(1).value()
		)
	}

	/// Domain associated with the data for the standard observer itself, as defined in their standard. 
	/// These standards uses meter as domain unit.
	fn domain(&self) -> Domain<WavelengthStep>;

	/*
	fn planckian_locus(ctl: CctLadder) -> CieXYZ<Self> {
		let v: Vec<f64> = ctl.into_iter().collect();
		CieXYZ::<Self>::from(Planckian::new(v))
	}
	 */
}



#[test]
 fn test_cie1931(){
 }


