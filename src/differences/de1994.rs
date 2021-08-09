
/*!
CIE &Delta;E 1976 color differences for two color source collections.

In this formula the color difference is represented by the direct distance
between two color points in the CIE L<sup>*</sup>a<sup>*</sup>b<sup>*</sup> color space.	
It has been succeeded by better color difference metrix in 1994, and 2000.

# Example
Calculate the CIE DE1976 color differences between Color Checker patches, and 
the CIE CES color samples.
```
	use crate::observers::Cie1931;
	use crate::illuminants::CieD65;
	use crate::swatches::{ColorChecker, IesTm30Ces};
	let de = CieDE1976::<Cie1931, CieD65>::new(ColorChecker, IesTm30Ces);
	println!("{:.1}", de.0);
```
This will print a matrix, with 16 rows, each row corresponding to one of the 
color checker samples, and 99 columns, each of the columns corresponding to 
one of the IES TM30 color samples. This matrix can be used to find the 
best match of a color checker sample to one of the IES CES samples.

The same color sample sets can also be evaluated using the CIE 2015 2º color matching functions,
and using a D50 white point:
```
	use crate::observers::{Cie2015};
	use crate::illuminants::{CieD50};
	use crate::swatches::{ColorChecker, IesTm30Ces};
	let de = CieDE1976::<Cie2015, CieD50>::new(ColorChecker, IesTm30Ces);
	println!("{:.1}", de.0);
```
 */

use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use nalgebra::DMatrix;

use crate::models::{CieLab, LabValues};
use crate::observers::{CieObs1931, StandardObserver};
use crate::illuminants::{CieIllD65, Illuminant};

pub trait Application {
	const KL: f64;
	const K1: f64;
	const K2: f64;
	const KC: f64 = 1.0;
	const KH: f64 = 1.0;

}

// {GraphicArts, Textiles}

#[derive(Debug, Default)]
pub struct GraphicArts;

impl Application for GraphicArts {
    const KL: f64 = 1.0;
    const K1: f64 = 0.045;
    const K2: f64 = 0.015;
}

#[derive(Debug, Default)]
pub struct Textiles;

impl Application for Textiles {
    const KL: f64 = 2.0;
    const K1: f64 = 0.048;
    const K2: f64 = 0.014;
}

#[derive()]
pub struct CieDE1994<C = CieObs1931, I = CieIllD65, A = GraphicArts>(
	pub DMatrix<f64>, 
	PhantomData<*const C>, 
	PhantomData<*const I>, 
	PhantomData<*const A>
);

impl<C: StandardObserver, I: Illuminant, A: Application> CieDE1994<C,I,A> 
{

    pub fn new<L1, L2>(l1: L1 , l2: L2) -> Self
	where 
		L1: Into::<CieLab<I,C>>,
		L2: Into::<CieLab<I,C>>,
	{
		let lab1: CieLab::<I,C> = l1.into();
		let lab2: CieLab::<I,C> = l2.into();

		let n1 = lab1.len();
		let n2 = lab2.len();
		let mut v: Vec<f64> = Vec::with_capacity(n1 * n2);
		for LabValues{l:l1,a:a1,b:b1} in lab1 {
			for LabValues{l:l2,a:a2, b:b2} in lab2.iter() {
				let dl = l2 - l1;
				let da = a2 - a1;
				let db = b2 -  b1;
				let c1 = (a1*a1 + b1*b1).sqrt();
				let c2 = (a2*a2 + b2*b2).sqrt();
				let dc = c1 - c2;
				let dh = (da * da + db * db - dc * dc).sqrt();
				let sl = 1.0;
				let sc = 1.0 + A::K1 * c1;
				let sh = 1.0 + A::K2 * c1;
				v.push((
					dl/(A::KL*sl).powi(2) +
					dc/(A::KC*sc).powi(2) +
					dh/(A::KH*sh).powi(2) 
				).sqrt());

			}
		};
		Self(DMatrix::from_vec(n1, n2, v), PhantomData, PhantomData, PhantomData)
    }

	pub fn top3_matches(&self) -> Vec<Vec<(usize, f64)>> {
		let mut matched: Vec<Vec<(usize, f64)>> = Vec::with_capacity(self.0.nrows());
		for r in self.0.row_iter() {
			let hm : HashMap<usize, f64> = r.into_iter().cloned().enumerate().collect();
			let mut v: Vec<(usize,f64)> = hm.into_iter().collect();
			v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
			matched.push(v.into_iter().take(3).collect::<Vec<(usize,f64)>>());
		}
		matched
	}
}

impl<C: StandardObserver, I: Illuminant, A: Application> Debug for CieDE1994<C,I,A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       self.0.fmt(f) 
    }
}

#[test]
fn test_ciede76(){
	use crate::ALL;
	use crate::observers::{CieObs1931};
	use crate::illuminants::{CieIllD65};
	use crate::swatches::{ColorChecker, IesTm30Ces};
	let de = CieDE1994::<CieObs1931, CieIllD65, GraphicArts>::new(ColorChecker::<ALL>, IesTm30Ces);
	println!("{:.0}", de.0);
	println!("{:.3?}", de.top3_matches());
}



