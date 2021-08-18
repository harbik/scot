
/*!
CIE &Delta;E 1976 color differences for two color source collections.

 */

use std::fmt::Debug;
use std::marker::PhantomData;

use nalgebra::DMatrix;

use crate::models::{CieLab, LabValues};
use crate::observers::{CieObs1931, StandardObserver};
use crate::illuminants::{CieIllD65, Illuminant};

use super::DeltaEValues;

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
pub struct CieDE1994<I = CieIllD65, A = GraphicArts, C = CieObs1931 >(
	pub DMatrix<f64>, 
	PhantomData<*const C>, 
	PhantomData<*const I>, 
	PhantomData<*const A>
);

impl<C: StandardObserver, I: Illuminant, A: Application> CieDE1994<I,A,C> 
{

    pub fn new<L1, L2>(l1: L1 , l2: L2) -> Self
	where 
		L1: Into::<CieLab<I,C>>,
		L2: Into::<CieLab<I,C>>,
	{
		Self::from((l1,l2))
	}
}

impl<I: Illuminant, A: Application, C: StandardObserver> DeltaEValues<I,C> for CieDE1994<I,A,C>{}

impl<I: Illuminant, A: Application, C: StandardObserver> AsRef<DMatrix<f64>> for CieDE1994<I,A, C> {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.0
    }
}

impl<C: StandardObserver, I: Illuminant, A: Application> Debug for CieDE1994<C,I,A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       self.0.fmt(f) 
    }
}

impl<L1,L2,I,C,A> From<(L1, L2)> for CieDE1994<I,A,C>
where
	L1: Into::<CieLab<I,C>>,
	L2: Into::<CieLab<I,C>>,
	I: Illuminant,
	C: StandardObserver,
	A: Application
{
    fn from(l: (L1, L2)) -> Self {
		let lab1: CieLab::<I,C> = l.0.into();
		let lab2: CieLab::<I,C> = l.1.into();

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
				let dh2 = da * da + db * db - dc * dc; 
				// avoid calculation of sqrt as only h^2 is needed, and potential small negative values
				let sl = 1.0;
				let sc = 1.0 + A::K1 * c1;
				let sh = 1.0 + A::K2 * c1; 
				// c1 here, not c2, according to wiki color differences, and bruce lindbloom site
				v.push((
					(dl/(A::KL*sl)).powi(2) +
					(dc/(A::KC*sc)).powi(2) +
					dh2/(A::KH*sh).powi(2) 
				).sqrt());

			}
		};
		Self(DMatrix::from_vec(n1, n2, v), PhantomData, PhantomData, PhantomData)
    }
}

#[test]
fn test_ciede76(){
	use crate::observers::{CieObs1931};
	use crate::illuminants::{CieIllD65};
	use crate::swatches::{ColorChecker, IesTm30Ces};
	use crate::ALL;
	let de = CieDE1994::<CieIllD65, GraphicArts, CieObs1931>::new(ColorChecker::<13>, IesTm30Ces::<ALL>);
	let m = de.matches();
	let mut prev = 0f64;
	// check if error differences are in increasing order
	for i in 0..m.ncols() {
		let ind = m[(0,i)];
		let v = de.0[(0,ind)];
		assert!(v>prev);
		prev = v;
		println!("{} {} {:.1}", i, ind, v);
	}
}



