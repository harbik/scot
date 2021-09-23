/*!
    A collection of a Lab values, associated with a standard observer and a reference white illuminant.

*/

use std::{marker::PhantomData};

use crate::{DefaultObserver, illuminants::D65, observers::StandardObserver};
use nalgebra::{Matrix3x1, Matrix3xX};

use super::{CieXYZ, XYZValues};

#[derive(Debug,Clone)]
pub struct CieLab<I = D65, C = DefaultObserver> {
    pub data: Matrix3xX<f64>,
    cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
    illuminant: PhantomData<*const I>, // only used through I:Default(), but needed to mark the type
}

impl<I, C> CieLab<I, C>
{
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

    pub fn iter(&self) -> LabIterRef<I, C> {
        (&self).into_iter()
    }
}

impl<I, C> From<CieLab<I, C>> for CieXYZ<C>
where
	C: StandardObserver,
    I: Default,
	I: Into<CieXYZ<C>>,
{
	// Scaled to Yn = 100
    fn from(lab: CieLab<I, C>) -> Self {
        let ill = I::default();
		let xyz: CieXYZ<C> = ill.into();
        let XYZValues { x: xn, y: yn, z: zn, } = xyz.into_iter().next().unwrap();
		let mut v: Vec<f64> = Vec::with_capacity(lab.data.len());
        for LabValues { l, a, b } in lab {
            let s = (l + 16f64) / 116f64;
            v.push(100.0 * xn / yn * lab_finv(s + a / 500f64));
			v.push(100.0 * lab_finv(s));
			v.push(100.0 * zn / yn * lab_finv(s - b / 200f64));
        }
		Self::new(Matrix3xX::from_vec(v))
    }
}

/*

#[test]
fn test_lab_to_xyz(){
	use crate::illuminants::D65;
	use crate::observers::CieObs1931;
	use crate::swatches::ColorChecker;
	let a_lab = CieLab::<D65,CieObs1931>::from(ColorChecker::<13>);
//	let a_xyz= CieXYZ::<CieObs1931>::from(ColorChecker::<13>.values(Waveleng));
	let a_xyz_via_lab = CieXYZ::from(a_lab.clone());
	println!("{} {}", a_lab, a_xyz_via_lab);

}
*/

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
        Self::IntoIter { lab: &self, i: 0 }
    }
}


#[test]
fn test_lab_iter() {
    use crate::swatches::checker::ColorChecker;
    //use crate::observers::CieObs1931;
    use crate::illuminants::CieIllD50;

    let labs: CieLab<CieIllD50> = ColorChecker.into(); // using CieObs1931 and CieIllD65 as Default
    for LabValues { l, a, b } in labs {
        println!("{}, {}, {}", l, a, b);
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


