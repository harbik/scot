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