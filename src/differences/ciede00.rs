/*!
CIE &Delta;E<sub>1976</sub> color differences for two spectral color collections.

In this formula the color difference is represented by the direct distance
between two color points in the CIE L<sup>*</sup>a<sup>*</sup>b<sup>*</sup> color space.
It has been superseeded by the color difference metrics CIE &Delta;E<sub>1994</sub>, and CIE &Delta;E<sub>2000</sub>.

# Example
Calculate the CIE DE1976 color differences between Color Checker Swatch #13, and
the CIE CES color samples.
```
    use rcs::observers::Cie1931;
    use rcs::illuminants::CieD65;
    use rcs::swatches::{ColorCheckerSwatch, Ces};
    let de = CieDE1976::<Cie1931, CieD65>::new(ColorCheckerSwatch::<13>, Ces);
    println!("{:.1}", de.0);
```
This will print a matrix, with 16 rows, each row corresponding to one of the
color checker samples, and 99 columns, each of the columns corresponding to
one of the IES TM30 color samples. This matrix can be used to find the
best match of a color checker sample to one of the IES CES samples.

The same color sample sets can also be evaluated using the CIE 2015 2º color matching functions,
and using a D50 white point:
```
    use rcs::observers::{Cie2015};
    use rcs::illuminants::{CieD50};
    use rcs::swatches::{ColorChecker, IesTm30Ces};
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
pub struct CieDE2000<I = D65, C = CieObs1931>(
    pub DMatrix<f64>,
    PhantomData<*const C>,
    PhantomData<*const I>,
);

impl<I, C> CieDE2000<I, C>
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

impl<I: Illuminant, C: StandardObserver> DeltaEValues<I, C> for CieDE2000<I, C> {}
/*
   See AsRef<DMatrix<f64>> implementation, which represent the error values.
   Using default methods only.
*/

/**
   Generates DeltaE values from a pair objects which can produce
   one or more CieLab values.
*/
impl<I, C, L1, L2> From<(L1, L2)> for CieDE2000<I, C>
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
                v.push(de2000(l1, a1, b1, l2, a2, b2));
            }
        }
        Self(DMatrix::from_vec(n2, n1, v), PhantomData, PhantomData)
    }
}

/*
        export function DE2000(Lab1:number[], Lab2:number[], kL: number = 1.0, kC: number = 1.0,  kH: number = 1.0, debug:boolean = false ) : number|number[] {
// see: http://www2.ece.rochester.edu/~gsharma/ciede2000/ciede2000noteCRNA.pdf table I.

if (Lab1.length<3) throw new Error("Need three coordinates in cieDE2000 first Lab parameter.")
if (Lab2.length<3) throw new Error("Need three coordinates in cieDE2000 second Lab parameter.")

let DLp = Lab2[0] - Lab1[0];
let Lavg = Lab2[0]/2.0 + Lab1[0]/2.0;
let C1 = Math.sqrt(Lab1[1]*Lab1[1] + Lab1[2] * Lab1[2]);
let C2 = Math.sqrt(Lab2[1]*Lab2[1] + Lab2[2] * Lab2[2]);
let Cavg = (C1+C2)/2.0;
let G =  0.5 * (1-Math.sqrt(Math.pow(Cavg,7)/(Math.pow(Cavg,7)+Math.pow(25.0,7))));
let a1p = (1. + G)  * Lab1[1];
let a2p = (1. + G)  * Lab2[1];
let C1p = Math.sqrt(a1p*a1p+Lab1[2]*Lab1[2]);
let C2p = Math.sqrt(a2p*a2p+Lab2[2]*Lab2[2]);
let Cavgp = (C1p + C2p)/2;
let DCp = C2p - C1p;

let h1p = Math.atan2(Lab1[2],a1p);
if (h1p<0) h1p += 2.0 * Math.PI;
let h2p = (Math.atan2(Lab2[2], a2p));
if (h2p<0) h2p += 2.0 * Math.PI;


let Dhp:number;

if (Math.abs(C1p)<1.E-10 || Math.abs(C2p)<1.E-10) Dhp = 0;
else if (Math.abs(h1p-h2p)<=Math.PI) Dhp = h2p - h1p;
else if ((h2p-h1p)>Math.PI)  Dhp = h2p - h1p - 2. * Math.PI;
else Dhp = h2p - h1p - 2. * Math.PI;

let DHp = 2. * Math.sqrt(C1p*C2p)*Math.sin(Dhp/2.0);

let Havgp:number;

if (Math.abs(C1p)<1.E-10 || Math.abs(C2p)<1.E-10) Havgp = h1p + h2p;
else if (Math.abs(h1p-h2p)<=Math.PI) Havgp = (h1p+h2p)/2.0;
else if ((h1p+h2p) < (2. * Math.PI)) Havgp = (h1p + h2p + 2.0 * Math.PI) / 2.0
else Havgp = (h1p + h2p - 2.* Math.PI)/ 2.0;


let T = 1.0 - 0.17 * Math.cos(Havgp - 30./180.*Math.PI) + 0.24 * Math.cos(2.0 * Havgp) + 0.32 * Math.cos(3. * Havgp + 6./180. * Math.PI) - 0.20 * Math.cos ( 4. * Havgp - 63./180. * Math.PI);

let Sl = 1.0 + 0.015 * Math.pow(Lavg - 50., 2.) / Math.sqrt(20.0 + Math.pow(Lavg - 50.0, 2.));
let Sc = 1. + 0.045 * Cavgp;
let Sh = 1.0 + 0.015 * Cavgp * T;

let Dt = 30. * Math.PI/180. * Math.exp(-1. * Math.pow((Havgp*180.0/Math.PI - 275.)/25.0,2.));
let Rc = 2. * Math.sqrt(Math.pow(Cavgp, 7) / (Math.pow(Cavgp, 7.) + Math.pow(25., 7)));
let Rt = -1. * Math.sin(2. * Dt) * Rc;
let DE00 = Math.sqrt(
        Math.pow(DLp/(kL * Sl),2) +
        Math.pow(DCp/(kC * Sc),2) +
        Math.pow(DHp/(kH * Sh),2) +
        Rt * DCp / (kC * Sc) * DHp / (kH * Sh)
    );

if (debug) return [DE00, DLp, Lavg, a1p, a2p, C1p, C2p, Cavgp, h1p, h2p, Havgp, G, T, Sl, Sc, Sh, Rt, DE00];
else  return DE00;
}

 */
