/*!
    Interpolation methods for spectral distributions.

    These type of interpolation methods work well with relatively smooth spectral distributions, such as typical surface
    reflections spectra, and color matching functions.  For peaky spectral distributions, which can occur with emissive
    sources such as hid and fluorescent lamps, these methods should not be used. It is best to interpoate smooth spetral
    distributions to peaky distributions: in this library, when calculating tristimulus values, color matching
    functions, which are smooth, are interpolated to illuminant spectra. In rare cases, having a peaky source spectrum,
    and peaky reflection or transmission spectrum, care has to be taken to make sure the color calculations are
    performed correctly.
*/






use nalgebra::{Const, Dynamic, MatrixSlice, VecStorage};
use nalgebra::{storage::Storage, DMatrix, Dim, Matrix,};

use crate::util::Domain;
use crate::util::{Step, Unit};

use super::IterInterpolateType;

pub fn matrix_from_data_by_lin_row_int<S1, S2, const R: usize, const C: usize>(
    from_domain: &Domain<S1>,
    to_domain: &Domain<S2>,
    data: &[[f64; R]; C], // 
) -> Matrix<f64, Const<R>, Dynamic, VecStorage<f64, Const<R>, Dynamic>> // MatrixNxX
where
    S1: Step + Clone + Copy,
    S2: Step + Clone + Copy,
    S1::UnitValueType: From<<S2>::UnitValueType>, // need to be able to express a value in domain S2 as a value in domain S1
{

	if  *from_domain== *to_domain {
		println!("Using direct copy");
		//let mut v: Vec<f64> = Vec::with_capacity(R*C);
		//data.iter().flat_map(|v|*v).for_each(|d|  v.push(d));
		//Matrix::<f64, Const<R>, Dynamic, _>::from_vec(v)
		Matrix::<f64, Const<R>, Dynamic, _>::from_iterator(to_domain.len(), data.iter().flat_map(|v|*v))
	} else {

		let mut values = Vec::<f64>::with_capacity(to_domain.len() * R);

		let start = from_domain.step.unitvalue(from_domain.range.start).value();
		let div = from_domain.step.unitvalue(1).value();

		for ut in to_domain {
			let from_domain_interval = (Into::<S1::UnitValueType>::into(ut).value() - start) / div;
			let index = from_domain_interval.floor() as usize;
			if (index==0 && from_domain_interval<0.0) || index>C-1 {
				for _r in 0..R {
					values.push(0.0)
				}
			} else {
				let frac = from_domain_interval.fract();
				if index==C-1 && frac<1E-6 { // end point
					for r in 0..R {
						//println!("{:?}", data[0][r]);
						values.push(data[index][r]);
					}
				} else {
					for r in 0..R {
						//println!("{:?}", data[0][r]);
						values.push(data[index][r]*(1.0-frac)+data[index+1][r]*frac);
					}
				}
			}
		}
		Matrix::<f64, Const<R>, Dynamic, _>::from_vec(values)
	}
}

#[test]
fn test_lin_row(){
	use super::{NONE5, NONE};
	use approx::assert_abs_diff_eq;
	use nalgebra::Matrix1xX;
	let dfrom = Domain::new(0,2, NONE5);
	let dto = Domain::new(0,10, NONE);
	let data = [[0.0], [5.0], [10.0]]; // column major array in nalgebra
	let m = matrix_from_data_by_lin_row_int(&dfrom, &dto, &data);
	//println!("{}", m);
	assert_abs_diff_eq!(m, Matrix1xX::from_vec(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]));
}


pub fn interp_cols<'a, S1, S2>( from_domain: &Domain<S1>, to_domain: &Domain<S2>, nc: usize, data: &'a [f64]  ) -> DMatrix<f64>
where
    S1: Step + Clone + Copy,
    S2: Step + Clone + Copy,
    S1::UnitValueType: From<<S2>::UnitValueType>, 
{
	if *from_domain== *to_domain {
		println!("Using direct copy");
		DMatrix::<f64>::from_iterator(to_domain.len(), nc, data.iter().cloned())
	} else {
		let mfr = MatrixSlice::from_slice_generic(data, Dynamic::new(from_domain.len()), Dynamic::new(nc));
		let mut mto = DMatrix::<f64>::zeros(to_domain.len(), nc);
		for ip in from_domain.iter_interpolate(to_domain) {
			match ip {
				IterInterpolateType::Interpolate(j,i,h) => (0..nc).into_iter().for_each(|c|mto[(j,c)] = mfr[(i, c)]*(1.0-h)+mfr[(i+1,c)]*h),
				IterInterpolateType::RangeEnd(j,i) => (0..nc).into_iter().for_each(|c|mto[(j,c)] = mfr[(i, c)]),
				_ => () // extrapolation with 0.0 in this case, by default in mto
			}
		}
		mto
	}
}

