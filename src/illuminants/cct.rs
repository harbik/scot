/*!

Methods to calculate correlated color temperature, or CCT, for illuminants.

Thermal emission of a perfectly black object –  a "blackbody radiator" – is described by Planck's law,
with its thermodynamic temperature as parameter.
At relatively low temperatures it appears to be dark reddish; at higher temperatures it turns yellowish, and bright blueish white at even higher temperatures.

The term correlated color temperature, or CCT, is used to characterize emission of non-blackbody or gray body objects, and light sources which are not thermal radiators,
such as LED and Fluorescent lights.
A formal standard defining CCT, and recommendations for its calculation, does not seem to exist – it is defined by the CIE in its vocabulary:

> "Correlated color temperature is the color temperature corresponding to the point on
> the Planckian locus which is nearest to the point representing the chromaticity of the illuminant considered, on an
> agreed uniform-chromaticity-scale diagram."

And as a footnote it says:

> “The presently agreed uniform-chromaticity-scale diagram is the CIE 1960 UCS diagram."

To calculate the correlated color temperature for a source accurately is quite difficult, and many different methods exist.
Here the following methods are implemented:

# Robertson

This method is decribed by A.R. Robertson Robertson\[1968\], and was one of the first methods
to calculate correlated color temperatures for computer use.
In comparison with the other methods in this library it is fast, especially if the Robertson table has already been calculated.
Here this method can be used for any observer, including the CIE1931 standard observer, which is the one implemented by Robertson.

Drawbacks of this method are its limited accuracy –
approximately 2K for CCTs around 6500K, and larger for points further from the Planckian locus –
and limit in low temperature range: it fails for color temperatures less than 1667K.


# Ohno: 1% step table search with parabolic and linear interpolation

This is in a implementation of the method as described by Ohno\[2014\].
In this method a table of Planckian locus values, in form of CIE 1960 u and v chromaticity coordinates, is generated for a range
of temperature values distributed on a multiplicative scale.
The implementation here starts with a temperature of 1000K, and uses a multiplication factor of 1%, so the next values in this scale
are 1000K * 1.01 = 1010K, and 1010K * 1.01 = 1020.1K.
In total 303 values are generated, with a maximum of 20186.21K.

The algorithm is a basic brute force minimum search, with either triangular (|Duv|<0.002) or parabolic interpolation (otherwise) within the
interval containing the minimum values.
As concerned to the accuracy of this method, Ohno reports:

> The errors are mostly
> within 0.5 K in the 2000 to 20,000 K range and −0.03 < Duv < 0.03 and the maximum error is 0.8 K

Here is an example, calculating the cct and Duv values for the CIE FL1 illuminant:
```
    use colorado::illuminants::{Ohno2014, FL};
    use colorado::observers::CieObs1931;

    let ohno2014 = Ohno2014::<CieObs1931>::default();

    let cct = ohno2014.cct_duv(FL::<1>);
    println!("{}", cct.0);
```


# Ohno's cascade tables

Another minimum search algorithm, proposed by Ohno, is to use Planckian locus tables iteratively,
starting with a large 15% step table – a multiplication factor of 1.15 –
and continuing with finer step table (with 10 times smaller step size, 1.5%) centered around the found minimum.
This can be repeated a couple of times, getting to very small step size, and small errors.

To use Ohno's Cascade method to calculate the CCT and Duv values for the CIE FL1 illuminant:

```
    use colorado::illuminants::{Ohno2014Cascade, FL};
    use colorado::observers::CieObs1931;

    let ohno2014_cascade = Ohno2014Cascade::<CieObs1931>::default();

    let cct = ohno2014_cascade.cct_duv(FL::<1>);
    println!("{}", cct.0);
```


# References

- Ohno\[2014\]: Yoshi Ohno, Practical Use and Calculation of CCT and Duv, LEUKOS: The Journal of the Illuminating Engineering Society
of North America, 10:1, 47-55, DOI: 10.1080/15502724.2014.839020


*/
use core::panic;
use std::fmt::Display;
use std::{error::Error, marker::PhantomData};

