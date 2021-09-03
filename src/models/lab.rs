/*!
    A collection of a Lab values, associated with a standard observer and a reference white illuminant.


*/

use std::{fmt::Display, marker::PhantomData};

use crate::illuminants::{CieIllD65, Illuminant};
use crate::observers::StandardObserver;
use crate::swatches::{Swatches};
use crate::util::{Meter, Step};
use crate::DefaultObserver;
use nalgebra::{Matrix3x1, Matrix3xX};

use super::{CieXYZ, XYZValues};

#[derive(Debug,Clone)]
pub struct CieLab<I: Illuminant<C> = CieIllD65, C: StandardObserver = DefaultObserver> {
    pub data: Matrix3xX<f64>,
    cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
    illuminant: PhantomData<*const I>, // only used through I:Default(), but needed to mark the type
}

impl<C: StandardObserver, I: Illuminant<C>> CieLab<I, C>
where
//	Meter: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>
//	C: 'static:into();,
//	&'a C: Default,
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
    I: Illuminant<C>,
    C: StandardObserver,
 //   Meter: From<<<I as SpectralTable>::StepType as Step>::UnitValueType>,
   // &'static C: Default,
{
    fn from(lab: CieLab<I, C>) -> Self {
        let ill = I::default();
		let xyz: CieXYZ<C> = ill.into();
        let XYZValues { x: xn, y: yn, z: zn, } = xyz.into_iter().next().unwrap();
		let mut v: Vec<f64> = Vec::with_capacity(lab.data.len());
        for LabValues { l, a, b } in lab {
            let s = (l + 16f64) / 116f64;
            v.push(xn * lab_finv(s + a / 500f64));
			v.push(yn * lab_finv(s));
			v.push(zn * lab_finv(s - b / 200f64));
        }
		Self::new(Matrix3xX::from_vec(v))
    }
}

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

pub struct LabIter<I: Illuminant<C>, C: StandardObserver> {
    lab: CieLab<I, C>,
    i: usize,
}

pub struct LabIterRef<'a, I: Illuminant<C>, C: StandardObserver> {
    lab: &'a CieLab<I, C>,
    i: usize,
}

impl<C: StandardObserver, I: Illuminant<C>> Iterator for LabIter<I, C> {
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

impl<'a, C: StandardObserver, I: Illuminant<C>> Iterator for LabIterRef<'a, I, C> {
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

impl<C: StandardObserver, I: Illuminant<C>> IntoIterator for CieLab<I, C> {
    type Item = LabValues;

    type IntoIter = LabIter<I, C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { lab: self, i: 0 }
    }
}

impl<'a, C: StandardObserver, I: Illuminant<C>> IntoIterator for &'a CieLab<I, C> {
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
    use crate::ALL;

    let labs: CieLab<CieIllD50> = ColorChecker::<ALL>.into(); // using CieObs1931 and CieIllD65 as Default
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
/**
  Calculates CIELAB values for color swatches
*/
impl<'a, S, C, I> From<S> for CieLab<I, C>
where
    S: Swatches,
    C: StandardObserver,
   // &'a C: Default,
    I: Illuminant<C>,
   // <<S as SpectralTable>::StepType as Step>::UnitValueType: From<<<I as SpectralTable>::StepType as Step>::UnitValueType>,
   // Meter: From<<<I as SpectralTable>::StepType as Step>::UnitValueType>,
{
    fn from(swatch: S) -> Self {
       // let ill = I::default(); // illuminant spectrum
       // let ill_dom = ill.domain();
       // let ill_data  = ill.values(&ill_dom);
       // let sw_data = swatch.values(&ill_dom);

	   /*
        let (xyz_n, xyz) = C::xyz_from_dom_ill_mat(ill_dom, ill_data, sw_data);

        CieLab {
            data: cielab(xyz_n, xyz),
            cmf: PhantomData,
            illuminant: PhantomData,
        }
	   */
	   todo!()
    }
}

impl<C: StandardObserver, I: Illuminant<C>> Display for CieLab<I, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lab<{}>: {:.5}", C::NAME, self.data)
    }
}

fn cielab(xyz_n: Matrix3x1<f64>, xyz: Matrix3xX<f64>) -> Matrix3xX<f64> {
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

#[test]
/**
    ColorChecker CieLab values calculation.

    The test values are from the Babel color spreadsheet, with spectral values defined at a domain from 380 to 730nm,
    with 10nm steps.  The values here use the illuminant D50 domain, which uses 5nm steps. This results in small deviations
    in the order of 0.1% in CieLab values.
*/
fn test_cielab_colorchecker() {
    use crate::illuminants::CieIllD50;
    use crate::models::CieLab;
    use crate::observers::CieObs1931;
    use crate::swatches::checker::ColorChecker;
    use crate::swatches::{Gray, White};
    use crate::ALL;

    use approx::assert_abs_diff_eq;
    use nalgebra::matrix;

    let white: CieLab<CieIllD50, CieObs1931> = White::default().into();
    assert_abs_diff_eq!(white.data[(0, 0)], 100.0, epsilon = 0.00001);
    assert_abs_diff_eq!(white.data[(1, 0)], 0.0, epsilon = 0.00001);
    assert_abs_diff_eq!(white.data[(2, 0)], 0.0, epsilon = 0.00001);
    //	println!("White {:.4}", white);

    let gray: CieLab<CieIllD50, CieObs1931> = Gray(0.18418651851244416).into();
    assert_abs_diff_eq!(gray.data[(0, 0)], 50.0, epsilon = 0.00001);
    assert_abs_diff_eq!(gray.data[(1, 0)], 0.0, epsilon = 0.00001);
    assert_abs_diff_eq!(gray.data[(2, 0)], 0.0, epsilon = 0.00001);
    //	println!("Gray {:.4}", gray);

    let checker_lab: CieLab<CieIllD50, CieObs1931> = ColorChecker::<ALL>::default().into();
    let babel = matrix![
        38.44, 13.61, 14.53;
        65.95, 17.91, 17.87;
        50.06, -4.52, -22.25;
        43.28, -13.21, 21.94;
        55.31, 8.82, -24.60;
        70.69, -33.03, -0.11;
        62.65, 35.35, 57.86;
        40.24, 9.74, -44.35;
        51.60, 47.80, 16.90;
        30.50, 21.07, -20.02;
        72.46, -23.30, 57.00;
        71.95, 19.46, 68.12;
        28.87, 14.81, -50.15;
        55.15, -37.80, 31.64;
        42.28, 54.12, 28.67;
        82.27, 4.02, 79.99;
        51.91, 49.80, -13.82;
        50.72, -28.11, -27.95;
        96.53, -0.47, 2.42;
        81.21, -0.64, 0.27;
        66.48, -0.53, 0.00;
        50.83, -0.64, -0.14;
        35.85, -0.54, -0.49;
        20.81, 0.03, -0.39
    ];

    for (i, cc) in checker_lab.data.column_iter().enumerate() {
        assert_abs_diff_eq!(cc.x, babel[(i, 0)], epsilon = 0.05);
        assert_abs_diff_eq!(cc.y, babel[(i, 1)], epsilon = 0.05);
        assert_abs_diff_eq!(cc.z, babel[(i, 2)], epsilon = 0.05);
    }
}