#[test]
fn test_interp_col(){
	use super::{NONE5, NONE};
	use approx::assert_abs_diff_eq;
	let dfrom = Domain::new(0,2, NONE5);
	let dto = Domain::new(0,10, NONE);
	let data = [0.0, 5.0, 10.0]; // column major array in nalgebra
	let m = interp_cols(&dfrom, &dto, 1, &data);
	println!("{}", m);
	assert_abs_diff_eq!(m, DMatrix::<f64>::from_vec(11, 1, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]));
}

#[test]
fn test_interp_col2(){
	use super::{NONE5, NONE};
	use approx::assert_abs_diff_eq;
	let dfrom = Domain::new(0,2, NONE5);
	let dto = Domain::new(0,10, NONE);
	let data = [0.0, 5.0, 10.0, 0.0, 50.0, 100.0]; // column major array in nalgebra
	let m = interp_cols(&dfrom, &dto, 2, &data);
	println!("{}", m);
	assert_abs_diff_eq!(m, DMatrix::<f64>::from_vec(
		22, 
		1, 
		vec![
			0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
			0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
			]),
	/*epsilon = 1E-9*/);
}



/**
Sprague interpolation, using a 5th order polynomial fitted through 6 points.

The interpolating location h in the middle of the 6 point interval, between points 3 and points 4, scaled to a range
from 0 to 1, with 0 the location at point 3 and 1.0 for the location being at point 4. This interpolating method is
recommended by the *CIE* for spectral data with uniform spaced wavelengths.

See, "The interpolation method of Sprague-Karup", by Joseph L.F. De Kerf Journal of Computational and Applied
Mathematics, volume I, no 2, 1975.

*/
pub fn sprague(h: f64, v: [f64; 6]) -> f64 {
    let cf = [
        v[2],
        (v[0] - 8.0 * v[1] + 8.0 * v[3] - v[4]) / 12.0,
        (-v[0] + 16.0 * v[1] - 30.0 * v[2] + 16.0 * v[3] - v[4]) / 24.0,
        (-9.0 * v[0] + 39.0 * v[1] - 70.0 * v[2] + 66.0 * v[3] - 33.0 * v[4] + 7.0 * v[5]) / 24.0,
        (13.0 * v[0] - 64.0 * v[1] + 126.0 * v[2] - 124.0 * v[3] + 61.0 * v[4] - 12.0 * v[5])
            / 24.0,
        (-5.0 * v[0] + 25.0 * v[1] - 50.0 * v[2] + 50.0 * v[3] - 25.0 * v[4] + 5.0 * v[5]) / 24.0,
    ];
    cf.iter().rev().fold(0.0, |acc, coeff| acc * h + coeff)
}

const FRAC_EPS: f64 = 0.0001;

/**
    Interpolate matrix values by row, mapping values from one to another  domain.

    According to CIE recommendations, the data are padded with two repeated end point values at both ends, to get better
    interpolation results at both ends. This is not extrapolation of the data range, just to take care of the data at
    the ends within the range. For extrapolation values of 0.0 are used.

    This interpolation function allows for interpoation for values defined on different domains, with even different units.


*/