pub fn de2000(l1: f64, a1: f64, b1: f64, l2: f64, a2: f64, b2: f64) -> f64 {
    const KL: f64 = 1.0;
    const KC: f64 = 1.0;
    const KH: f64 = 1.0;
    //	const double deg360InRad = CIEDE2000::deg2Rad(360.0);
    //	const double deg180InRad = CIEDE2000::deg2Rad(180.0);
    const POW25TO7: f64 = 6103515625.0; /* pow(25, 7) */

    /*
     * Step 1
     */
    /* Equation 2 */
    let c1 = ((a1 * a1) + (b1 * b1)).sqrt();
    let c2 = ((a2 * a2) + (b2 * b2)).sqrt();
    /* Equation 3 */
    let bar_c = (c1 + c2) / 2.0;
    /* Equation 4 */
    let g = 0.5 * (1.0 - (bar_c.powi(7) / (bar_c.powi(7) + POW25TO7)).sqrt());
    /* Equation 5 */
    let a1_prime = (1.0 + g) * a1;
    let a2_prime = (1.0 + g) * a2;
    /* Equation 6 */
    let c_prime1 = ((a1_prime * a1_prime) + (b1 * b1)).sqrt();
    let c_prime2 = ((a2_prime * a2_prime) + (b2 * b2)).sqrt();
    /* Equation 7 */
    let mut h_prime1;
    if b1 == 0.0 && a1_prime == 0.0 {
        h_prime1 = 0.0;
    } else {
        h_prime1 = b1.atan2(a1_prime);
        /*
         * This must be converted to a hue angle in degrees between 0
         * and 360 by addition of 2􏰏 to negative hue angles.
         */
        if h_prime1 < 0.0 {
            h_prime1 += 360f64.to_radians();
        }
    }
    let mut h_prime2;
    if b2 == 0.0 && a2_prime == 0.0 {
        h_prime2 = 0.0;
    } else {
        h_prime2 = b2.atan2(a2_prime);
        /*
         * This must be converted to a hue angle in degrees between 0
         * and 360 by addition of 2􏰏 to negative hue angles.
         */
        if h_prime2 < 0.0 {
            h_prime2 += 360f64.to_radians();
        }
    }

    /*
     * Step 2
     */
    /* Equation 8 */
    let delta_l_prime = l2 - l1;
    /* Equation 9 */
    let delta_c_prime = c_prime2 - c_prime1;
    /* Equation 10 */
    let mut delta_h_prime: f64;
    let c_prime_product = c_prime1 * c_prime2;
    if c_prime_product == 0.0 {
        delta_h_prime = 0.0;
    } else {
        /* Avoid the fabs() call */
        delta_h_prime = h_prime2 - h_prime1;
        if delta_h_prime < -180f64.to_radians() {
            delta_h_prime += 360f64.to_radians();
        } else if delta_h_prime > 180f64.to_radians() {
            delta_h_prime -= 360f64.to_radians();
        }
    }
    /* Equation 11 */
    let delta_h_prime = 2.0 * c_prime_product.sqrt() * (delta_h_prime / 2.0).sin();

    /*
     * Step 3
     */
    /* Equation 12 */
    let bar_l_prime = (l1 + l2) / 2.0;
    /* Equation 13 */
    let bar_c_prime = (c_prime1 + c_prime2) / 2.0;
    /* Equation 14 */
    let bar_h_prime;
    let h_prime_sum = h_prime1 + h_prime2;
    if c_prime1 * c_prime2 == 0.0 {
        bar_h_prime = h_prime_sum;
    } else if (h_prime1 - h_prime2).abs() <= 180f64.to_radians() {
        bar_h_prime = h_prime_sum / 2.0;
    } else if h_prime_sum < 360f64.to_radians() {
        bar_h_prime = (h_prime_sum + 360f64.to_radians()) / 2.0;
    } else {
        bar_h_prime = (h_prime_sum - 360f64.to_radians()) / 2.0;
    }

    /* Equation 15 */
    let t = 1.0 - (0.17 * (bar_h_prime - 30f64.to_radians()).cos())
        + (0.24 * (2.0 * bar_h_prime).cos())
        + (0.32 * ((3.0 * bar_h_prime) + 6f64.to_radians()).cos())
        - (0.20 * ((4.0 * bar_h_prime) - 63f64.to_radians()).cos());
    /* Equation 16 */
    let delta_theta = 30f64.to_radians()
        * (-((bar_h_prime - 275f64.to_radians()) / 25f64.to_radians()).powi(2)).exp();
    /* Equation 17 */
    let r_c = 2.0 * (bar_c_prime.powi(7) / (bar_c_prime.powi(7) + POW25TO7)).sqrt();
    /* Equation 18 */
    let s_l = 1.0
        + ((0.015 * (bar_l_prime - 50.0).powi(2)) / (20.0 + (bar_l_prime - 50.0).powi(2)).sqrt());
    /* Equation 19 */
    let s_c = 1.0 + (0.045 * bar_c_prime);
    /* Equation 20 */
    let s_h = 1.0 + (0.015 * bar_c_prime * t);
    /* Equation 21 */
    let r_t = -(2.0 * delta_theta).sin() * r_c;

    /* Equation 22 */
    ((delta_l_prime / (KL * s_l)).powi(2)
        + (delta_c_prime / (KC * s_c)).powi(2)
        + (delta_h_prime / (KH * s_h)).powi(2)
        + (r_t * (delta_c_prime / (KC * s_c)) * (delta_h_prime / (KH * s_h))))
        .sqrt()
}

impl<I: Illuminant, C: StandardObserver> AsRef<DMatrix<f64>> for CieDE2000<I, C> {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.0
    }
}

impl<C: StandardObserver, I: Illuminant> Debug for CieDE2000<I, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<I, C> std::fmt::Display for CieDE2000<I, C>
where
    I: Illuminant,
    C: StandardObserver,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/*
#[test]
fn test_ciede00(){
    use crate::observers::{CieObs1931};
    use crate::illuminants::{CieIllD65};
    use crate::swatches::{ColorCheckerSwatch, Ces};
    let de = CieDE2000::<CieIllD65, CieObs1931>::from((ColorCheckerSwatch::<13>, Ces));
//	println!("{:.1}", de);
    let m = de.matches();
    let mut prev = 0f64;
    // check if error differences are in increasing order
    for i in 0..m.ncols() {
        let ind = m[(0,i)];
        let v = de.0[(0,ind)];
        assert!(v>prev);
        prev = v;
    //	println!("{} {} {:.1}", i, ind, v);
    }
}

*/
