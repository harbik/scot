/*!
	`CieCamJCh` color appearance type, using Lightness (J), Chroma (C), and hue angle (h) correlates.

 */

use super::{CieCamEnv, CieLab, CieXYZ, VcAvg, cam::CieCam};
use crate::{DefaultObserver, illuminants::D65, models::xyz_to_lab, observers::{StandardObserver}};
use nalgebra::{Const, Dynamic, OMatrix};
use std::marker::PhantomData;


pub struct CieCamJCh<V = VcAvg, I = D65, C = DefaultObserver> {
    pub data: OMatrix<f64, Const<3>, Dynamic>,
    v: PhantomData<*const V>,
    i: PhantomData<*const I>,
    c: PhantomData<*const C>,
}
impl<V, I, C> CieCamJCh<V, I, C> 
{
    pub fn new(data: OMatrix<f64, Const<3>, Dynamic>) -> Self {
        Self {
            data,
            i: PhantomData,
            c: PhantomData,
            v: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.ncols()
    }

	/**
		Inverse transform, back to `CieLab<I,C>`.

		This follows the procedure as outlined by Luo, Appendix A, Part 2: The Reverse Mode.
	*/
	pub fn into_cielab(mut self) -> CieLab<I,C> 
	where 
		V: Default + Into<CieCamEnv<I, C>>,
		I: Default + Into<CieXYZ<C>>,
		C: StandardObserver,
	{
		// Can not use: impl<V,I,C> From<CieCamJCh<V,I,C>> for CieLab<I,C> 
		// gets into a T From T error

        let cam: CieCamEnv<I, C> = V::default().into();
		self.data.column_iter_mut().for_each(|jch| cam.transform_jch_to_xyz(jch));
		let xyz_n: CieXYZ<C> = I::default().into();
		xyz_to_lab(xyz_n.data.column(0), &mut self.data);	
		CieLab::<I,C>::new(self.data)
    }
}



impl<V, I, C, L> From<L> for CieCamJCh<V, I, C>
where
    I: Default + Into<CieXYZ<C>>,
    L: Into<CieLab<I, C>>,
    C: StandardObserver,
    V: Default + Into<CieCamEnv<I, C>>,
{
    fn from(samples: L) -> Self {
        let cam: CieCamEnv<I, C> = V::default().into();

        // Calculate XYZ values from CieLab input data
        let lab: CieLab<I, C> = samples.into();
        let n_samples = lab.len();
        let xyz: CieXYZ<C> = lab.into();
        let rgb_pa = cam.post_adaptation_cone_response_from_xyz(xyz);

        // 3xX Matrix, CIECAM-JCh (J, C, h), Lightness, Chroma, and hue angle.
        let mut vdata: Vec<f64> = Vec::with_capacity(3 * n_samples);
        for rgb in rgb_pa.column_iter() {
            let achromatic_response = cam.achromatic_response(rgb); // A
            let lightness = cam.lightness(achromatic_response); // J
            let red_green = cam.red_green(rgb); // a
            let blue_yellow = cam.blue_yellow(rgb); // b
            let hue_angle = cam.hue_angle(red_green, blue_yellow); // h
            let chroma = cam.chroma(rgb, lightness, red_green, blue_yellow, hue_angle); // C
            vdata.append(&mut vec![lightness, chroma, hue_angle]);
        }
        let data = OMatrix::<f64, Const<3>, Dynamic>::from_vec(vdata);
        Self::new(data)
    }
}

impl<V,I,C> From<&CieCam> for CieCamJCh<V,I,C> {
    fn from(ciecam: &CieCam) -> Self {
        let mut vdata: Vec<f64> = Vec::with_capacity(3 * ciecam.len());
		for ciecam_data in ciecam.data.column_iter() {
			let &[lightness, _, _, _, chroma, _, _, hue_angle, _ ]: &[f64;9] = ciecam_data.as_ref();
			vdata.append(&mut vec![lightness, chroma, hue_angle]);
		}
        let data = OMatrix::<f64, Const<3>, Dynamic>::from_vec(vdata);
        Self::new(data)
    }
}


#[test]
fn test_from_lab(){
	use crate::observers::CieObs1931;
	use approx::assert_relative_eq;
	use crate::illuminants::D50;
	use nalgebra::Matrix3xX;
	use super::{SR_AVG, D_AUTO, ViewConditions};
	let lab: CieLab<D50> = CieLab::new(Matrix3xX::from_vec(vec![
		50.0, 0.0, 0.0,
		50.0, -20.0, 20.0,
		50.0, 20.0, -20.0,
		50.0, -20.0, -20.0,
		0.0, 0.0, 0.0,
		100.0, 100.0, 0.0,
		100.0, 0.0, 100.0,
		100.0, 0.0, -100.0,
		100.0, 100.0, -100.0,
	]));
	let cam: CieCamJCh<ViewConditions<32, 20, SR_AVG, D_AUTO>, D50, CieObs1931> = lab.into();
	// From ciecam02.xls by Eric Walowit and Grit O'Brien <https://web.archive.org/web/20070109143710/http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS>
	// see also cielab.xyz
	let want = OMatrix::<f64,Const::<3>, Dynamic>::from_vec(vec![
		39.614, 1.104, 112.539,
		38.867, 28.643, 135.844,
		40.378, 28.455, 315.552,
		38.683, 37.260, 218.554,
		0.000, 0.000, 180.000,
		106.226, 100.220, 1.993,
		99.751, 79.525, 95.020,
		98.294, 97.974, 248.127,
		105.470, 105.655, 312.958,
	]);
	//println!("{:.3}", cam.data.transpose());
	for (c,w) in cam.data.iter().zip(want.iter()){
		assert_relative_eq!(c, w, epsilon=1E-3, max_relative=5E-4); // abs<1.E-3 or rel<5E-4
	}
}

#[test]
fn test_reverse(){
	use nalgebra::Matrix3xX;
	use crate::illuminants::D50;
	use crate::observers::CieObs1931;
	use approx::assert_abs_diff_eq;
	let  m_jch = Matrix3xX::<f64>::from_vec(vec![
			39.890206,	0.065758,	110.250459,
			39.126848,	28.068355,	136.265379,
			40.675788,	29.327191,	314.544438,
			38.972361,	37.709782,	220.145277,
			0.000000,	0.000000,	180.000000,
			106.171077,	99.637157,	1.382338,
			99.681887,	78.569894,	94.800651,
			98.456360,	98.160627,	248.371519,
			105.577618,	105.394956,	312.434013,
	]);
	let want = OMatrix::<f64,_,_>::from([
		[50.0, 0.0, 0.0],
		[50.0, -20.0, 20.0],
		[50.0, 20.0, -20.0],
		[50.0, -20.0, -20.0],
		[0.0, 0.0, 0.0],
		[100.0, 100.0, 0.0],
		[100.0, 0.0, 100.0],
		[100.0, 0.0, -100.0],
		[100.0, 100.0, -100.0]
]);

	let jch: CieCamJCh<VcAvg, D50, CieObs1931> = CieCamJCh::new(m_jch);
	let xyz = jch.into_cielab();
//	println!("{}", xyz.data);
	xyz.data.iter().zip(want.iter()).for_each(|(&v,&w)|assert_abs_diff_eq!(v,w,epsilon=4E-3));

}