/*!
   `CieCamUcs`, the CIECAM Uniform Color Space model, using J', a', and b' correlates.
*/

use super::{CieCamEnv, CieLab, CieXYZ, VcAvg};
use crate::{DefaultObserver, illuminants::D65, models::xyz_to_lab, observers::{StandardObserver}};
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

        /**
        Inverse Transform, Back To `Cielab<I,C>`.

        This follows the procedure as outlined by Luo, Appendix A, Part 2: The Reverse Mode.
        Consumes (moves) CieCamJch, and overwrites data in wrapper.
    */
    pub fn into_cielab(mut self) -> CieLab<I, C>
    where
        V: Default + Into<CieCamEnv<I, C>>,
        I: Default + Into<CieXYZ<C>>,
        C: StandardObserver,
    {
        // Can not use: impl<V,I,C> From<CieCamJCh<V,I,C>> for CieLab<I,C>
        // gets into a T From T error

        let cam: CieCamEnv<I, C> = V::default().into();
        for mut lab_p in self.data.column_iter_mut(){
            let [j_p,a_p,b_p]: &mut [f64;3] = lab_p.as_mut();
            let [x,y,z] = cam.ucs_lab_into_xyz(*j_p, *a_p, *b_p);
            *j_p = x; *a_p = y; *b_p = z; // overwrite data
        }
        // move data into CieLab container after calculating lab values
        let xyz_n: CieXYZ<C> = I::default().into();
        CieLab::<I, C>::new(xyz_to_lab(xyz_n.data.column(0), self.data))
    }
}

impl<V, I, C, L> From<L> for CieCamUcs<V, I, C>
where
    I: Default + Into<CieXYZ<C>>,
    L: Into<CieLab<I, C>>,
    C: StandardObserver,
    V: Default + Into<CieCamEnv<I, C>>,
{
    fn from(samples: L) -> Self {
        // Calculate Viewing Environment Parameters
        let view: CieCamEnv<I, C> = V::default().into();
        let lab: CieLab<I, C> = samples.into();
        let mut m_xyz: CieXYZ<C> = lab.into();
        for mut xyz in m_xyz.data.column_iter_mut(){
            let [x,y,z]: &mut [f64;3] = xyz.as_mut();
            let [j,a,b] = view.xyz_into_ucs_jab(*x, *y, *z);
            *x = j; *y = a; *z = b;
        }
        Self::new(m_xyz.data) // move data in CieCamCucs container
    }
}

// For forward test see test_tm30_data.rs in colorado-tm30: ces_us

#[test]
/**
    Test Reverse CieCamUcs By Round Trip 
    `CieLab<D50,CieObs1931>` -> `CieCamUcs<VcAvg, D50, CieObs1931` -> `CieLab<D50,CieObs1931>`
    For A Set of CieLab Test Values
 */
fn test_reverse_ucs_jab(){
    use crate::illuminants::D50; 
    use nalgebra::Matrix3xX;
    use crate::observers::CieObs1931;
    use approx::assert_abs_diff_eq;

    let lab: CieLab<D50,CieObs1931> = CieLab::new(Matrix3xX::from_vec(vec![
        0.0,   0.0,    0.0,
        1.0,   10.0,   0.0,
        1.0,   10.0,  10.0,
        50.0,    0.0,  0.0, 
        50.0,  -20.0,  20.0, 
        50.0,   20.0, -20.0, 
        50.0,  -20.0, -20.0, 
        100.0, 100.0,    0.0, 
        100.0,   0.0,  100.0, 
        100.0,   0.0, -100.0, 
        100.0, 100.0, -100.0,
    ]));
    let ucs: CieCamUcs<VcAvg, D50, CieObs1931> = lab.clone().into();
    let lab_calc = ucs.into_cielab();
    println!("{:.4}", lab_calc.data.transpose());
    lab_calc.data
        .iter()
        .zip(lab.data.iter())
        .for_each(|(&v, &w)| assert_abs_diff_eq!(v, w, epsilon = 1E-6));

}