pub fn sprague_rows<S1, S2, R, C, S>(
    from_domain: &Domain<S1>,
    to_domain: &Domain<S2>,
    data: &Matrix<f64, R, C, S>,
) -> DMatrix<f64>
where
    S1: Step + Clone + Copy/* + Eq + PartialEq */,
    S2: Step + Clone + Copy/* + Eq + PartialEq */,
    S1::UnitValueType: From<<S2>::UnitValueType>, // need to be able to express a value in domain S2, as a value in domain S1
    R: Dim,
    C: Dim,
    S: Storage<f64, R, C>,
{
    let n = data.nrows(); // nr of vectors in the row matrix

    let mut values = Vec::<f64>::with_capacity(to_domain.len() * n);

    let start = from_domain.step.unitvalue(from_domain.range.start).value();
    let div = from_domain.step.unitvalue(1).value();
    let m = from_domain.len() - 1;

    for ut in to_domain {
        let from_domain_interval = (Into::<S1::UnitValueType>::into(ut).value() - start) / div;
        let index = from_domain_interval.floor() as isize;
        let undex = index as usize; // saturating cast since rust 1.45
        let frac = from_domain_interval.fract();
        for r in 0..n {
            // number of vectors
            values.push(match (index, undex, frac) {
                (_, u, h) if u >= 2 && u <= m - 3 => sprague( h, [ data[(r, u - 2)], data[(r, u - 1)], data[(r, u)], data[(r, u + 1)], data[(r, u + 2)], data[(r, u + 3)], ],), // most frequent condition
                (i, _, _) if i < 0 => 0.0,
                (_, 0, h) => sprague( h, [ data[(r, 0)], data[(r, 0)], data[(r, 0)], data[(r, 1)], data[(r, 2)], data[(r, 3)], ],),
                (_, 1, h) => sprague( h, [ data[(r, 0)], data[(r, 0)], data[(r, 1)], data[(r, 2)], data[(r, 3)], data[(r, 4)], ],),
                (_, u, h) if u == m - 2 => sprague( h, [ data[(r, m - 4)], data[(r, m - 3)], data[(r, m - 2)], data[(r, m - 1)], data[(r, m)], data[(r, m)], ],),
                (_, u, h) if u == m - 1 => sprague( h, [ data[(r, m - 3)], data[(r, m - 2)], data[(r, m - 1)], data[(r, m)], data[(r, m)], data[(r, m)], ],),
                (_, u, h) if u == m && h.abs() < FRAC_EPS => data[(r, m)],
                _ => 0.0,
            })
        }
    }
    DMatrix::from_vec(n, to_domain.len(), values)
}

#[test]
fn test_sprague_rows() {
    use crate::util::{NONE, NONE2};
    use nalgebra::matrix;
    let m_in = matrix!(
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0;
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0;
        1.0, 4.0, 9.0, 16.0, 16.0, 9.0, 4.0, 1.0;
    );

    println! {"{}", m_in};
    let from_domain = Domain::new(2, 9, NONE2);
    let to_domain2 = Domain::new(3, 19, NONE);

    println! {"{}", sprague_rows(&from_domain, &from_domain, &m_in)};
    println! {"{:.2}", sprague_rows(&from_domain, &to_domain2, &m_in)};
}

/**
    Interpolate matrix values by column, mapping values from one to another spectral domain.

*/

pub fn sprague_cols<S1, S2, R, C, S>(
    from_domain: &Domain<S1>,
    to_domain: &Domain<S2>,
    data: &Matrix<f64, R, C, S>,
) -> DMatrix<f64>
where
    S1: Step + Clone + Copy /*+ Eq + PartialEq*/,
    S2: Step + Clone + Copy /*+ Eq + PartialEq*/,
    S1::UnitValueType: From<<S2>::UnitValueType>, // need to be able to express a value in domain S2, as a value in domain S1
    R: Dim,
    C: Dim,
    S: Storage<f64, R, C>,
{
    let n = data.ncols(); // nr of vectors in the column matrix

    let mut values = Vec::<f64>::with_capacity(to_domain.len() * n);

    let start = from_domain.step.unitvalue(from_domain.range.start).value();
    let div = from_domain.step.unitvalue(1).value();
    let m = from_domain.len() - 1;

    for ut in to_domain {
        let from_domain_interval = (Into::<S1::UnitValueType>::into(ut).value() - start) / div;
        let index = from_domain_interval.floor() as isize;
        let undex = index as usize; // saturating cast since rust 1.45
        let frac = from_domain_interval.fract();
        for c in 0..n {
            // number of vectors
            values.push(match (index, undex, frac) {
                (_, u, h) if u >= 2 && u <= m - 3 => sprague( h, [ data[(u - 2, c)], data[(u - 1, c)], data[(u, c)], data[(u + 1, c)], data[(u + 2, c)], data[(u + 3, c)], ],), // most frequent condition
                (i, _, _) if i < 0 => 0.0,
                (_, 0, h) => sprague( h, [ data[(0, c)], data[(0, c)], data[(0, c)], data[(1, c)], data[(2, c)], data[(3, c)], ],),
                (_, 1, h) => sprague( h, [ data[(0, c)], data[(0, c)], data[(1, c)], data[(2, c)], data[(3, c)], data[(4, c)], ],),
                (_, u, h) if u == m - 2 => sprague( h, [ data[(m - 4, c)], data[(m - 3, c)], data[(m - 2, c)], data[(m - 1, c)], data[(m, c)], data[(m, c)], ],),
                (_, u, h) if u == m - 1 => sprague( h, [ data[(m - 3, c)], data[(m - 2, c)], data[(m - 1, c)], data[(m, c)], data[(m, c)], data[(m, c)], ],),
                (_, u, h) if u == m && h.abs() < FRAC_EPS => data[(m, c)],
                _ => 0.0,
            })
        }
    }
    DMatrix::from_vec(n, to_domain.len(), values).transpose()
}

