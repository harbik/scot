/*! 

# 1964 CIEU\*V\*W\* Color Space

This color space was developed by Wyszecki 
 (based on the CIE 1960 UCS (Uniform Color Space))
 to create a more perceptually uniform color space,
 and to be able to calculate color differences between samples with different luminance.

It is calculated using:

  *U\* = 13 W\* (u-u<sub>n</sub>)*,

  *V\* = 13 W\* (v-v<sub>n</sub>)*,

  *W\* = 25 Y<sup>1/3</sup> - 17*,

with (*u<sub>n</sub>,v<sub>n</sub>*) the CIE UCS 1960 chromaticity coordinates of a white reference
    -similar to CIELAB-
    and *Y* the relative luminance value, normalized for a reference white value of *Y<sub>n</sub>=100*, 
    and (*(u,v)*) the CIE UCS 1960 chromaticity coordinates of its target.

It has been superseded by the CIELAB colorspace in 1976,
 but is still used to calculate the color rendering index (CRI) for lightsources,
 a quality metric which is still a CIE recommended standard.

 */

use std::{marker::PhantomData};
use crate::{DefaultObserver, illuminants::{Illuminant, }, observers::StandardObserver};
use nalgebra::Matrix3xX;
use super::{CieLab, CieXYZ, lab_to_xyz, uv60};

#[derive(Debug, Clone)]
/**
 CIEU\*V\*W\* `Matrix3xX` values wrapper.

 Besides a standard observer *C*,
  `CieUVW<I,C>` als carries the type parameter `Illuminant` I, 
  which is used as reference white in this colorspace,
  similar to `CieLab<I,C>`.
 */
pub struct CieUVW<I: Illuminant, C: StandardObserver = DefaultObserver> {
    pub data: Matrix3xX<f64>,
    i: PhantomData<*const I>, // only used through C::Default(), but needed to mark the type
    cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<I: Illuminant, C: StandardObserver> CieUVW<I, C> {
    pub fn new(data: Matrix3xX<f64>) -> Self {
        Self {
            data,
            i: PhantomData,
            cmf: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.ncols()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}


/*
Calculate `CieUVW<I,C>` color space coordinates from any `CieLab<I,C>` producer.

*/
impl<'a, I, C, L> From<L> for CieUVW<I, C>
where
    I: Illuminant + Into<CieXYZ<C>>,
    C: StandardObserver,
    L: Into<CieLab<I,C>>
{
    fn from(lab_src: L) -> Self {
        let white: CieXYZ<C> = I::default().into();
        let xyz_n = white.data.column(0);
        let &[x_n, y_n, z_n]: &[f64;3] = xyz_n.as_ref();
        let [_, u_n, v_n] = uv60(x_n, y_n, z_n);

        let labs: CieLab<I,C> = lab_src.into();
        let mut xyzs = lab_to_xyz(xyz_n, labs.data);
        xyzs.column_iter_mut().for_each(|mut xyz|{
            let [x, y, z]: &mut [f64;3] = xyz.as_mut();
            let [_, u, v]  = uv60(*x, *y, *z);
            let ws = 25.0 * y.powf(1.0/3.0) - 17.0;
            let us = 13.0 * ws * (u - u_n);
            let vs = 13.0 * ws * (v - v_n);
            *x = us; *y = vs; *z = ws;
        });

        Self::new(xyzs)
    }
}



#[test]

fn cie_uvw(){
    use crate::illuminants::D65;
    use crate::observers::CieObs1931;
    use crate::swatches::Gray;
    let lab = CieLab::<D65, CieObs1931>::from([[50.0, -20.0, 20.0]]);
    let uvw: CieUVW<D65, CieObs1931> = lab.into();
    println!("{}", uvw.data);
}
