use std::{fmt::Display, marker::PhantomData};

use crate::{
    illuminants::{CctDuv, CctDuvValue, Planckian},
    observers::StandardObserver,
    DefaultObserver,
};
use nalgebra::Matrix3xX;

use super::{CieXYZ, XYZValues};

#[derive(Debug, Clone)]
pub struct CieYuv1960<C: StandardObserver = DefaultObserver> {
    pub data: Matrix3xX<f64>,
    _cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieYuv1960<C> {
    pub fn new(data: Matrix3xX<f64>) -> Self {
        Self {
            data,
            _cmf: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.ncols()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}


pub(crate) fn uv60(x: f64, y: f64, z: f64) -> [f64; 3] {
    let den = x + 15.0 * y + 3.0 * z;
    [y, 4.0 * x / den, 6.0 * y / den]
}

impl<C, X> From<X> for CieYuv1960<C>
where
    C: StandardObserver,
    X: Into<CieXYZ<C>>,
{
    fn from(x: X) -> Self {
        let m: CieXYZ<C> = x.into();

        let mut v: Vec<f64> = Vec::with_capacity(m.data.len());
        for XYZValues { x, y, z } in m {
            //	let den = x + 15.0 * y + 3.0 * z;
            //	v.push(y);
            //	v.push(4.0 * x / den);
            //	v.push(6.0 * y / den);
            uv60(x, y, z).iter().for_each(|a| v.push(*a));
        }
        Self::new(Matrix3xX::<f64>::from_vec(v))
    }
}

#[allow(dead_code)]
pub(crate) fn uv_from_cct_duv<C: StandardObserver>(cct: f64, duv: f64) -> (f64, f64) {
    let CieYuv1960Values { y: _, u: u0, v: v0 } = CieYuv1960::<C>::from(Planckian::new(cct))
        .into_iter()
        .next()
        .unwrap();
    let CieYuv1960Values { y: _, u: u1, v: v1 } = CieYuv1960::<C>::from(Planckian::new(cct + 0.01))
        .into_iter()
        .next()
        .unwrap();
    let du = u0 - u1;
    let dv = v0 - v1;
    let hyp = du.hypot(dv);
    (u0 - dv * duv / hyp, v0 + du * duv / hyp) // see Ohno, Leukos, Practical Use and Calculation of CCT and DUV
}

impl<C> From<CctDuv<C>> for CieYuv1960<C>
where
    C: StandardObserver,
{
    fn from(tds: CctDuv<C>) -> Self {
        let mut mv: Vec<f64> = Vec::with_capacity(3 * tds.len());
        for CctDuvValue { t, d } in tds {
            let (u, v) = uv_from_cct_duv::<C>(t, d);
            mv.push(1.0); // y
            mv.push(u);
            mv.push(v);
        }
        Self::new(Matrix3xX::<f64>::from_vec(mv))
    }
}

#[test]
fn test_from_cctduv() {
    use crate::observers::CieObs1931;
    let tds: CctDuv<CieObs1931> = CctDuv::new(vec![[3000.0, 0.0], [3000.0, 0.01], [3000.0, -0.01]]);
    let yuv: CieYuv1960<_> = tds.into();
    println!("{}", yuv);
}

impl<C: StandardObserver> Display for CieYuv1960<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Yuv 1960<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct Yuv1960Iter<C: StandardObserver> {
    yuv: CieYuv1960<C>,
    i: usize,
}

pub struct Yuv1960IterRef<'a, C: StandardObserver> {
    yuv: &'a CieYuv1960<C>,
    i: usize,
}

#[derive(Debug)]
pub struct CieYuv1960Values {
    pub y: f64,
    pub u: f64,
    pub v: f64,
}

impl<C: StandardObserver> Iterator for Yuv1960Iter<C> {
    type Item = CieYuv1960Values;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.yuv.data.ncols() {
            let y = self.yuv.data[(0, self.i)];
            let u = self.yuv.data[(1, self.i)];
            let v = self.yuv.data[(2, self.i)];
            self.i += 1;
            Some(CieYuv1960Values { y, u, v })
        } else {
            None
        }
    }
}

impl<'a, C: StandardObserver> Iterator for Yuv1960IterRef<'a, C> {
    type Item = CieYuv1960Values;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.yuv.data.ncols() {
            let y = self.yuv.data[(0, self.i)];
            let u = self.yuv.data[(1, self.i)];
            let v = self.yuv.data[(2, self.i)];
            self.i += 1;
            Some(CieYuv1960Values { y, u, v })
        } else {
            None
        }
    }
}

impl<C: StandardObserver> IntoIterator for CieYuv1960<C> {
    type Item = CieYuv1960Values;
    type IntoIter = Yuv1960Iter<C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { yuv: self, i: 0 }
    }
}

impl<'a, C: StandardObserver> IntoIterator for &'a CieYuv1960<C> {
    type Item = CieYuv1960Values;
    type IntoIter = Yuv1960IterRef<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { yuv: self, i: 0 }
    }
}
