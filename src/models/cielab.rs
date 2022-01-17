/*!

    A collection of a Lab values, associated with a standard observer and a reference white illuminant.

*/

use std::marker::PhantomData;

use crate::{illuminants::D65, observers::StandardObserver, DefaultObserver};
use nalgebra::{Matrix3x1, Matrix3xX};

use super::CieXYZ;

#[derive(Debug, Clone)]
pub struct CieLab<I = D65, C = DefaultObserver> {
    pub data: Matrix3xX<f64>,
    cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
    illuminant: PhantomData<*const I>, // only used through I:Default(), but needed to mark the type
}

impl<I, C> CieLab<I, C> {
    pub fn new(data: Matrix3xX<f64>) -> Self {
        Self {
            data,
            cmf: PhantomData,
            illuminant: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.ncols()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> LabIterRef<I, C> {
        self.into_iter()
    }
}


impl<I,C, const M:usize> From<[[f64; 3]; M]>  for CieLab<I,C> {
    fn from(m: [[f64;3];M]) -> Self {
        let data = Matrix3xX::<f64>::from_fn(M, |i,j| m[j][i]);
        Self::new(data)
    }
}

impl<I,C, const M:usize> From<&[[f64; 3]; M]>  for CieLab<I,C> {
    fn from(m: &[[f64;3];M]) -> Self {
        let data = Matrix3xX::<f64>::from_fn(M, |i,j| m[j][i]);
        Self::new(data)
    }
}

#[test]
fn test_cielab_from_array(){
    let lab : CieLab = [[50.0,20.0,0.0], [100.0, 0.0, 30.0]].into();
    println!("{}", lab.data);
}

impl<I, C> From<CieLab<I, C>> for CieXYZ<C>
where
    C: StandardObserver,
    I: Default,
    I: Into<CieXYZ<C>>,
{
    fn from(lab: CieLab<I, C>) -> Self {
        let xyz_n: CieXYZ<C> = I::default().into();
        Self::new(lab_to_xyz(xyz_n.data.column(0), lab.data))
    }
}

#[test]
fn test_lab_to_xyz() {
    use crate::illuminants::D50;
    use crate::observers::CieObs1931Classic;
    use approx::assert_abs_diff_eq;
    use nalgebra::OMatrix;
    let lab =  CieLab::<D50, CieObs1931Classic>::from([
        [100.0, 100.0, -100.0], 
        [100.0, 50.0, 0.0],
        [100.0, 0.0, 50.0], 
        [0.0, 0.0, 0.0], 
        [20.0, 100.0, -50.0]
    ]);

    // CIECAM02.XLS spreadsheet, with XYZ_W [96.42150208438176, 100.0, 82.52098537603804], as I get with
    // my D50 CieObsClassic calculation
    let want = OMatrix::<f64, _, _>::from([
        [166.6163556, 100.0, 278.5083256],
        [128.3370193, 100.0, 82.52098538],
        [96.42150208, 100.0, 34.81354071],
        [0.0, 0.0, 0.0],
        [12.81637025, 2.989052442, 14.5187928],
    ]);

    let a_lab = CieXYZ::<CieObs1931Classic>::from(lab);
    for (c, w) in a_lab.data.iter().zip(want.iter()) {
        assert_abs_diff_eq!(c, w, epsilon = 1E-7); // abs<1.E-3 or rel<5E-4
    }
}

pub struct LabIter<I, C> {
    lab: CieLab<I, C>,
    i: usize,
}

pub struct LabIterRef<'a, I, C> {
    lab: &'a CieLab<I, C>,
    i: usize,
}

impl<C, I> Iterator for LabIter<I, C> {
    type Item = LabValues;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.lab.data.ncols() {
            let l = self.lab.data[(0, self.i)];
            let a = self.lab.data[(1, self.i)];
            let b = self.lab.data[(2, self.i)];
            self.i += 1;
            Some(LabValues { l, a, b })
        } else {
            None
        }
    }
}

