/*!
CIE &Delta;E<sub>1976</sub> color differences for two spectral color collections.

In this formula the color difference is represented by the direct distance
between two color points in the CIE L<sup>*</sup>a<sup>*</sup>b<sup>*</sup> color space.
It has been superseeded by the color difference metrics CIE &Delta;E<sub>1994</sub>, and CIE &Delta;E<sub>2000</sub>.

# Example
Calculate the CIE DE1976 color differences between Color Checker Swatch #13, and
the CIE CES color samples.
```
    use scot::observers::CieObs1931;
    use scot::illuminants::D65;
    use scot::swatches::{ColorCheckerSwatch, Ces};
    let de = CieDE1976::<CieObs1931, D65>::new(ColorCheckerSwatch::<13>, Ces);
    println!("{:.1}", de.0);
```
This will print a matrix, with 16 rows, each row corresponding to one of the
color checker samples, and 99 columns, each of the columns corresponding to
one of the IES TM30 color samples. This matrix can be used to find the
best match of a color checker sample to one of the IES CES samples.

The same color sample sets can also be evaluated using the CIE 2015 2º color matching functions,
and using a D50 white point:
```
    use scot::observers::{CieObsF2};
    use scot::illuminants::{CieD50};
    use scot::swatches::{ColorChecker, IesTm30Ces};
    let de = CieDE1976::<Cie2015, CieD50>::new(ColorChecker, IesTm30Ces);
    println!("{:.1}", de.0);
```

 */

use std::fmt::Debug;
use std::marker::PhantomData;

use nalgebra::DMatrix;

use crate::illuminants::{Illuminant, D65};
use crate::models::{CieLab, LabValues};
use crate::observers::{CieObs1931, StandardObserver};

use super::DeltaEValues;

#[derive()]
pub struct CieDE1976<I = D65, C = CieObs1931>(
    pub DMatrix<f64>,
    PhantomData<*const C>,
    PhantomData<*const I>,
);

impl<I, C> CieDE1976<I, C>
where
    I: Illuminant,
    C: StandardObserver,
{
    pub fn new<L1, L2>(l1: L1, l2: L2) -> Self
    where
        L1: Into<CieLab<I, C>>,
        L2: Into<CieLab<I, C>>,
    {
        Self::from((l1, l2))
    }
}

impl<I: Illuminant, C: StandardObserver> DeltaEValues<I, C> for CieDE1976<I, C> {}
/*
   See AsRef<DMatrix<f64>> implementation, which represent the error values.
   Using default methods only.
*/

/**
   Generates DeltaE values from a pair objects which can produce
   one or more CieLab values.
*/
impl<I, C, L1, L2> From<(L1, L2)> for CieDE1976<I, C>
where
    L1: Into<CieLab<I, C>>,
    L2: Into<CieLab<I, C>>,
    I: Illuminant,
    C: StandardObserver,
{
    fn from(l: (L1, L2)) -> Self {
        let lab1: CieLab<I, C> = l.0.into();
        let lab2: CieLab<I, C> = l.1.into();

        let n1 = lab1.len();
        let n2 = lab2.len();
        let mut v: Vec<f64> = Vec::with_capacity(n1 * n2);
        for LabValues {
            l: l1,
            a: a1,
            b: b1,
        } in lab1
        {
            for LabValues {
                l: l2,
                a: a2,
                b: b2,
            } in lab2.iter()
            {
                v.push(
                    ((l2 - l1) * (l2 - l1) + (a2 - a1) * (a2 - a1) + (b2 - b1) * (b2 - b1)).sqrt(),
                );
            }
        }
        Self(DMatrix::from_vec(n2, n1, v), PhantomData, PhantomData)
    }
}

impl<I: Illuminant, C: StandardObserver> AsRef<DMatrix<f64>> for CieDE1976<I, C> {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.0
    }
}

impl<C: StandardObserver, I: Illuminant> Debug for CieDE1976<I, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<I, C> std::fmt::Display for CieDE1976<I, C>
where
    I: Illuminant,
    C: StandardObserver,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
