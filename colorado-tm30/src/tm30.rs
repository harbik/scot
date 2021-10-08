// TODO: implement CieCam<I,C,V>
// TODO: calculate Rf, Rg indices for one or more illuminants


// From Ocher

/*


use crate::physics::polygon_area;
use std::f64::consts::PI;
use crate::sources::spc_bb_d_mix;
use crate::cie1964::CIE1964;
use crate::sources::Spectra;
use crate::cie::XYZ;
use crate::cie1931::CIE1931;
use crate::filter::FilterSpectra;
use crate::ciecam::{CIECAM, ViewConditions};

#[derive(Debug, Serialize)]
pub struct CES {
	pub ciecam_dut: CIECAM,
	pub ciecam_ref: CIECAM,
	pub cct: (f64, f64),
}

#[derive(Debug, Serialize)]
pub struct Gamut {
	pub rg: f64,
	pub chroma: [f64;16],
	pub hue: [f64; 16],
	pub fidelity: [f64;16],
	pub cvg: ([f64;16],[f64;16],[f64;16], [f64;16]),
	pub jab_ref: Vec<[f64;3]>,
	pub jab_dut: Vec<[f64;3]>,
	pub cnt_ref: [usize; 16]
}

impl CES {

	pub fn from(sp: &Spectra) -> CES {
		let ltd = XYZ::from_sp(sp, &CIE1931).to_ltd()[0];
		let xyz_dut_w = XYZ::from_sp(sp, &CIE1964).scale_to_y(100.0);

		let sp_ref = spc_bb_d_mix(ltd[1], 4000.0, 5000., 1.0);
		let xyz_ref_w = XYZ::from_sp(&sp_ref, &CIE1964).scale_to_y(100.0);

		let xyz_dut = XYZ::from(sp, &ces_samples(), &CIE1964);
		let xyz_ref = XYZ::from(&sp_ref, &ces_samples(), &CIE1964);

		let mut vc = ViewConditions::default();
		vc.D_opt = Some(1.0);

		let ciecam_dut = CIECAM::from(xyz_dut, vc);
		let ciecam_ref = CIECAM::from(xyz_ref, vc);

		CES{
			ciecam_dut,
			ciecam_ref,
			cct: (ltd[1],ltd[2]),
		}
	}



	// Ra: average over first 8
	pub fn rf(&self) -> (f64, Vec<f64>, Vec<[f64;3]>, Vec<[f64;3]>) {
		let mut de_sum = 0.0;
		
		let jab_dut = self.ciecam_dut.jab_p();
		let jab_ref = self.ciecam_ref.jab_p();
		let mut rfi: Vec<f64> = Vec::with_capacity(jab_dut.len());
		for ([j_dut,a_dut, b_dut],[j_ref, a_ref, b_ref]) in jab_dut.iter().zip(jab_ref.iter()) {
			let de = ((j_dut-j_ref).powi(2) + (a_dut-a_ref).powi(2) + (b_dut-b_ref).powi(2)).sqrt();
//			println!("{:?}", (j_dut, a_dut, b_dut, j_ref, a_ref, b_ref, de));
			de_sum += de;
			rfi.push(rf_from_de(de));
		}
		(rf_from_de(de_sum/99.0), rfi, jab_dut, jab_ref)
	}



	// R9, red rendering
	pub fn gamut(&self) -> Gamut {

		let mut angle_ref_vec = [0.0f64; 16];
		let mut j_ref_vec = [0.0f64; 16];
		let mut a_ref_vec = [0.0f64; 16];
		let mut b_ref_vec = [0.0f64; 16];
		let mut j_dut_vec = [0.0f64; 16];
		let mut a_dut_vec = [0.0f64; 16];
		let mut b_dut_vec = [0.0f64; 16];
		let mut cnt_ref : [usize;16] = [0; 16];
		let mut de_vec : [f64; 16] = [0.0; 16];


		const TO_BIN : f64 = 180.0 / (PI * 22.5);

		let jab_dut = self.ciecam_dut.jab_p();
		let jab_ref = self.ciecam_ref.jab_p();
		for ([j_dut ,a_dut, b_dut],[j_ref, a_ref, b_ref]) in jab_dut.iter().zip(jab_ref.iter()) {
			let mut angle = b_ref.atan2(*a_ref);
			if angle<0.0 { angle += 2.0 * PI; }
			let angle_ref = (angle * TO_BIN).floor() as usize;
			angle_ref_vec[angle_ref] += angle;
			j_ref_vec[angle_ref] += j_ref;
			a_ref_vec[angle_ref] += a_ref;
			b_ref_vec[angle_ref] += b_ref;
			j_dut_vec[angle_ref] += j_dut;
			a_dut_vec[angle_ref] += a_dut;
			b_dut_vec[angle_ref] += b_dut;
			cnt_ref[angle_ref] += 1;
			de_vec[angle_ref] += ((j_dut-j_ref).powi(2) + (a_dut-a_ref).powi(2)+ (b_dut-b_ref).powi(2)).sqrt();
		}

		for i in 0..16 {
			angle_ref_vec[i] /= cnt_ref[i] as f64;
			j_ref_vec[i] /= cnt_ref[i] as f64;
			a_ref_vec[i] /= cnt_ref[i] as f64;
			b_ref_vec[i] /= cnt_ref[i] as f64;
			j_dut_vec[i] /= cnt_ref[i] as f64;
			a_dut_vec[i] /= cnt_ref[i] as f64;
			b_dut_vec[i] /= cnt_ref[i] as f64;
			de_vec[i] /= cnt_ref[i] as f64;
		}


		let mut chroma : [f64;16] = [0.0; 16];
		let mut hue : [f64;16] = [0.0; 16];
		let mut fidelity : [f64;16] = [0.0; 16];
		let mut cvg_x : [f64;16] = [0.0; 16];
		let mut cvg_y : [f64;16] = [0.0; 16];
		let mut cvg_x_ref : [f64;16] = [0.0; 16];
		let mut cvg_y_ref : [f64;16] = [0.0; 16];

		let mut jab_ref : Vec<[f64;3]> = Vec::with_capacity(16);
		let mut jab_dut : Vec<[f64;3]> = Vec::with_capacity(16);
		for i in 0..16 {
			let theta =b_ref_vec[i].atan2(a_ref_vec[i]); 
			let nom = (a_ref_vec[i].powi(2) + b_ref_vec[i].powi(2)).sqrt();
			cvg_x_ref[i] = angle_ref_vec[i].cos();
			cvg_y_ref[i] = angle_ref_vec[i].sin();
			cvg_x[i] = cvg_x_ref[i] + (a_dut_vec[i]-a_ref_vec[i])/(a_ref_vec[i].powi(2)+b_ref_vec[i].powi(2)).sqrt();
			cvg_y[i] = cvg_y_ref[i] + (b_dut_vec[i]-b_ref_vec[i])/(a_ref_vec[i].powi(2)+b_ref_vec[i].powi(2)).sqrt();
			chroma[i] = 
				(
					(a_dut_vec[i]-a_ref_vec[i]) * theta.cos()  
					+ (b_dut_vec[i]-b_ref_vec[i]) * theta.sin()  
				) / nom;

			hue[i] = 
				(
					- (a_dut_vec[i]-a_ref_vec[i]) * theta.sin()  
					+ (b_dut_vec[i]-b_ref_vec[i]) * theta.cos()  
				) / nom;

			fidelity[i] =  rf_from_de(de_vec[i]);
			jab_ref.push([j_ref_vec[i], a_ref_vec[i], b_ref_vec[i]]);
			jab_dut.push([j_dut_vec[i], a_dut_vec[i], b_dut_vec[i]]);
		}



		//println!("{:#?}", (&a_ref_vec, &b_ref_vec, &cnt_ref_vec, &a_dut_vec, &b_dut_vec));
		
		Gamut {
			rg: 100.0 * polygon_area(a_dut_vec.to_vec(), b_dut_vec.to_vec()) / polygon_area(a_ref_vec.to_vec(), b_ref_vec.to_vec()),
			chroma,
			hue,
			fidelity,
			cvg: (cvg_x_ref, cvg_y_ref, cvg_x, cvg_y),
			jab_ref,
			jab_dut,
			cnt_ref,
		}
	}
}

#[inline]
pub fn rf_from_de(de: f64) -> f64 {
	const CF: f64 = 6.73; 
	//const CF: f64 = 7.54; // TM30-15 value
	10.0 * (
		(
			(100.0 - CF * de)/10.0
		).exp() + 1.0
	).ln()
}


#[test]
fn test_cie224(){
	use crate::fluorescent::cie_f;

	let f3 = cie_f();
	for i in 0..1{
		let fs = f3.get(i);
		let ces = CES::from(&fs);
		println!{"To do! in ces.rs"};
	//	println!{"{:#?}", ces.jab_ref};
	//	println!{"{:#?}", ces.rf()};
	//	println!{"{:#?}", ces.gamut()};
	}
	
}
*/