impl<'a, C, I> Iterator for LabIterRef<'a, I, C> {
    type Item = LabValues;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.lab.data.ncols() {
            let l = self.lab.data[(0, self.i)];
            let a = self.lab.data[(1, self.i)];
            let b = self.lab.data[(2, self.i)];
            self.i += 1;
            Some(LabValues { l, a, b })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LabValues {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl<C, I> IntoIterator for CieLab<I, C> {
    type Item = LabValues;

    type IntoIter = LabIter<I, C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { lab: self, i: 0 }
    }
}

impl<'a, C, I> IntoIterator for &'a CieLab<I, C> {
    type Item = LabValues;

    type IntoIter = LabIterRef<'a, I, C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { lab: self, i: 0 }
    }
}


const DELTA: f64 = 24f64 / 116f64;
const DELTA_POW2: f64 = DELTA * DELTA;
const DELTA_POW3: f64 = DELTA_POW2 * DELTA;
const LABPOW: f64 = 1f64 / 3f64;
const LABC1: f64 = 1f64 / (3f64 * DELTA_POW2);
const LABC2: f64 = 4f64 / 29f64;

fn lab_f(t: f64) -> f64 {
    if t > DELTA_POW3 {
        t.powf(LABPOW)
    } else {
        LABC1 * t + LABC2
    }
}

fn lab_finv(t: f64) -> f64 {
    if t > DELTA {
        t.powi(3)
    } else {
        3f64 * DELTA_POW2 * (t - LABC2)
    }
}

// deprecated, use xyz_to_lab instead
pub fn cielab(xyz_n: Matrix3x1<f64>, xyz: Matrix3xX<f64>) -> Matrix3xX<f64> {
    let mut m: Matrix3xX<f64> = Matrix3xX::from_fn(xyz.ncols(), |i, j| xyz[(i, j)] / xyz_n[(i, 0)]);
    for mut xyz in m.column_iter_mut() {
        let x = xyz.x;
        let y = xyz.y;
        let z = xyz.z;
        xyz.x = 116f64 * lab_f(y) - 16f64;
        xyz.y = 500f64 * (lab_f(x) - lab_f(y));
        xyz.z = 200f64 * (lab_f(y) - lab_f(z));
    }
    m
}

/**
   In place tranformation from CieXYZ to CieLab values.
*/
//pub fn xyz_to_lab(xyz_n: MatrixSlice3x1<f64>, mut xyz: Matrix3xX<f64>) -> Matrix3xX<f64> {
pub fn xyz_to_lab(xyz_n: impl AsRef<[f64;3]>, mut xyz: Matrix3xX<f64>) -> Matrix3xX<f64> {
    let &[xn, yn, zn]: &[f64; 3] = xyz_n.as_ref();
    xyz.column_iter_mut().for_each(|mut xyz_j| {
        let [x, y, z]: &mut [f64; 3] = xyz_j.as_mut();
        let yyn = *y / yn;
        *y = 500f64 * (lab_f(*x / xn) - lab_f(yyn));
        *x = 116f64 * lab_f(yyn) - 16f64;
        *z = 200f64 * (lab_f(yyn) - lab_f(*z / zn));
    });
    xyz
}

/**
   In place transformation from CieLab to CieXYZ data values.

   See [CIELAB Color Space on Wikipedia](https://en.wikipedia.org/wiki/CIELAB_color_space)
*/
//pub fn lab_to_xyz(xyz_n: MatrixSlice3x1<f64>, mut lab: Matrix3xX<f64>) -> Matrix3xX<f64> {
pub fn lab_to_xyz(xyz_n: impl AsRef<[f64;3]>, mut lab: Matrix3xX<f64>) -> Matrix3xX<f64> {
    let &[xn, yn, zn]: &[f64; 3] = xyz_n.as_ref();
    lab.column_iter_mut().for_each(|mut lab_j| {
        let [l, a, b]: &mut [f64; 3] = lab_j.as_mut();
        let s = (*l + 16f64) / 116f64;
        // Y normalized to 100, independent of actual value of yn
        *l = 100.0 * xn / yn * lab_finv(s + *a / 500f64); // X
        *a = 100.0 * lab_finv(s); // Y
        *b = 100.0 * zn / yn * lab_finv(s - *b / 200f64); // Z
    });
    lab
}
