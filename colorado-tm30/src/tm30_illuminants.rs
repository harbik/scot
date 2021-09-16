/*!

IES TM30 Sample Illuminants

The collection of spectral distribution data use used by the IES TM30 standardization committee for testing
their color fidelity and color preference metrics.

*/


mod data;
pub use data::*;

use std::collections::HashMap;
use colorado::illuminants::Illuminant;
use colorado::observers::StandardObserver;
use colorado::models::{CieXYZ, XYZValues};
use colorado::{DataSpectrumFromSlice, Domain, NM, SpectralDistribution, WavelengthStep};
use nalgebra::{SVectorSlice, Matrix3xX, SMatrixSlice};
use self::data::{TM30_ILLUMINANTS_DATA, TM30_CIE1931, N, M};


#[derive(Clone,  PartialEq,  Eq)]
pub enum EmissionType {
	FluorescentBroadband = 0,
	FluorescentNarrowband = 1,
	HighIntensityDischarge = 3,
	IncandescentOrFilament = 4,
	LedHybrid = 5,
	LedMixed = 6,
	LedPhosphor = 7,
	Mathematical = 8,
	Other = 9,
}

pub enum ModelType {
	Model = 0,
	Commercial = 1,
	Experimental = 2,
	Theoretical = 3,
}

pub fn tm30_cie1931_xy() -> HashMap<&'static str, [f64;2]> {
	TM30_CIE1931.iter().map(|(key,_,_,x,y)|(*key,[*x,*y])).collect()
}


impl From<EmissionType> for Vec<&str> {
	fn from(et: EmissionType) -> Self {
		let e = et as u32;
		let mut v: Vec<&str> = Vec::with_capacity(M);
		for (k,j,..) in TM30_CIE1931.iter() {
			if e==*j {
				v.push(k);
			}
		}
		v
	}

}

impl<C: StandardObserver> From<EmissionType> for CieXYZ<C> {
	fn from(et: EmissionType) -> Self {
		let e = et as u32;
		let mut v: Vec<f64> = Vec::with_capacity(3*M);
		for (i,(_,j,..)) in TM30_CIE1931.iter().enumerate(){
			if e==*j {
				let sd = DataSpectrumFromSlice::new(Domain::new(380, 780, NM), &TM30_ILLUMINANTS_DATA[i*N..(i+1)*N]);
				let XYZValues{x, y, z} = sd.xyz::<C>().into_iter().next().unwrap();
				v.push(x);
				v.push(y);
				v.push(z);
			}
		}
		Self::new(Matrix3xX::from_vec(v))
	}
}



#[derive(Default)]
pub struct TM30Illuminant<const K:usize>;

impl<const K:usize> SpectralDistribution for TM30Illuminant<K> {
    type MatrixType = SVectorSlice<'static, f64, N>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (Domain::new(380, 780, NM), Self::MatrixType::from_slice(&TM30_ILLUMINANTS_DATA[(K-1)*N..K*N]))
    }

    fn shape(&self) -> (usize, usize) {
		(N,1)
    }
}

impl<const K:usize> Illuminant for TM30Illuminant<K>{}

impl<C: StandardObserver, const K: usize> From<TM30Illuminant<K>> for CieXYZ<C> {
	fn from(ill: TM30Illuminant<K>) -> Self {
		ill.xyz()	
	}
}

#[test]
fn test_tm30_ill(){
	use colorado::models::CieYxy;
	let ill = TM30Illuminant::<CIE_F1>;
	let xy: CieYxy = ill.into();
	println!{"{}", xy};
}

#[derive(Default)]
pub struct TM30Illuminants;

impl SpectralDistribution for TM30Illuminants {
    type MatrixType = SMatrixSlice<'static, f64, N, M>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (Domain::new(380, 780, NM), Self::MatrixType::from_slice(&TM30_ILLUMINANTS_DATA))
    }

    fn shape(&self) -> (usize, usize) {
		(N,M)
    }
}


impl<C: StandardObserver> From<TM30Illuminants> for CieXYZ<C> {
	fn from(ill: TM30Illuminants) -> Self {
		ill.xyz()	
	}
}

#[test]
fn test_tm30_illuminants(){
	use colorado::models::CieYxy;
	let ill = TM30Illuminants;
	let xy: CieYxy = ill.into();
	println!{"{}", xy.data.transpose()};
}




#[test]
fn test_from_emission_type(){
	use colorado::models::{CieYxy, YxyValues};
	use approx::assert_abs_diff_eq;

	for emission_type in [
			EmissionType::FluorescentNarrowband,
			EmissionType::FluorescentBroadband,
			EmissionType::HighIntensityDischarge,
			EmissionType::IncandescentOrFilament,
			EmissionType::LedHybrid,
			EmissionType::LedMixed,
			EmissionType::LedPhosphor,
			EmissionType::Mathematical,
			EmissionType::Other,
		]
		{
		let xyz: CieYxy = emission_type.clone().into();
		let keys: Vec<&str> = emission_type.into();
		let w = tm30_cie1931_xy();
		for (YxyValues {l: _, x,y}, k) in xyz.into_iter().zip(keys.into_iter()) {
			let [xw,yw] = w[k];
			assert_abs_diff_eq!(x,xw,epsilon=5E-7);
			assert_abs_diff_eq!(y,yw,epsilon=5E-7);
			println!("{} {} {} {} {}" , k, x, y, xw, yw);
		}

	}
}