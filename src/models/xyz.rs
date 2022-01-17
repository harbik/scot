use std::{fmt::Display, marker::PhantomData};

use crate::{observers::StandardObserver, DefaultObserver};
use nalgebra::{Const, DVector, DefaultAllocator, Dim, Matrix3xX, OMatrix};

/**
    A collection of a tristimulus values, associated with a standard observer,
    and an optional set of tristimulus values of a reference white point.

    The reference to a standard observers color matching functions is not only used to uniquely identify the observer
    associated with the tristimulus values, but also for the conversion of chromaticity coordinates between different
    observers, using for example transforming back to a set of reference RGB spectra, and calculating the tristimulus
    values for a different observer. The standard observers have global (static) scope.


    TODO 
    - [ ] Convert data containter from Matrix3xX to OMatrix, the abstraction for a Vec or Array based container?
          This to avoid new allocation and copy in case we have a static container, 



*/
#[derive(Debug)]
pub struct CieXYZ<C: StandardObserver = DefaultObserver> {
    pub data: Matrix3xX<f64>,  // use OMatrix here? 
    pub y: Option<DVector<f64>>,
    cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieXYZ<C> {
    pub fn new(xyz: Matrix3xX<f64>) -> Self {
        Self {
            data: xyz,
            y: None,
            cmf: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.ncols()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn normalize(mut self, v: f64) -> Self {
        let mut ys: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            let y = self.data[(1, i)];
            let t = v / y;
            ys.push(y);
            self.data[(0, i)] *= t;
            self.data[(1, i)] = v;
            self.data[(2, i)] *= t;
        }
        self.y = Some(DVector::from(ys));
        self
    }
}

/**
    Convert static matrix to a dynamic matrix.

    Both are owned matrice.
    Copies from array storage, on the stack, to vector storage on the heap.

    // TODO: This is not needed!!!
    // Is here because of SpectralDistribution.xyz trait bound CieXYZ<C>:: From
    // Would be better to also use OMatrix as basic container in CieXYZ
*/
impl<O: StandardObserver, C: Dim> From<OMatrix<f64, Const<3>, C>> for CieXYZ<O>
where
    DefaultAllocator: nalgebra::allocator::Allocator<f64, Const<3>, C>,
{
    fn from(xyz: OMatrix<f64, Const<3>, C>) -> Self {
        let data = Matrix3xX::from_iterator(xyz.ncols(), xyz.into_iter().cloned());
        Self {
            data,
            y: None,
            cmf: PhantomData,
        }
    }
}

/// Convenience from array function to define values manualy
impl<C:StandardObserver, const M:usize> From<[[f64; 3]; M]>  for CieXYZ<C> {
    fn from(m: [[f64;3];M]) -> Self {
        let data = Matrix3xX::<f64>::from_fn(M, |i,j| m[j][i]);
        Self::new(data)
    }
}

/// Ditto from array reference
impl<C:StandardObserver, const M:usize> From<&[[f64; 3]; M]>  for CieXYZ<C> {
    fn from(m: &[[f64;3];M]) -> Self {
        let data = Matrix3xX::<f64>::from_fn(M, |i,j| m[j][i]);
        Self::new(data)
    }
}

/**
   Calculate XYZ tristimilus value from spectral distributions.

   This is a generic implementation for calculation of XYZ values.
   It interpolates the color matching functions values onto the
   spectral distribution's domain.

   # Examples
   Calculate Tristimulus values for a Blackbody radiator
   ```
   use rcs::illuminants::Planckian;
   use rcs::observers::CieObs1931;
   use rcs::models::CieXYZ;

   let bb = CieXYZ::<CieObs1931>::from(Planckian::new(3000));
   println!("{}",bb);
   ```
*/
impl<C: StandardObserver> Display for CieXYZ<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XYZ<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct XYZIter<C: StandardObserver> {
    xyz: CieXYZ<C>,
    i: usize,
}

impl<C: StandardObserver> Iterator for XYZIter<C> {
    type Item = XYZValues;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.xyz.data.ncols() {
            let x = self.xyz.data[(0, self.i)];
            let y = self.xyz.data[(1, self.i)];
            let z = self.xyz.data[(2, self.i)];
            self.i += 1;
            Some(XYZValues { x, y, z })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct XYZValues {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<C: StandardObserver> IntoIterator for CieXYZ<C> {
    type Item = XYZValues;

    type IntoIter = XYZIter<C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { xyz: self, i: 0 }
    }
}

#[derive(Debug)]
pub struct CieYxy<C: StandardObserver = DefaultObserver> {
    pub data: Matrix3xX<f64>,
    cmf: PhantomData<*const C>, // only used through C::Default(), but needed to mark the type
}

impl<C: StandardObserver> CieYxy<C> {
    pub fn new(yxy: Matrix3xX<f64>) -> Self {
        Self {
            data: yxy,
            cmf: PhantomData,
        }
    }

    pub fn yxy(&self, i: usize) -> [f64; 3] {
        let v = self.data.column(i);
        [v.x, v.y, v.z]
    }
}

impl<C, X> From<X> for CieYxy<C>
where
    C: StandardObserver,
    X: Into<CieXYZ<C>>,
{
    fn from(x: X) -> Self {
        let m: CieXYZ<C> = x.into();

        let mut v: Vec<f64> = Vec::with_capacity(m.data.len());
        for xyz in m.data.column_iter() {
            let s = xyz.sum();
            v.push(xyz.y);
            v.push(xyz.x / s);
            v.push(xyz.y / s);
        }
        Self::new(Matrix3xX::<f64>::from_vec(v))
    }
}

impl<C: StandardObserver> Display for CieYxy<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Yxy<{}>: {:.5}", C::NAME, self.data)
    }
}

pub struct YxyIter<C: StandardObserver> {
    lxy: CieYxy<C>,
    i: usize,
}

impl<C: StandardObserver> Iterator for YxyIter<C> {
    type Item = YxyValues;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.lxy.data.ncols() {
            let l = self.lxy.data[(0, self.i)];
            let x = self.lxy.data[(1, self.i)];
            let y = self.lxy.data[(2, self.i)];
            self.i += 1;
            Some(YxyValues { l, x, y })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct YxyValues {
    pub l: f64,
    pub x: f64,
    pub y: f64,
}

impl<C: StandardObserver> IntoIterator for CieYxy<C> {
    type Item = YxyValues;

    type IntoIter = YxyIter<C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { lxy: self, i: 0 }
    }
}
