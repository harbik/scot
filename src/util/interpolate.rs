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


use nalgebra::{DMatrix, Dim, Matrix, storage::Storage};

use crate::util::domain::{Domain};
use crate::util::units::{Unit};

/**
	Sprague interpolation, using a 5th order polynomial fitted through 6 points.
	
	The interpolating location h in the middle of the 6 point interval, between points 3 and points 4, scaled to a range
	from 0 to 1, with 0 the location at point 3 and 1.0 for the location being at point 4. This interpolating method is
	recommended by the *CIE* for spectral data with uniform spaced wavelengths.

	See, "The interpolation method of Sprague-Karup", by Joseph L.F. De Kerf Journal of Computational and Applied
	Mathematics, volume I, no 2, 1975.

	*/
pub fn sprague(h: f64, v: [f64;6]) -> f64 
{
	let cf = [
		v[2],
		(v[0] - 8.0 * v[1] + 8.0 * v[3] - v[4]) / 12.0,
		(-v[0] + 16.0 * v[1] - 30.0 * v[2] + 16.0 * v[3] - v[4]) / 24.0,
		(-9.0 * v[0] + 39.0 * v[1] - 70.0 * v[2] + 66.0 * v[3] - 33.0 * v[4] + 7.0 * v[5]) / 24.0,
		(13.0 * v[0] - 64.0 * v[1] + 126.0 * v[2] - 124.0 * v[3] + 61.0 * v[4] - 12.0 * v[5]) / 24.0,
		(-5.0 * v[0] + 25.0 * v[1] - 50.0 * v[2] + 50.0 * v[3] - 25.0 * v[4] + 5.0 * v[5]) / 24.0			
	];
	cf.iter().rev().fold(0.0, | acc, coeff| acc * h + coeff)
}

/**
	Interpolate matrix values by row, mapping values from one to another spectral domain.

	According to CIE recommendations, the data are padded with two repeated end point values at both ends, to get better
	interpolation results at both ends. This is not extrapolation of the data range, just to take care of the data at
	the ends within the range. for extrapolation values of 0.0 are used.
	
*/

pub fn sprague_rows<U, R, C, S> (from_domain: &Domain<U>, to_domain: &Domain<U>, data: &Matrix<f64, R, C, S>) -> DMatrix<f64> 
	where 
		U: Unit + Clone + Copy + Eq + PartialEq, 
		R: Dim, 
		C: Dim, 
		S: Storage<f64,R,C>
{
	
	if from_domain == to_domain { // copy the data directly, no interpolation required
		DMatrix::from_iterator(data.nrows(), data.ncols(), data.iter().cloned())
	} 
	else {
		let n = data.nrows(); // nr of vectors in the row matrix

		let mut values = Vec::<f64>::with_capacity(to_domain.len() * n);

		let i_max = from_domain.len() - 1;

		for f in from_domain.iter_interpolate(to_domain) { // counter and interpolation domain value
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
		DMatrix::from_vec(n, to_domain.len(), values)

	}
}

#[test]
fn test_sprague_rows(){
	use nalgebra::matrix;
	use crate::util::units::{NM10, NM5};
	let m_in = 
		matrix!(
			1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0;
			1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0;
			1.0, 4.0, 9.0, 16.0, 16.0, 9.0, 4.0, 1.0;
		);

	println!{"{}", m_in};
	let from_domain = Domain::new(2, 9, NM10);
	let to_domain = Domain::new(3, 19, NM5);

	println!{"{}", sprague_rows(&from_domain, &from_domain, &m_in)};
	println!{"{}", sprague_rows(&from_domain, &to_domain, &m_in)};

}

/**
	Interpolate matrix values by column, mapping values from one to another spectral domain.
	
*/

pub fn sprague_cols<U, R, C, S> (from_domain: &Domain<U>, to_domain: &Domain<U>, data: &Matrix<f64, R, C, S>) -> DMatrix<f64> 
	where 
		U: Unit + Clone + Copy + Eq + PartialEq, 
		R: Dim, 
		C: Dim, 
		S: Storage<f64,R,C>
{
	
	if from_domain == to_domain { // copy the data directly, no interpolation required
		DMatrix::from_iterator(data.nrows(), data.ncols(), data.iter().cloned())
	} 
	else {
		let n = data.ncols(); // nr of vectors in the column matrix

		let mut values = Vec::<f64>::with_capacity(to_domain.len() * n);

		let i_max = from_domain.len() - 1;

		for c in 0..n { // number of vectors (columns in this case)
			for f in from_domain.iter_interpolate(&to_domain) { // counter and interpolation domain value
			let h = f.fract();
				values.push(
					if f<0.0 || f>i_max as f64 {
						0.0
					} else {
						match f.floor() as usize {
							// point with at least three points to its left, and three points to its right
							i if i>=2 && i<=i_max-3 => 
								sprague(h,[data[(i-2,c)], data[(i-1,c)], data[(i,c)], data[(i+1,c)], data[(i+2,c)], data[(i+3,c)]]),

							// take care of end points of the array, by padding them on the left, and the right
							0 =>  sprague(h, [data[(0,c)], data[(0,c)], data[(0,c)], data[(1,c)], data[(2,c)], data[(3,c)]]),
							1 =>  sprague(h, [data[(0,c)], data[(0,c)], data[(1,c)], data[(2,c)], data[(3,c)], data[(4,c)]]),
							i if i == i_max-2 => sprague(h, [data[(i_max-4,c)], data[(i_max-3,c)], data[(i_max-2,c)], data[(i_max-1,c)], data[(i_max,c)], data[(i_max,c)]]),
							i if i == i_max-1 => sprague(h, [data[(i_max-3,c)], data[(i_max-2,c)], data[(i_max-1,c)], data[(i_max,c)],   data[(i_max,c)], data[(i_max,c)]]),
							i if i == i_max =>   sprague(h, [data[(i_max-2,c)], data[(i_max-1,c)], data[(i_max,c)],   data[(i_max,c)],   data[(i_max,c)], data[(i_max,c)]]),
							_ => 0.0,
						}
					}

				)
			}
		}
		DMatrix::from_vec(to_domain.len(), n,  values)

	}
}

#[test]
fn test_sprague_cols(){
	use nalgebra::matrix;
	use crate::util::units::{NONE50, NONE100};
	let m_in = 
		matrix!(
			1.0, 1.0, 1.0;
			1.0, 2.0, 4.0;
			1.0, 3.0, 9.0;
			1.0, 4.0, 16.0;
			1.0, 5.0, 16.0;
			1.0, 6.0, 9.0;
			1.0, 7.0, 4.0;
			1.0, 8.0, 1.0;
		);

	println!{"{}", m_in};
	let from_domain = Domain::new(2, 9, NONE100);
	let to_domain = Domain::new(3, 19, NONE50);

	println!{"{}", sprague_cols(&from_domain, &from_domain, &m_in)};
	println!{"{}", sprague_cols(&from_domain, &to_domain, &m_in)};

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