use crate::models::yuv1960::{CieYuv1960, CieYuv1960Values};
use crate::observers::StandardObserver;
use crate::DefaultObserver;
use approx::AbsDiffEq;
use nalgebra::{DVector, Matrix2xX, Matrix3xX};

use super::Planckian;

/**
    Correlated color temperatures, CCT, and distances to a Planckian locus, Duv, for a collection of spectral sources.

    Output of the CctDuvCalc trait, encapsulating a matrix with two rows, the first row with the correlated color
    temperatures in Kelvin, and the second row distances to the Planckian, or Duv's, with positive values being
    above the Planckian, and negative values below the Planckian locus.
    For Duv's larger than 0.05, or CCTs below or above the covered range, `f64::NAN` values are reported.
*/
#[derive(PartialEq, Debug, Clone)]
pub struct CctDuv<C: StandardObserver>(Matrix2xX<f64>, PhantomData<*const C>);

impl<C: StandardObserver> CctDuv<C> {
    pub fn new(td: Vec<[f64; 2]>) -> Self {
        let mut mv: Vec<f64> = Vec::with_capacity(td.len() * 2);
        for [t, d] in td {
            mv.push(t);
            mv.push(d);
        }
        Self(Matrix2xX::from_vec(mv), PhantomData)
    }

    pub fn len(&self) -> usize {
        self.0.ncols()
    }
}

impl<C> AbsDiffEq for CctDuv<C>
where
    C: StandardObserver + PartialEq,
{
    type Epsilon = (f64, f64);

    fn default_epsilon() -> Self::Epsilon {
        (1.0, 1E-4)
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for (a, b) in self.0.column_iter().zip(other.0.column_iter()) {
            if (a.x - b.x).abs() > epsilon.0 || (a.y - b.y).abs() > epsilon.1 {
                return false;
            };
        }
        true
    }
}

pub struct CctDuvValue {
    pub t: f64,
    pub d: f64,
}

pub struct CctDuvIter {
    m: Matrix2xX<f64>,
    i: usize,
    n: usize,
}

impl Iterator for CctDuvIter {
    type Item = CctDuvValue;

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.i;
        if k < self.n {
            self.i += 1;
            let td = self.m.column(k);
            Some(CctDuvValue { t: td.x, d: td.y })
        } else {
            None
        }
    }
}

impl<C> IntoIterator for CctDuv<C>
where
    C: StandardObserver,
{
    type Item = CctDuvValue;

    type IntoIter = CctDuvIter;

    fn into_iter(self) -> Self::IntoIter {
        CctDuvIter {
            n: self.0.ncols(),
            m: self.0,
            i: 0usize,
        }
    }
}

pub trait CctDuvCalc {
    // Illuminant?
    type Observer: StandardObserver;

    fn cct_duv<U>(&self, uv: U) -> CctDuv<Self::Observer>
    where
        U: Into<CieYuv1960<Self::Observer>> //		S: SpectralData,
                                            //		Meter: From<<<S as SpectralData>::StepType as Step>::UnitValueType>
    ;
}

impl<C: StandardObserver> Display for CctDuv<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:7.5}", self.0)
    }
}

/**
# Robertson

In comparison with the other methods in this library it is fast, especially if the Robertson table has already been calculated,
which is done using:
```
    use colorado::illuminants::{Robertson, FL, CctDuvCalc};
    use colorado::observers::{CieObs1931};
    let robertson: Robertson<CieObs1931> = Robertson::new();

    let cct_duv_fl1 = robertson.cct_duv(FL::<1>);
    println!("Robertson {}", cct_duv_fl1);
```
 */
pub struct Robertson<C: StandardObserver = DefaultObserver>(Matrix3xX<f64>, PhantomData<*const C>);

impl<C: StandardObserver> Robertson<C> {
    pub fn new() -> Self {
        Robertson::<C>::default()
    }
}

