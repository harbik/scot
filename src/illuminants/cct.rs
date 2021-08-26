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

# Ohno: 1% step table search with parabolic and linear interpolation

This is in a implementation of the method as described by Ohno\[2014\].
In this method a table of Planckian locus values, in form of CIE 1960 u and v chromaticity coordinates, is generated for a range 
of temperature values distributed on a multiplicative scale.
The implementation here starts with a temperature of 1000K, and uses a multiplication factor of 1%, so the next values in this scale
are 1000K * 1.01 = 1010K, and 1010K * 1.01 = 1020.1K.
In total 303 values are generated, with a maximum of 20186.21K.

# Ohno's cascase tables



# References

- Ohno\[2014\]: Yoshi Ohno, Practical Use and Calculation of CCT and Duv, LEUKOS: The Journal of the Illuminating Engineering Society
of North America, 10:1, 47-55, DOI: 10.1080/15502724.2014.839020


*/
use core::panic;
use std::{error::Error,  marker::PhantomData};

use nalgebra::{DVector, Matrix2xX};
use crate::{DefaultObserver, Meter, SpectralData, Step,};
use crate::models::yuv1960::{CieYuv1960, CieYuv1960Values};
use crate::observers::{StandardObserver};

use super::Planckian;


/**
	Correlated color temperatures, CCT, and distances to a Planckian locus, Duv, for a collection of spectral sources.
*/
pub struct CctDuv(Matrix2xX<f64>);

pub trait CctDuvCalc { // Illuminant?
	fn cct_duv<S>(sd: S) -> CctDuv 
	where
		S: SpectralData,
		Meter: From<<<S as SpectralData>::StepType as Step>::UnitValueType>
	;
}


/**
	Multiplicative increasing temperature scale
*/
#[derive(Clone,Copy)]
pub struct 	CctLadder {
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
	pub fn new(start:f64, end: f64, mul: f64) -> Self {
		if start <=0.0 || end<start || mul <=1.0 {
			panic!("CctLadder Error: end value should be larger than start value, and the multiplicaton factor should be larger than 1.0");
		}
		Self {
			cct_min: start,
			cct_mul: mul,
			imax: ((end/start).log10()/mul.log10()).ceil() as i32,
		}
	}

	pub fn cct(&self, i: i32) -> Result<f64, Box<dyn Error>> {
		if i< self.imax {
			Ok(self.cct_min * self.cct_mul.powi(i))
		} else {
			Err("Index out of range".into())
		}
	}
}