#[test]
fn test_sprague_cols() {
    use crate::util::{NONE, NONE2};
    use nalgebra::matrix;
    let m_in = matrix!(
        1.0, 1.0, 1.0;
        1.0, 2.0, 4.0;
        1.0, 3.0, 9.0;
        1.0, 4.0, 16.0;
        1.0, 5.0, 16.0;
        1.0, 6.0, 9.0;
        1.0, 7.0, 4.0;
        1.0, 8.0, 1.0;
    );

    println! {"{}", m_in};
    let from_domain = Domain::new(2, 9, NONE2); // 4 to 18 in steps of 2
    let to_domain = Domain::new(3, 19, NONE);

    println! {"{}", sprague_cols(&from_domain, &from_domain, &m_in)};
    println! {"{}", sprague_cols(&from_domain, &to_domain, &m_in)};
}

/*

pub fn sprague_rows<R,C,S> (from_domain: SpectralDomain, to_domain: SpectralDomain, data: &Matrix<f64, R, C, S>) -> OMatrix<f64, R, Dynamic>
    where
        R: Dim + DimName,
        C: Dim + DimName,
        S: Storage<f64,R,C>,
        DefaultAllocator: Allocator<f64, R, Dynamic>,
{

    if from_domain == to_domain { // copy the data directly, no interpolation required
        //DMatrix::from_iterator(data.nrows(), data.ncols(), data.iter().cloned())
        todo!()
    }
    else {
        let n = data.nrows(); // nr of vectors in the row matrix

        let mut values = Vec::<f64>::with_capacity(to_domain.size * n);

        let i_max = from_domain.size - 1;

        for (c, f) in from_domain.iter_interpolate(to_domain).enumerate() { // counter and interpolation domain value
            let h = f.fract();
            for r in 0..n { // number of vectors
                values.push(
                    if f<0.0 || f>i_max as f64 {
                        0.0
                    } else {
                        match f.floor() as usize {
                            // point with at least three points to its left, and three points to its right
                            i if i>=2 && i<=i_max-3 =>
                                sprague(h,[data[(r,i-2)], data[(r,i-1)], data[(r,i)], data[(r,i+1)], data[(r,i+2)], data[(r, i+3)]]),

                            // take care of end points of the array
                            0 =>  sprague(h, [data[(r,0)], data[(r,0)], data[(r,0)], data[(r,1)], data[(r,2)], data[(r,3)]]),
                            1 =>  sprague(h, [data[(r,0)], data[(r,0)], data[(r,1)], data[(r,2)], data[(r,3)], data[(r,4)]]),
                            i if i == i_max-2 => sprague(h, [data[(r,i_max-4)], data[(r,i_max-3)], data[(r,i_max-2)], data[(r,i_max-1)], data[(r,i_max)], data[(r, i_max)]]),
                            i if i == i_max-1 => sprague(h, [data[(r,i_max-3)], data[(r,i_max-2)], data[(r,i_max-1)], data[(r,i_max)],   data[(r,i_max)], data[(r, i_max)]]),
                            i if i == i_max =>   sprague(h, [data[(r,i_max-2)], data[(r,i_max-1)], data[(r,i_max)],   data[(r,i_max)],   data[(r,i_max)], data[(r, i_max)]]),
                            _ => 0.0,
                        }
                    }

                )
            }
        }
    //	Matrix::<f64,R,C,S>::from_vec(R::name(), Dynamic::new(to_domain.size), values)
        OMatrix::from_data(DefaultAllocator::allocate_from_iterator(R::name(), Dynamic::new(to_domain.size), values.into_iter()))

    }
}

*/
