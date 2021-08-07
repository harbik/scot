
use core::panic;

use nalgebra::{ArrayStorage, SMatrix, SVectorSlice};

use crate::spectra::SpectralData;
use crate::util::domain::Domain;
use crate::util::units::{WavelengthScale, Scale, NM5, NM};
use crate::util::interpolate::sprague_cols;

use super::ALL;
use super::fluorescent_data::*;

#[derive(Debug, Default)]
pub struct FL<const I:usize>;

impl<const I:usize> SpectralData for FL<I> {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(FDATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=M => {
				let data = SVectorSlice::<f64, N>::from_slice(&FDATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in Fluorescent Illuminant")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380/5, 780/5, NM5)
    }

	fn description(&self) -> Option<String> {
		Some("CIE F Standard Illuminants".to_string())
	}
}


#[derive(Debug, Default)]
pub struct FL3<const I:usize>;

impl<const I:usize> SpectralData for FL3<I> {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(FL3DATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=M3 => {
				let data = SVectorSlice::<f64, N>::from_slice(&FL3DATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in New Fluorescent Illuminant")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380/5, 780/5, NM5)
    }

	fn description(&self) -> Option<String> {
		Some("CIE F3 Standard Illuminants".to_string())
	}
}

#[derive(Debug, Default)]
pub struct FIES<const I:usize>;

impl<const I:usize> SpectralData for FIES<I> {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(FIES_DATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=M_IES => {
				let data = SVectorSlice::<f64, N_IES>::from_slice(&FIES_DATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in IES Fluorescent Data")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380, 780, NM)
    }

	fn keys(&self) -> Option<Vec<String>> {
		Some(FIES_KEYS.iter().map(|s| s.to_string()).collect())
	

	}


	fn description(&self) -> Option<String> {
		Some("CIE F3 Standard Illuminants".to_string())
	}
}


#[test]
fn test_f(){
	use crate::observers::Cie1931;
	use approx::assert_abs_diff_eq;
	let f = crate::models::CieYxy::<Cie1931>::from(FL::<1>);
	// println!("{}", f);
	let [_, x, y] = f.yxy(0);
	assert_abs_diff_eq!(x, 0.3131, epsilon=0.0005); // CIE.15.2004 table 8
	assert_abs_diff_eq!(y, 0.3371, epsilon=0.0005);

	let fall = crate::models::CieYxy::<Cie1931>::from(FL::<ALL>);

	let cie_fl_test = SMatrix::from_data(ArrayStorage(FLTEST));
	let cie_fl_data = cie_fl_test.slice_range(..2, ..);
	
	let fall_data = fall.data.slice_range(1.., ..);
	
	//println!("{:.5}", fall_data);
	//println!("{:.5}", cie_fl_data);

	assert_abs_diff_eq!(
		SMatrix::<f64, 2, 12>::from_iterator(fall_data.iter().cloned()), 
		SMatrix::<f64, 2, 12>::from_iterator(cie_fl_data.iter().cloned()),
		epsilon = 5E-5 // reference data's precision
	);

	let cie_fl3_test = SMatrix::from_data(ArrayStorage(FL3TEST));
	let f3all = crate::models::CieYxy::<Cie1931>::from(FL3::<ALL>);
	// println!("{:.5}", f3all.data.slice_range(1..3,..));
	// println!("{:.5}", cie_fl3_test.slice_range(0..2,..));
	//	SMatrix::<f64, 12, 2>::from_iterator(fall_data.iter().cloned()), 
	//	SMatrix::<f64, 12, 2>::from_iterator(cie_fl_data.iter().cloned()),
	assert_abs_diff_eq!(
		SMatrix::<f64, 15, 2>::from_iterator(f3all.data.slice_range(1..3,..).iter().cloned()),
		SMatrix::<f64, 15, 2>::from_iterator(cie_fl3_test.slice_range(0..2,..).iter().cloned()),
		epsilon = 7E-5 // reference data's precision
	);

	let fies = crate::models::CieYxy::<Cie1931>::from(FIES::<ALL>);
	println!("{:.5}", fies.data.slice_range(1..3,..).transpose());
	// println!("{:.5}", cie_fl3_test.slice_range(0..2,..));
	//	SMatrix::<f64, 12, 2>::from_iterator(fall_data.iter().cloned()), 
	//	SMatrix::<f64, 12, 2>::from_iterator(cie_fl_data.iter().cloned()),



}