impl<C> Default for Robertson<C>
where
    C: StandardObserver,
{
    fn default() -> Self {
        let mut rv: Vec<f64> = Vec::with_capacity(3 * ROBERTSON_MRD.len());
        for t in ROBERTSON_MRD {
            let (u, v, m) = robertson_normal::<crate::observers::CieObs1931>(1E6 / t);
            rv.push(u);
            rv.push(v);
            rv.push(m);
        }
        Self(Matrix3xX::from_vec(rv), PhantomData)
    }
}

impl<C: StandardObserver> CctDuvCalc for Robertson<C> {
    type Observer = C;

    fn cct_duv<U>(&self, uv: U) -> CctDuv<Self::Observer>
    where
        U: Into<CieYuv1960<Self::Observer>>,
    {
        let yuvs: CieYuv1960<C> = uv.into();
        let mut tdv: Vec<f64> = Vec::with_capacity(2 * yuvs.data.len());
        for CieYuv1960Values { y: _, u, v } in yuvs {
            let mut dm = 0f64;
            let mut di = 0f64;
            let mut ir = 0usize;
            for (i, uvl) in self.0.column_iter().enumerate() {
                let (ur, vr, t) = (uvl.x, uvl.y, uvl.z);
                di = (v - vr) - t * (u - ur);
                if i > 0 && (((di < 0.0) && (dm >= 0.0)) || ((di >= 0.0) && (dm < 0.0))) {
                    ir = i;
                    break;
                }
                dm = di;
            }
            if ir == 0usize {
                tdv.push(f64::NAN);
                tdv.push(f64::NAN);
            } else {
                di /= (1.0 + self.0[(2, ir)].powi(2)).sqrt();
                dm /= (1.0 + self.0[(2, ir - 1)].powi(2)).sqrt();
                let p = dm / (dm - di); // p interpolation parameter
                let t = (ROBERTSON_MRD[ir - 1] * (1.0 - p) + ROBERTSON_MRD[ir] * p).recip() * 1E6;
                tdv.push(t);
                let CieYuv1960Values { y: _, u: up, v: vp } =
                    CieYuv1960::<C>::from(Planckian::new(t))
                        .into_iter()
                        .next()
                        .unwrap();
                let d = (u - up).hypot(v - vp);
                if d > 0.05 {
                    tdv.push(f64::NAN)
                } else {
                    if v < vp {
                        tdv.push(-d)
                    } else {
                        tdv.push(d)
                    }
                }
            }
        }
        CctDuv(Matrix2xX::from_vec(tdv), PhantomData)
    }
}

#[allow(dead_code)]
fn robertson_normal<C: StandardObserver>(cct: f64) -> (f64, f64, f64) {
    let CieYuv1960Values { y: _, u, v } = CieYuv1960::<C>::from(Planckian::new(cct))
        .into_iter()
        .next()
        .unwrap();
    let CieYuv1960Values { y: _, u: u1, v: v1 } =
        CieYuv1960::<C>::from(Planckian::new(cct * 0.9999999))
            .into_iter()
            .next()
            .unwrap();
    let du = u - u1;
    let dv = v - v1;
    (u, v, -du / dv)
}

#[allow(dead_code)]
const ROBERTSON_MRD: [f64; 31] = [
    1.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 125.0, 150.0, 175.0, 200.0,
    225.0, 250.0, 275.0, 300.0, 325.0, 350.0, 375.0, 400.0, 425.0, 450.0, 475.0, 500.0, 525.0,
    550.0, 575.0, 600.0,
];

#[test]
fn robertson_normal_test() {
    use crate::illuminants::{CctDuvCalc, Robertson, FL};
    use crate::observers::{CieObs1931Classic, CieObsF10};

    let r: Robertson<CieObs1931Classic> = Robertson::new();
    println!("{:8.4}", r.0.transpose());

    let cct_duv_fl1 = r.cct_duv(FL::<1>);
    let CctDuvValue { t, d } = cct_duv_fl1.into_iter().next().unwrap();
    println!("Robertson {} {}", t, d);

    let r: Robertson<CieObsF10> = Robertson::new();
    //	println!("{:8.4}", r.0.transpose());
    let cct_duv_fl1 = r.cct_duv(FL::<1>);
    let CctDuvValue { t, d } = cct_duv_fl1.into_iter().next().unwrap();
    println!("Robertson F10 {} {}", t, d);
}

