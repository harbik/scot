/*!
   `CieCamJCh` color appearance type, using Lightness (J), Chroma (C), and hue angle (h) correlates.

*/

use super::{cam::CieCam, CieCamEnv, CieLab, CieXYZ, VcAvg};
use crate::{illuminants::D65, models::xyz_to_lab, observers::StandardObserver, DefaultObserver};
use nalgebra::{Const, Dynamic, OMatrix};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct CieCamJCh<V = VcAvg, I = D65, C = DefaultObserver> {
    pub data: OMatrix<f64, Const<3>, Dynamic>,
    v: PhantomData<*const V>,
    i: PhantomData<*const I>,
    c: PhantomData<*const C>,
}
impl<V, I, C> CieCamJCh<V, I, C> {
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
        Transforms `CieCamJCh<V,I,C>` in to `Cielab<I2,C>`, with view conditions V2, and reference white illuminant I2.
        Does not (and cannot, here, as spectraldata is unavailable here) change the observer.

        This follows the procedure as outlined by Luo, Appendix A, Part 2: The Reverse Mode.
        Consumes (moves) CieCamJch, and overwrites data in wrapper.

    */
    pub fn into_cielab<V2, I2>(mut self) -> CieLab<I2, C>
    where
        V2: Default + Into<CieCamEnv<I2, C>>,
        I2: Default + Into<CieXYZ<C>>,
        C: StandardObserver,
    {
        // Can not use: impl<V,I,C> From<CieCamJCh<V,I,C>> for CieLab<I,C>
        // gets into a cyclical T From T error

        let cam: CieCamEnv<I2, C> = V2::default().into();
        let xyz_n: CieXYZ<C> = I2::default().into();
        for mut jch in self.data.column_iter_mut(){
            let [j,c,h]: &mut [f64;3] = jch.as_mut();
            let [x,y,z] = cam.jch_into_xyz(*j, *c, *h);
            *j = x; *c = y; *h = z; // overwrite data
        }
        // move data into CieLab container after calculating lab values
        CieLab::<I2, C>::new(xyz_to_lab(xyz_n.data.column(0), self.data))
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
        let mut m_xyz: CieXYZ<C> = lab.into();
        for mut xyz in m_xyz.data.column_iter_mut() {
            let [x,y,z]: &mut [f64;3] = xyz.as_mut();
            let [j,c,h, ..] = cam.xyz_into_jchab(*x, *y, *z);
            *x = j; *y=c; *z=h;
        }
        Self::new(m_xyz.data)
    }
}

#[test]
/**
   Test forward `CieCamJch` transform from  `CieLab` to `CieCamJCh` to `CieLab` coordinates,
   using JCh's `From<L>` or `From<impl Into<CieLab<I,C>>` implementation, for a set of 9 Jch and Lab values.

   The test data are created using [CIECAM02.XLS](https://web.archive.org/web/20070109143710/http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS)
   spreadsheet, created by Eric Walowit and Grit O'Brien, with a revision data of July 28, 2004.
*/
fn test_from_lab_for_ciecam_jch() {
    use super::{ViewConditions, D_AUTO, SR_AVG};
    use crate::illuminants::D50;
    use crate::observers::CieObs1931;
    use approx::assert_relative_eq;
    use nalgebra::Matrix3xX;
    let lab: CieLab<D50> = CieLab::new(Matrix3xX::from_vec(vec![
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
    let cam: CieCamJCh<ViewConditions<32, 20, SR_AVG, D_AUTO>, D50, CieObs1931> = lab.into();
    // From ciecam02.xls by Eric Walowit and Grit O'Brien <https://web.archive.org/web/20070109143710/http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS>
    // see also cielab.xyz, calculated using XYZ<sub>W</sub>=[96.42150, 100.0, 82.52099]
    let want = OMatrix::<f64, Const<3>, Dynamic>::from_vec(vec![
         0.0000,   0.0000, 180.0000, 
         2.6795,  40.8822,   0.7837,
         2.4800, 394.7046,  43.9344,
        39.6135,   1.1042, 112.5431,
        38.8666,  28.6425, 135.8441, 
        40.3782,  28.4553, 315.5517, 
        38.6828,  37.2596, 218.5544, 
       106.2265, 100.2295,   1.9929,
        99.7506,  79.5249,  95.0197, 
        98.2934,  97.9738, 248.1272, 
       105.4703, 105.6546, 312.9581,
    ]);
    println!("{:.4}", cam.data.transpose());
    for (c, w) in cam.data.iter().zip(want.iter()) {
        assert_relative_eq!(c, w, epsilon = 1E-4, max_relative = 5E-4); // abs<1.E-3 or rel<5E-4
    }
}

#[test]
/**
   Test reverse transform from `CieCamJCh` to `CieLab` coordinates, using JCh's into_cielab method,
   for a set of 9 Jch and Lab values.

   The test data are created using [CIECAM02.XLS](https://web.archive.org/web/20070109143710/http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS)
   spreadsheet, created by Eric Walowit and Grit O'Brien, with a revision data of July 28, 2004.

*/
fn test_reverse() {
    use crate::illuminants::D50;
    use crate::observers::CieObs1931;
    use crate::models::VcDark;
    use approx::assert_abs_diff_eq;
    use nalgebra::Matrix3xX;
    let m_jch = Matrix3xX::<f64>::from_vec(vec![
        39.890206, 0.065758, 110.250459, 39.126848, 28.068355, 136.265379, 40.675788, 29.327191,
        314.544438, 38.972361, 37.709782, 220.145277, 0.000000, 0.000000, 180.000000, 106.171077,
        99.637157, 1.382338, 99.681887, 78.569894, 94.800651, 98.456360, 98.160627, 248.371519,
        105.577618, 105.394956, 312.434013,
    ]);
    let want = OMatrix::<f64, _, _>::from([
        [50.0, 0.0, 0.0],
        [50.0, -20.0, 20.0],
        [50.0, 20.0, -20.0],
        [50.0, -20.0, -20.0],
        [0.0, 0.0, 0.0],
        [100.0, 100.0, 0.0],
        [100.0, 0.0, 100.0],
        [100.0, 0.0, -100.0],
        [100.0, 100.0, -100.0],
    ]);

    let jch: CieCamJCh<VcAvg, D50, CieObs1931> = CieCamJCh::new(m_jch);
    let lab_self = jch.clone().into_cielab::<VcAvg, D50>();
    println!("{}", lab_self.data);
    let lab2 = jch.into_cielab::<VcDark, D65>();
    println!("{}", lab_self.data);
    println!("{}", lab2.data);
    lab_self.data
        .iter()
        .zip(want.iter())
        .for_each(|(&v, &w)| assert_abs_diff_eq!(v, w, epsilon = 4E-3));
}

/**
    Create Jch Values From A Set Of Ciecam Values.

    This borrows the CieCam data, and collects the JCh values in a new Jch container.
 */
impl<V, I, C> From<&CieCam> for CieCamJCh<V, I, C> {
    fn from(ciecam: &CieCam) -> Self {
        let mut vdata: Vec<f64> = Vec::with_capacity(3 * ciecam.len());
        for ciecam_data in ciecam.data.column_iter() {
            let &[lightness, _, _, _, chroma, _, _, hue_angle, _]: &[f64; 9] = ciecam_data.as_ref();
            vdata.append(&mut vec![lightness, chroma, hue_angle]);
        }
        let data = OMatrix::<f64, Const<3>, Dynamic>::from_vec(vdata);
        Self::new(data)
    }
}