impl Iterator for CctLadder {

	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.imax>0 {
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
fn test_cct() -> Result<(), Box<dyn Error>>{
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
where C: StandardObserver
{
	fn new(l: Option<CctLadder>) -> Self {
		let tpv: Vec<f64> = l.unwrap_or_default().into_iter().collect();
		let cct_uv: CieYuv1960<C> = Planckian::new(tpv.clone()).into();
		Self(tpv, cct_uv)
	} 

	/**
		Calculates distances between a test (u,v) point, and all the (u,v) points in the planckian table.
	*/
	fn sq_distances(&self, u: f64, v:f64) -> DVector<f64>{
			let mut d2v: Vec<f64> = Vec::with_capacity(self.0.len());
			for CieYuv1960Values{ y: _, u: ur, v: vr} in &self.1 {
				d2v.push((u - ur) * (u - ur) + (v - vr) * (v - vr));
			}
			DVector::from_vec(d2v)
	}

	fn triangular(&self, i: usize, duv2values: DVector<f64>) -> [f64;2] {
		let tp = self.0[i-1];
		let dp2 = duv2values[i-1];
		let tn = self.0[i+1];
		let dn2 = duv2values[i+1];
		let l2 = (self.1.data[(1, i+1)]-self.1.data[(1, i-1)]).powi(2) + (self.1.data[(2, i+1)]-self.1.data[(2, i-1)]).powi(2) ;
		let l = l2.sqrt();
		let x = (dp2 - dn2 + l2) / (2. * l);
		let t = tp + (tn - tp) * x / l;
		let d = (dp2 - x * x).sqrt();
		[t, d]
	}

	fn parabolic(&self, i: usize, duv2values: DVector<f64>) -> [f64;2] {
		let tp = self.0[i-1];
		let t = self.0[i];
		let tn = self.0[i+1];
		let dp = duv2values[i-1].sqrt();
		let d = duv2values[i].sqrt();
		let dn = duv2values[i+1].sqrt();
		let x = (tn - t) * (tp - tn) * (t - tp);
		let a = (tp * (dn - d) + t * (dp - dn) + tn * (d - dp)) / x;
		let b = -(tp * tp * (dn - d) + t * t * (dp - dn) + tn * tn * (d - dp)) / x;
		let c = -(dp * (tn - t) * t * tn + d * (tp - tn) * tp * tn + dn * (t - tp) * tp * t) / x;
		let tt = -b / (2. * a); 
			// see 	Y. Ohno, Leukos Non uniformity effect CCT scale in uv coordinates, not needed if the step factor is less than 1.003
		[tt, a * tt * tt + b * tt + c]
	}

	fn ohno2014(&self, u: f64, v: f64) -> [f64; 2] {
		let d2m = self.sq_distances(u,v);
		let imin = d2m.imin();
		if imin<1 || imin>self.0.len()-2 {
			[f64::NAN, f64::NAN]
		} else {
			let [t,d] = if d2m[imin].sqrt()<0.002 {
				self.triangular(imin, d2m)
			} else {
				self.parabolic(imin, d2m)
			};
			if v<self.1.data[(2,imin)] {
				[t,-d]
			} else {
				[t,d]
			}
		}
	}

	fn zoom(&self, u: f64, v: f64, mul: f64) -> CctLadder {
		let d2m = self.sq_distances(u,v);
		let imin = d2m.imin();
		CctLadder::new(self.0[imin-1], self.0[imin+1], mul)
	}
}

fn cascade<C: StandardObserver>(u: f64, v: f64) -> [f64;2] {
	let pt = PlanckianTable::<C>::new(Some(CctLadder::new(1000.0, 32000.0, 1.0 + 0.15)));
	let pt2 = PlanckianTable::<C>::new(Some(pt.zoom(u, v, 1.0 + 0.015)));
	let pt3 = PlanckianTable::<C>::new(Some(pt2.zoom(u, v, 1.0 + 0.0015)));
	let pt4 = PlanckianTable::<C>::new(Some(pt3.zoom(u, v, 1.0 + 0.00015)));
	pt4.ohno2014(u, v) // correction here not needed, due to small step size
}

const OHNO_CORR_1PCT_STEP: f64 = 0.99991; // the somewhat 'magical' correction factor, as listed in Ohno's article for the 1% step table

#[derive(Default)]
pub struct Ohno2014<C:StandardObserver = DefaultObserver>(PhantomData::<*const C>);

impl<C: StandardObserver> CctDuvCalc for Ohno2014<C>	 {
    fn cct_duv<S>(sd: S) -> CctDuv
	where
		S: SpectralData,
		Meter: From<<<S as SpectralData>::StepType as Step>::UnitValueType>
	{
		let pt = PlanckianTable::<C>::new(None);
		let uvs_test: CieYuv1960<C> = sd.into();
		let mut mv: Vec<f64> = Vec::with_capacity(uvs_test.data.len());
		for CieYuv1960Values { y: _, u, v} in uvs_test {
			let [t,d] = pt.ohno2014(u, v);
			mv.push(t * OHNO_CORR_1PCT_STEP);
			if d.abs()<= 0.05 {mv.push(d)}
			else {mv.push(f64::NAN)}; // out of range
		}
		CctDuv(Matrix2xX::<f64>::from_vec(mv))
	}
}

#[derive(Default)]
pub struct Ohno2014Cascade<C:StandardObserver = DefaultObserver>(PhantomData::<*const C>);

impl<C: StandardObserver> CctDuvCalc for Ohno2014Cascade<C>	 {
    fn cct_duv<S>(sd: S) -> CctDuv
	where
		S: SpectralData,
		Meter: From<<<S as SpectralData>::StepType as Step>::UnitValueType>
	{
		let uvs_test: CieYuv1960<C> = sd.into();
		let mut mv: Vec<f64> = Vec::with_capacity(uvs_test.data.len());
		for CieYuv1960Values { y: _, u, v} in uvs_test {
			let [t,d] = cascade::<C>(u, v);
			mv.push(t);
			if d.abs()<= 0.05 {mv.push(d)}
			else {mv.push(f64::NAN)}; // out of range
		}
		CctDuv(Matrix2xX::<f64>::from_vec(mv))
	}
}

#[allow(dead_code)]
fn uv_from_cct_duv<C: StandardObserver>(cct:f64, duv:f64) ->(f64,f64) {
	let CieYuv1960Values{y: _, u: u0, v: v0} = CieYuv1960::<C>::from(Planckian::new(cct)).into_iter().next().unwrap();
	let CieYuv1960Values{y: _, u: u1, v: v1} = CieYuv1960::<C>::from(Planckian::new(cct+0.01)).into_iter().next().unwrap();
	let du = u0 - u1;
	let dv = v0 - v1;
	let hyp = du.hypot(dv);
	(u0 - dv * duv/hyp, v0 + du  * duv/hyp) // see Ohno, Leukos, Practical Use and Calculation of CCT and DUV 
}

#[test]
fn test_ohno(){
	use approx::assert_abs_diff_eq;

	for [t,d] in vec![
		[3000.0, 0.045],
		[3000.0, -0.045],
		[3000.0, 0.001],
		[3000.0, -0.001],
		[6500.0, 0.045],
		[6500.0, -0.045],
		[6500.0, 0.001],
		[6500.0, -0.001],
	] {
		let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(t, d);
		let p = PlanckianTable::<crate::observers::CieObs1931>::new(None);
		let [tc,dc] = p.ohno2014(u,v);
//		println!("{} {}", t, d);
		assert_abs_diff_eq!(t, tc*OHNO_CORR_1PCT_STEP, epsilon = 0.15); 
			// using correction factor here, as this is a basic uv test.
			// 
		assert_abs_diff_eq!(d, dc, epsilon = 0.000_01);
	}
}

#[test]
fn test_bounds(){
	let p = PlanckianTable::<crate::observers::CieObs1931>::new(None);

	let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(900.0, 0.0);
	let [tc,dc] = p.ohno2014(u,v);
	println!("{} {}",tc, dc);
	assert!(tc.is_nan());
	assert!(dc.is_nan());

	let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(1001.0, 0.0);
	let [tc,dc] = p.ohno2014(u,v);
	println!("{} {}",tc, dc);
	assert!(tc.is_nan());
	assert!(dc.is_nan());
	
	let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(20186.0, 0.0);
	let [tc,dc] = p.ohno2014(u,v);
	println!("{} {}",tc, dc);
	assert!(tc.is_nan());
	assert!(dc.is_nan());

	let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(22000.0, 0.0);
	let [tc,dc] = p.ohno2014(u,v);
	println!("{} {}",tc, dc);
	assert!(tc.is_nan());
	assert!(dc.is_nan());
}


#[test]
fn test_ohno_cascade(){

	use approx::assert_abs_diff_eq;

	for [t,d] in vec![
		[3000.0, 0.045],
		[3000.0, -0.045],
		[3000.0, 0.001],
		[3000.0, -0.001],
		[6500.0, 0.045],
		[6500.0, -0.045],
		[6500.0, 0.001],
		[6500.0, -0.001],
	] {
		let (u,v) = uv_from_cct_duv::<crate::observers::CieObs1931>(t, d);
		let [tc,dc] = cascade::<crate::observers::CieObs1931>(u,v);
	//	println!("{} {}", t, d);
		assert_abs_diff_eq!(t, tc, epsilon = 5E-3);
		assert_abs_diff_eq!(d, dc, epsilon = 1E-6);
	}
	
}