#[test]
fn test_robertson_from_cctduv() {
    use crate::illuminants::{CctDuvCalc, Robertson};
    use crate::observers::CieObs1931;
    use approx::assert_abs_diff_eq;

    let r: Robertson<CieObs1931> = Robertson::new();

    let tds: CctDuv<CieObs1931> = CctDuv::new(vec![
        [12000.0, 0.0],
        [12000.0, 0.01],
        [12000.0, -0.],
        [6500.0, 0.0],
        [6500.0, 0.01],
        [6500.0, -0.0499],
        [3000.0, 0.0],
        [3000.0, 0.01],
        [3000.0, -0.01],
        [1800.0, 0.0],
        [1667.0, 0.0],
    ]);
    let yuv: CieYuv1960<_> = tds.clone().into();

    let td_calc = r.cct_duv(yuv);
    assert_abs_diff_eq!(tds, td_calc, epsilon = (5.0, 0.000_01));
}
/**
    Multiplicative increasing temperature scale as used in Ohno's method
*/
#[derive(Clone, Copy)]
pub struct CctLadder {
    pub cct_min: f64,
    pub cct_mul: f64,
    pub imax: i32,
}

/**
    Using the 1% ladder, as described by Ohno\[2014\].
*/
impl Default for CctLadder {
    fn default() -> Self {
        Self {
            cct_min: 1000.0,
            cct_mul: 1.01,
            imax: 303,
        }
    }
}

impl CctLadder {
    /**
        Creates a multiplicative temperature scale from a start and an (inclusive) end temperature, and a muliplication factor.
        It includes and ranges beyond the given end temperature.
        Fails if `start` is larger or equal than `end`, or if the multiplication factor `mul` is less or equal than 1.0.
    */
    pub fn new(start: f64, end: f64, mul: f64) -> Self {
        if start <= 0.0 || end < start || mul <= 1.0 {
            panic!("CctLadder Error: end value should be larger than start value, and the multiplicaton factor should be larger than 1.0");
        }
        Self {
            cct_min: start,
            cct_mul: mul,
            imax: ((end / start).log10() / mul.log10()).ceil() as i32,
        }
    }

    pub fn cct(&self, i: i32) -> Result<f64, Box<dyn Error>> {
        if i < self.imax {
            Ok(self.cct_min * self.cct_mul.powi(i))
        } else {
            Err("Index out of range".into())
        }
    }
}

impl Iterator for CctLadder {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.imax > 0 {
            let t = self.cct_min;
            self.imax -= 1;
            self.cct_min = t * self.cct_mul;
            Some(t)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.imax as usize, Some(self.imax as usize))
    }
}

#[test]
fn test_cct() -> Result<(), Box<dyn Error>> {
    use approx::assert_abs_diff_eq;
    let vcct: Vec<f64> = CctLadder::default().into_iter().collect();
    //	println!("{:.3?}", vcct);
    //	println!("{:?}", vcct.len());
    assert_eq!(vcct.len(), 303);
    assert_abs_diff_eq!(CctLadder::default().cct(0)?, 1000.0, epsilon = 1E-3);
    assert_abs_diff_eq!(CctLadder::default().cct(302)?, 20186.21, epsilon = 5E-3);

    let l = CctLadder::new(1000.0, 1000.01, 1.01);
    assert_eq!(1, l.imax);

    let l = CctLadder::new(1000.0, 20186.22, 1.01);
    assert_eq!(303, l.imax);

    Ok(())
}

#[doc(hidden)]
/**
    A convenience object, used to implement various methods to calculate CCT and Duv's.
*/
struct PlanckianTable<C: StandardObserver>(Vec<f64>, CieYuv1960<C>);

