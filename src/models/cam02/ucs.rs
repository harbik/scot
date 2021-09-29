/*!
   `CieCamUcs`, the CIECAM Uniform Color Space model, using J', a', and b' correlates.
*/

use super::{CieCamEnv, CieLab, CieXYZ, VcAvg};
use crate::{DefaultObserver, illuminants::D65, observers::{StandardObserver}};
use nalgebra::{Const, Dynamic, OMatrix};
use std::{marker::PhantomData, };

pub struct CieCamUcs<V = VcAvg, I = D65, C = DefaultObserver> {
    pub data: OMatrix<f64, Const<3>, Dynamic>,
    v: PhantomData<*const V>,
    i: PhantomData<*const I>,
    c: PhantomData<*const C>,
}
impl<V, I, C> CieCamUcs<V, I, C> {
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<V, I, C, L> From<L> for CieCamUcs<V, I, C>
where
    I: Default + Into<CieXYZ<C>>,
    L: Into<CieLab<I, C>>,
    C: StandardObserver,
    V: Default + Into<CieCamEnv<I, C>>,
{
    /* 
    fn from(samples: L) -> Self {
        // Calculate Viewing Environment Parameters
        let view: CieCamEnv<I, C> = V::default().into();

        // Calculate XYZ values from CieLab input data
        let lab: CieLab<I, C> = samples.into();
        let n_samples = lab.len();
        let xyz: CieXYZ<C> = lab.into();
        let rgb_pa = view.post_adaptation_cone_response_from_xyz(xyz);

        // 3xX Matrix, CIECAM-UCS (J', a', b')
        let mut vdata: Vec<f64> = Vec::with_capacity(3 * n_samples);
        for rgb in rgb_pa.column_iter() {
            let achromatic_response = view.achromatic_response_mat_slice(rgb); // A
            let lightness = view.lightness(achromatic_response); // J
            let red_green = view.red_green_mat_slice(rgb); // a
            let blue_yellow = view.blue_yellow_mat_slice(rgb); // b
            let hue_angle = view.hue_angle(red_green, blue_yellow); // h
            let chroma = view.chroma_mat_slice(rgb, lightness, red_green, blue_yellow, hue_angle); // C
            let colorfulness = view.colorfulness(chroma); // M
            let j_prime = view.ucs_j_prime(lightness); // CIECAM UCS J'
            let (a_prime, b_prime) = view.ucs_ab_prime(colorfulness, hue_angle); // CIECAM UCS (a',b')
            vdata.append(&mut vec![j_prime, a_prime, b_prime]);
        }
        let data = OMatrix::<f64, Const<3>, Dynamic>::from_vec(vdata);
        Self::new(data)
    }
    */

    fn from(samples: L) -> Self {
        // Calculate Viewing Environment Parameters
        let view: CieCamEnv<I, C> = V::default().into();

        // Calculate XYZ values from CieLab input data
        let lab: CieLab<I, C> = samples.into();
   //     println!("{}", lab.data.transpose());
        let mut m_xyz: CieXYZ<C> = lab.into();
   //     println!("{}", m_xyz.data.transpose());

        for xyz in m_xyz.data.column_iter_mut(){
            view.transform_xyz_to_jab_prime(xyz);
        }
        Self::new(m_xyz.data)
    }
}

