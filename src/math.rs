use num::Float;

/**
	Get index and minimum value of a float sice.
 */

pub fn min_slice<F:Float + PartialEq>(vec: &[F]) -> (usize, F) {
	let mut imin = 0usize;
	let mut vmin =  F::infinity();
	for (i, v) in vec.iter().enumerate() {
		if v<&vmin {
			imin = i;
			vmin = *v;
		}
	}
	(imin, vmin)
}

pub fn simpson<F>(sp: F, a: f64, b: f64, n: usize) -> f64
where
	F: Fn(f64) -> f64,
{
	let h = (b - a) / n as f64;
	let mut sum = sp(a) + sp(b);
	let mut x = a + h;
	for i in 1..n {
		if i % 2 == 1 {
			sum += 4. * sp(x)
		} else {
			sum += 2. * sp(x)
		}
		x += h;
	}
	sum * h / 3.0
}

#[inline]
pub fn linterp(x:f64, x1:f64, y1:f64, x2:f64, y2:f64) -> f64 {
	let s = (x - x1)/(x2 - x1);
	y1 * (1.0 - s) + y2 * s
}