impl<C> PlanckianTable<C>
where
    C: StandardObserver,
{
    fn new(l: Option<CctLadder>) -> Self {
        let tpv: Vec<f64> = l.unwrap_or_default().into_iter().collect();
        let cct_uv: CieYuv1960<C> = Planckian::new(tpv.clone()).into();
        Self(tpv, cct_uv)
    }

    /**
        Calculates distances between a test (u,v) point, and all the (u,v) points in the planckian table.
    */
    fn sq_distances(&self, u: f64, v: f64) -> DVector<f64> {
        let mut d2v: Vec<f64> = Vec::with_capacity(self.0.len());
        for CieYuv1960Values { y: _, u: ur, v: vr } in &self.1 {
            d2v.push((u - ur) * (u - ur) + (v - vr) * (v - vr));
        }
        DVector::from_vec(d2v)
    }

    fn triangular(&self, i: usize, duv2values: DVector<f64>) -> [f64; 2] {
        let tp = self.0[i - 1];
        let dp2 = duv2values[i - 1];
        let tn = self.0[i + 1];
        let dn2 = duv2values[i + 1];
        let l2 = (self.1.data[(1, i + 1)] - self.1.data[(1, i - 1)]).powi(2)
            + (self.1.data[(2, i + 1)] - self.1.data[(2, i - 1)]).powi(2);
        let l = l2.sqrt();
        let x = (dp2 - dn2 + l2) / (2. * l);
        let t = tp + (tn - tp) * x / l;
        let d = (dp2 - x * x).sqrt();
        [t, d]
    }

    fn parabolic(&self, i: usize, duv2values: DVector<f64>) -> [f64; 2] {
        let tp = self.0[i - 1];
        let t = self.0[i];
        let tn = self.0[i + 1];
        let dp = duv2values[i - 1].sqrt();
        let d = duv2values[i].sqrt();
        let dn = duv2values[i + 1].sqrt();
        let x = (tn - t) * (tp - tn) * (t - tp);
        let a = (tp * (dn - d) + t * (dp - dn) + tn * (d - dp)) / x;
        let b = -(tp * tp * (dn - d) + t * t * (dp - dn) + tn * tn * (d - dp)) / x;
        let c = -(dp * (tn - t) * t * tn + d * (tp - tn) * tp * tn + dn * (t - tp) * tp * t) / x;
        let tt = -b / (2. * a);
        // see 	Y. Ohno, Leukos Non uniformity effect CCT scale in uv coordinates, not needed if the step factor is less than 1.003
        [tt, a * tt * tt + b * tt + c]
    }

    fn ohno2014(&self, u: f64, v: f64) -> [f64; 2] {
        let d2m = self.sq_distances(u, v);
        let imin = d2m.imin();
        if imin < 1 || imin > self.0.len() - 2 {
            [f64::NAN, f64::NAN]
        } else {
            let [t, d] = if d2m[imin].sqrt() < 0.002 {
                self.triangular(imin, d2m)
            } else {
                self.parabolic(imin, d2m)
            };
            if v < self.1.data[(2, imin)] {
                [t, -d]
            } else {
                [t, d]
            }
        }
    }

    fn zoom(&self, u: f64, v: f64, mul: f64) -> CctLadder {
        let d2m = self.sq_distances(u, v);
        let imin = d2m.imin();
        CctLadder::new(self.0[imin - 1], self.0[imin + 1], mul)
    }
}

/*
fn cascade<C: StandardObserver>(u: f64, v: f64) -> [f64;2] {
    let pt = PlanckianTable::<C>::new(Some(CctLadder::new(1000.0, 32000.0, 1.0 + 0.15)));
    let pt2 = PlanckianTable::<C>::new(Some(pt.zoom(u, v, 1.0 + 0.015)));
    let pt3 = PlanckianTable::<C>::new(Some(pt2.zoom(u, v, 1.0 + 0.0015)));
    let pt4 = PlanckianTable::<C>::new(Some(pt3.zoom(u, v, 1.0 + 0.00015)));
    pt4.ohno2014(u, v) // correction here not needed, due to small step size
}
*/

