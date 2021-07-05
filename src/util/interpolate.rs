
use nalgebra::{DMatrix, Dim, Matrix, Scalar, storage::Storage};

use crate::spectra::SpectralDomain;



/**
	Interpolate matrix values by row, as in the definition of data based standard observers. T
*/

pub fn sprague_row_mat<T, R, C, S> (from_domain: SpectralDomain, to_domain: SpectralDomain, data: &Matrix<f64, R, C, S>) -> DMatrix<f64> 
	where 
		T: Scalar + std::fmt::Debug,
		R: Dim, 
		C: Dim, 
		S: Storage<f64,R,C>
{
	
	if from_domain == to_domain {
		DMatrix::from_iterator(data.nrows(), data.ncols(), data.iter().cloned())
	} 
	else {
		let m_rows = data.nrows();
		let m_cols = to_domain.size;
		let mut dm =  DMatrix::<f64>::repeat(m_rows, m_cols, 0.0); 

		for (c, f) in from_domain.iter_interpolate(to_domain) { // counter and interpolation domain value
			let h = f.fract();
			for r in 0..m_rows { // number of vectors

				dm[(r,c)] = if f<0.0 {
					0.0
				} else {
					match f.floor() as usize {
						i if i>=2 || i<m_cols-2 => sprague(h,[data[(r,i-2)], data[(r,i-1)], data[(r,i)], data[(r,i+1)], data[(r,i+2)], data[(r, i+3)]]),
						0 =>  sprague(h, [data[(r,0)], data[(r,0)], data[(r,0)], data[(r,1)], data[(r,2)], data[(r,3)]]),
						1 =>  sprague(h, [data[(r,0)], data[(r,0)], data[(r,1)], data[(r,2)], data[(r,3)], data[(r,4)]]),
						i if i == m_cols-2 => sprague(h, [data[(r,m_cols-4)], data[(r,m_cols-3)], data[(r,m_cols-2)], data[(r,m_cols-1)], data[(r,m_cols-1)], data[(r, m_cols-1)]]),
						i if i == m_cols-1 => sprague(h, [data[(r,m_cols-3)], data[(r,m_cols-2)], data[(r,m_cols-1)], data[(r,m_cols-1)], data[(r,m_cols-1)], data[(r, m_cols-1)]]),
						i if i == m_cols => sprague(h, [data[(r,m_cols-2)], data[(r,m_cols-1)], data[(r,m_cols-1)], data[(r,m_cols-1)], data[(r,m_cols-1)], data[(r, m_cols-1)]]),
						_ => 0.0,
					}
				};
			}
		}
		dm

	}
}

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
