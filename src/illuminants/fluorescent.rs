
use core::panic;

use nalgebra::{ArrayStorage, SMatrix, SVectorSlice};

use crate::spectra::SpectralData;
use crate::util::domain::Domain;
use crate::util::{WavelengthStep, Step, NM5, NM};
use crate::util::interpolate::sprague_cols;

use crate::ALL;
use super::fluorescent_data::*;

#[derive(Debug, Default)]
pub struct FL<const I:usize>;

impl<const I:usize> SpectralData for FL<I> {
    type ScaleType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Step,
		<Self::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType> 
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
    type ScaleType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Step,
		<Self::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType> 
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
pub struct IesTm30Fluorescent<const I:usize>;

impl<const I:usize> SpectralData for IesTm30Fluorescent<I> {
    type ScaleType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Step,
		<Self::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(FL_IES_DATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=M_IES => {
				let data = SVectorSlice::<f64, N_IES>::from_slice(&FL_IES_DATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in IES Fluorescent Data")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380, 780, NM)
    }

	fn keys(&self) -> Option<Vec<String>> {
		Some(FL_IES_KEYS.iter().map(|s| s.to_string()).collect())
	

	}


	fn description(&self) -> Option<String> {
		Some("CIE F3 Standard Illuminants".to_string())
	}
}


#[test]
fn test_f(){
	use crate::observers::CieObs1931;
	use approx::assert_abs_diff_eq;
	let f = crate::models::CieYxy::<CieObs1931>::from(FL::<1>);
	// println!("{}", f);
	let [_, x, y] = f.yxy(0);
	assert_abs_diff_eq!(x, 0.3131, epsilon=0.0005); // CIE.15.2004 table 8
	assert_abs_diff_eq!(y, 0.3371, epsilon=0.0005);

	let fall = crate::models::CieYxy::<CieObs1931>::from(FL::<ALL>);

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
	let f3all = crate::models::CieYxy::<CieObs1931>::from(FL3::<ALL>);
	// println!("{:.5}", f3all.data.slice_range(1..3,..));
	// println!("{:.5}", cie_fl3_test.slice_range(0..2,..));
	//	SMatrix::<f64, 12, 2>::from_iterator(fall_data.iter().cloned()), 
	//	SMatrix::<f64, 12, 2>::from_iterator(cie_fl_data.iter().cloned()),
	assert_abs_diff_eq!(
		SMatrix::<f64, 15, 2>::from_iterator(f3all.data.slice_range(1..3,..).iter().cloned()),
		SMatrix::<f64, 15, 2>::from_iterator(cie_fl3_test.slice_range(0..2,..).iter().cloned()),
		epsilon = 7E-5 // reference data's precision
	);

	let fies = crate::models::CieYxy::<CieObs1931>::from(IesTm30Fluorescent::<ALL>);
	println!("{:.5}", fies.data.slice_range(1..3,..).transpose());
	// println!("{:.5}", cie_fl3_test.slice_range(0..2,..));
	//	SMatrix::<f64, 12, 2>::from_iterator(fall_data.iter().cloned()), 
	//	SMatrix::<f64, 12, 2>::from_iterator(cie_fl_data.iter().cloned()),



}
pub static FLTEST : [[f64;4];12] = [
		[0.3131, 0.3371, 6430.0, 76.0], // x, y, CCT, CRI
		[0.3721, 0.3751, 4230.0, 64.0],
		[0.4091, 0.3941, 3450.0, 57.0],
		[0.4402, 0.4031, 2940.0, 51.0],
		[0.3138, 0.3452, 6350.0, 72.0],
		[0.3779, 0.3882, 4150.0, 59.0],
		[0.3129, 0.3292, 6500.0, 90.0],
		[0.3458, 0.3586, 5000.0, 95.0],
		[0.3741, 0.3727, 4150.0, 90.0],
		[0.3458, 0.3588, 5000.0, 81.0],
		[0.3805, 0.3769, 4000.0, 83.0],
		[0.4370, 0.4042, 3000.0, 83.0]
];

/**
Reference values for the CIE FL3-series standard illuminant, as provided by CIE in CIE Technical Report 15:2004, 3rd Edition.
*/
pub static FL3TEST : [[f64;18];15] = [
	// x, y, CCT, Ra, R1 ..= R14
	[0.4407, 0.4033, 2932.0, 51.0, 42.0, 69.0, 89.0, 39.0, 41.0, 52.0, 66.0, 13.0, -109.0, 29.0, 19.0, 21.0, 47.0, 93.0],
	[0.3808, 0.3734, 3965.0, 70.0, 65.0, 80.0, 89.0, 66.0, 66.0, 71.0, 79.0, 48.0, -37.0, 51.0, 56.0, 59.0, 68.0, 94.0],
	[0.3153, 0.3439, 6280.0, 72.0, 64.0, 80.0, 89.0, 69.0, 69.0, 74.0, 81.0, 49.0, -63.0, 52.0, 62.0, 68.0, 68.0, 93.0],
	[0.4429, 0.4043, 2904.0, 87.0, 91.0, 89.0, 79.0, 88.0, 88.0, 82.0, 88.0, 89.0, 76.0, 69.0, 88.0, 63.0, 91.0, 87.0],
	[0.3749, 0.3672, 4086.0, 95.0, 97.0, 97.0, 92.0, 94.0, 97.0, 95.0, 94.0, 94.0, 88.0, 90.0, 95.0, 90.0, 97.0, 95.0],
	[0.3488, 0.36, 4894.0, 96.0, 97.0, 97.0, 93.0, 97.0, 97.0, 95.0, 96.0, 96.0, 93.0, 90.0, 97.0, 92.0, 98.0, 95.0],
	[0.4384, 0.4045, 2979.0, 82.0, 97.0, 94.0, 54.0, 88.0, 86.0, 81.0, 87.0, 64.0, -9.0, 51.0, 76.0, 50.0, 98.0, 69.0],
	[0.382, 0.3832, 4006.0, 79.0, 94.0, 89.0, 50.0, 85.0, 83.0, 73.0, 86.0, 72.0, 5.0, 40.0, 68.0, 48.0, 95.0, 67.0],
	[0.3499, 0.3591, 4853.0, 79.0, 94.0, 89.0, 48.0, 84.0, 84.0, 72.0, 85.0, 78.0, 22.0, 38.0, 68.0, 51.0, 95.0, 66.0],
	[0.3455, 0.356, 5000.0, 88.0, 99.0, 97.0, 63.0, 92.0, 92.0, 85.0, 92.0, 86.0, 46.0, 62.0, 78.0, 72.0, 97.0, 75.0],
	[0.3245, 0.3434, 5854.0, 78.0, 90.0, 86.0, 49.0, 82.0, 81.0, 70.0, 85.0, 79.0, 24.0, 34.0, 64.0, 50.0, 90.0, 67.0],
	[0.4377, 0.4037, 2984.0, 93.0, 95.0, 98.0, 92.0, 95.0, 94.0, 97.0, 93.0, 83.0, 58.0, 88.0, 93.0, 85.0, 97.0, 94.0],
	[0.383, 0.3724, 3896.0, 96.0, 98.0, 97.0, 98.0, 97.0, 99.0, 97.0, 94.0, 88.0, 71.0, 99.0, 94.0, 89.0, 99.0, 98.0],
	[0.3447, 0.3609, 5045.0, 95.0, 93.0, 94.0, 97.0, 94.0, 94.0, 93.0, 97.0, 97.0, 93.0, 91.0, 95.0, 85.0, 92.0, 97.0],
	[0.3127, 0.3288, 6509.0, 98.0, 99.0, 99.0, 96.0, 98.0, 99.0, 100.0, 98.0, 98.0, 96.0, 99.0, 100.0, 95.0, 98.0, 98.0]
];