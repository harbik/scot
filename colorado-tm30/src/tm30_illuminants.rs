mod data;

use std::collections::HashMap;
use colorado::observers::StandardObserver;
use colorado::models::{CieXYZ, XYZValues, CieYxy};
use colorado::{Domain, NM, DataSpectrumFromSlice, SpectralDistribution};
use nalgebra::Matrix3xX;
use self::data::{TM30_ILLUMINANTS_DATA, TM30_CIE1931, N, M};


#[derive(Clone)]
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