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

use nalgebra::{DMatrix, Dim, Matrix, Scalar, dmatrix, matrix, storage::Storage};

use crate::spectra::SpectralDomain;



/**
	Interpolate matrix values by row, mapping values from one to another spectral domain.

	According to CIE recommendations, four additional points are added for each of the vectors:
	two points at the short, and two point at the long wavelength side of the data set, and setting
	these points to the same value as the values of the end points, respectively. 
	
*/

pub fn sprague_row_mat<R, C, S> (from_domain: SpectralDomain, to_domain: SpectralDomain, data: &Matrix<f64, R, C, S>) -> DMatrix<f64> 
	where 
		R: Dim, 
		C: Dim, 
		S: Storage<f64,R,C>
{
	
	if from_domain == to_domain { // copy the data directly, no interpolation required
		DMatrix::from_iterator(data.nrows(), data.ncols(), data.iter().cloned())
	} 
	else {
		let n = data.nrows(); // nr of vectors in the row matrix
		let m = to_domain.size;
		let i_max = from_domain.size - 1;
		let mut dm =  DMatrix::<f64>::repeat(n, m, 0.0); 

		for (c, f) in from_domain.iter_interpolate(to_domain).enumerate() { // counter and interpolation domain value
			let h = f.fract();
			println!{"c,f,h {:?}", (c, f, h)};
			for r in 0..n { // number of vectors

				dm[(r,c)] = if f<0.0 || f>i_max as f64 {
					0.0
				} else {
					match f.floor() as usize {
						0 =>  sprague(h, [data[(r,0)], data[(r,0)], data[(r,0)], data[(r,1)], data[(r,2)], data[(r,3)]]),
						1 =>  sprague(h, [data[(r,0)], data[(r,0)], data[(r,1)], data[(r,2)], data[(r,3)], data[(r,4)]]),
						i if i>=2 && i<=i_max-3 => {
							println!{"r,c,f,i,i_max {:?}", (r, c, f, i, i_max)};
							sprague(h,[data[(r,i-2)], data[(r,i-1)], data[(r,i)], data[(r,i+1)], data[(r,i+2)], data[(r, i+3)]])
						},
						i if i == i_max-2 => sprague(h, [data[(r,i_max-4)], data[(r,i_max-3)], data[(r,i_max-2)], data[(r,i_max-1)], data[(r,i_max)], data[(r, i_max)]]),
						i if i == i_max-1 => sprague(h, [data[(r,i_max-3)], data[(r,i_max-2)], data[(r,i_max-1)], data[(r,i_max)],   data[(r,i_max)], data[(r, i_max)]]),
						i if i == i_max =>   sprague(h, [data[(r,i_max-2)], data[(r,i_max-1)], data[(r,i_max)],   data[(r,i_max)],   data[(r,i_max)], data[(r, i_max)]]),
						_ => 0.0,
					}
				};
			}
		}
		dm

	}
}

#[test]
fn test_sprague_mat_row(){
	let m_in = 
		DMatrix::from_vec(1, 8, vec![
			2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
			]);
	println!{"{:?}", m_in};
	let from_domain = SpectralDomain::new(2, 9 , 100 );
	let to_domain = SpectralDomain::new(4, 18, 50);
//	let m_out = sprague_row_mat::<f64,_,_,_>(from_domain, to_domain, &m_in);
	let m_out = sprague_row_mat(from_domain, to_domain, &m_in);
	println!{"{:?}", m_out};
}

#[test]
fn test_sprague_mat_row2(){
	let m_in = 
		DMatrix::from_vec(1, 8, vec![
			2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
			]);
	println!{"{:?}", m_in};
	let from_domain = SpectralDomain::new(2, 9, 100);
	let to_domain = SpectralDomain::new(3, 19, 50);
//	let m_out = sprague_row_mat::<f64,_,_,_>(from_domain, to_domain, &m_in);
	let m_out = sprague_row_mat(from_domain, to_domain, &m_in);
	println!{"{:?}", m_out};
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