impl<C> Default for PlanckianTable<C>
where
    C: StandardObserver,
{
    fn default() -> Self {
        Self::new(None)
    }
}

const OHNO_CORR_1PCT_STEP: f64 = 0.99991; // the somewhat 'magical' correction factor, as listed in Ohno's article for the 1% step table

#[derive(Default)]
pub struct Ohno2014<C: StandardObserver = DefaultObserver>(PlanckianTable<C>);

impl<C: StandardObserver> Ohno2014<C> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<C> CctDuvCalc for Ohno2014<C>
where
    C: StandardObserver,
{
    type Observer = C;
    fn cct_duv<U>(&self, uv: U) -> CctDuv<Self::Observer>
    where
        U: Into<CieYuv1960<Self::Observer>>,
    {
        //	let pt = PlanckianTable::<C>::new(None);
        let uvs_test: CieYuv1960<C> = uv.into();
        let mut mv: Vec<f64> = Vec::with_capacity(uvs_test.data.len());
        for CieYuv1960Values { y: _, u, v } in uvs_test {
            let [t, d] = self.0.ohno2014(u, v);
            mv.push(t * OHNO_CORR_1PCT_STEP);
            if d.abs() <= 0.05 {
                mv.push(d)
            } else {
                mv.push(f64::NAN)
            }; // out of range
        }
        CctDuv(Matrix2xX::<f64>::from_vec(mv), PhantomData)
    }
}

#[test]
fn test_ohno_trait() {
    use crate::illuminants::{Ohno2014, FL};
    use crate::observers::CieObs1931;

    let ohno = Ohno2014::<CieObs1931>::default();

    let cct = ohno.cct_duv(FL::<1>);
    println!("{}", cct.0);

    let ohno_cascade = Ohno2014Cascade::<CieObs1931>::default();
    let cct = ohno_cascade.cct_duv(FL::<1>);
    println!("{}", cct.0);
}

pub struct Ohno2014Cascade<C: StandardObserver = DefaultObserver>(PlanckianTable<C>);

impl<C: StandardObserver> Ohno2014Cascade<C> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<C> Default for Ohno2014Cascade<C>
where
    C: StandardObserver,
{
    fn default() -> Self {
        let pt = PlanckianTable::<C>::new(Some(CctLadder::new(1000.0, 32000.0, 1.0 + 0.15)));
        Self(pt)
    }
}

impl<C> CctDuvCalc for Ohno2014Cascade<C>
where
    C: StandardObserver,
{
    type Observer = C;
    fn cct_duv<U>(&self, uv: U) -> CctDuv<Self::Observer>
    where
        U: Into<CieYuv1960<Self::Observer>>,
    {
        let uvs_test: CieYuv1960<C> = uv.into();
        let mut mv: Vec<f64> = Vec::with_capacity(uvs_test.data.len());
        for CieYuv1960Values { y: _, u, v } in uvs_test {
            let pt2 = PlanckianTable::<C>::new(Some(self.0.zoom(u, v, 1.0 + 0.015)));
            let pt3 = PlanckianTable::<C>::new(Some(pt2.zoom(u, v, 1.0 + 0.0015)));
            let pt4 = PlanckianTable::<C>::new(Some(pt3.zoom(u, v, 1.0 + 0.00015)));
            let [t, d] = pt4.ohno2014(u, v); // correction here not needed, due to small step size
            mv.push(t);
            if d.abs() <= 0.05 {
                mv.push(d)
            } else {
                mv.push(f64::NAN)
            }; // out of range
        }
        CctDuv(Matrix2xX::<f64>::from_vec(mv), PhantomData)
    }
}

