
use core::panic;

use nalgebra::{ArrayStorage, SMatrix, SVectorSlice};

use crate::spectra::SpectralData;
use crate::util::Domain;
use crate::util::{WavelengthStep, Step, NM5, NM};
use crate::util::sprague_cols;

use crate::ALL;
use super::hid_data::*;

#[derive(Debug, Default)]
pub struct HP<const I:usize>;

impl<const I:usize> SpectralData for HP<I> {
    type ScaleType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Step,
		<Self::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(HP_DATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=N_CIE => {
				let data = SVectorSlice::<f64, N_CIE>::from_slice(&HP_DATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in CIE HP Illuminant")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380/5, 780/5, NM5)
    }

	fn description(&self) -> Option<String> {
		Some("CIE HP Standard Illuminants".to_string())
	}
}


#[derive(Debug, Default)]
pub struct IesTm30Hid<const I:usize>;

impl<const I:usize> SpectralData for IesTm30Hid<I> {
    type ScaleType = WavelengthStep;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Step,
		<Self::ScaleType as Step>::UnitValueType: From<<L>::UnitValueType> 
	{
		match I {
			ALL => {
				let data = SMatrix::from_data(ArrayStorage(HID_IES_DATA));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=N_IES => {
				let data = SVectorSlice::<f64, N_IES>::from_slice(&HID_IES_DATA[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in IES Fluorescent Data")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(380, 780, NM)
    }

	fn keys(&self) -> Option<Vec<String>> {
		Some(HID_IES_KEYS.iter().map(|s| s.to_string()).collect())
	

	}


	fn description(&self) -> Option<String> {
		Some("IES TM30 Example HID Illuminants".to_string())
	}
}