#[test]
fn test_ohno() {
    use approx::assert_abs_diff_eq;

    for [t, d] in vec![
        [3000.0, 0.045],
        [3000.0, -0.045],
        [3000.0, 0.001],
        [3000.0, -0.001],
        [6500.0, 0.045],
        [6500.0, -0.045],
        [6500.0, 0.001],
        [6500.0, -0.001],
    ] {
        let (u, v) = crate::models::uv_from_cct_duv::<crate::observers::CieObs1931>(t, d);
        let p = PlanckianTable::<crate::observers::CieObs1931>::new(None);
        let [tc, dc] = p.ohno2014(u, v);
        //		println!("{} {}", t, d);
        assert_abs_diff_eq!(t, tc * OHNO_CORR_1PCT_STEP, epsilon = 0.15);
        // using correction factor here, as this is a basic uv test.
        //
        assert_abs_diff_eq!(d, dc, epsilon = 0.000_01);
    }
}

#[test]
fn test_bounds() {
    use crate::models::uv_from_cct_duv;
    let p = PlanckianTable::<crate::observers::CieObs1931>::new(None);

    let (u, v) = uv_from_cct_duv::<crate::observers::CieObs1931>(900.0, 0.0);
    let [tc, dc] = p.ohno2014(u, v);
    println!("{} {}", tc, dc);
    assert!(tc.is_nan());
    assert!(dc.is_nan());

    let (u, v) = uv_from_cct_duv::<crate::observers::CieObs1931>(1001.0, 0.0);
    let [tc, dc] = p.ohno2014(u, v);
    println!("{} {}", tc, dc);
    assert!(tc.is_nan());
    assert!(dc.is_nan());

    let (u, v) = uv_from_cct_duv::<crate::observers::CieObs1931>(20186.0, 0.0);
    let [tc, dc] = p.ohno2014(u, v);
    println!("{} {}", tc, dc);
    assert!(tc.is_nan());
    assert!(dc.is_nan());

    let (u, v) = uv_from_cct_duv::<crate::observers::CieObs1931>(22000.0, 0.0);
    let [tc, dc] = p.ohno2014(u, v);
    println!("{} {}", tc, dc);
    assert!(tc.is_nan());
    assert!(dc.is_nan());
}

#[test]
/*

fn test_ohno_cascade(){

    use approx::assert_abs_diff_eq;
    use crate::models::uv_from_cct_duv;

    for [t,d] in vec![
        [3000.0, 0.045],
        [3000.0, -0.045],
        [3000.0, 0.001],
        [3000.0, -0.001],
        [6500.0, 0.045],
        [6500.0, -0.045],
        [6500.0, 0.001],
        [6500.0, -0.001],
        [13000.0, 0.045],
        [13000.0, -0.045],
        [13000.0, 0.001],
        [13000.0, -0.001],
    ] {
        let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(t, d);
        let [tc,dc] = cascade::<crate::observers::CieObs1931>(u,v);
        println!("{} {}", tc, dc);
        assert_abs_diff_eq!(t, tc, epsilon = 5E-3);
        assert_abs_diff_eq!(d, dc, epsilon = 1E-6);
    }

}
*/
#[test]
fn test_ohno_cascade_from_cctduv() {
    use crate::illuminants::{CctDuvCalc, Ohno2014Cascade};
    use crate::observers::CieObs1931;
    use approx::assert_abs_diff_eq;

    let oc: Ohno2014Cascade<CieObs1931> = Ohno2014Cascade::default();

    let tds: CctDuv<CieObs1931> = CctDuv::new(vec![
        [13000.0, -0.001],
        [13000.0, 0.0495],
        [13000.0, -0.0495],
        [6500.0, -0.001],
        [6500.0, 0.01],
        [6500.0, -0.0495],
        [3000.0, -0.001],
        [3000.0, 0.01],
        [3000.0, -0.01],
        [1500.0, -0.001],
        [1500.0, 0.04],
        [1500.0, -0.04],
    ]);
    let yuv: CieYuv1960<_> = tds.clone().into();

    let td_calc = oc.cct_duv(yuv);
    assert_abs_diff_eq!(tds, td_calc, epsilon = (5E-3, 1E-8));
}
