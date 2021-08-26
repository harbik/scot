/*!
Data types and algorithms for Correlated Color Temperature calculations.

Natural light is for large part generated by thermal emission, and characterized by the temperature of its source.
The incandescent lamp, the most important of electric light of the 20th century, is also a thermal radiator.
Emission from other lamps, such as fluorescent and LED, is not related to their physical temperature.
However, it is common practice to characterize their color appearance through comparison with a thermal source.
As reference for such a thermal source a blackbody radiator is used.
The correlated color temperature of a source is the physical temperature of a blackbody radiator, which matches the color of the source best.
*/


use nalgebra::{MatrixXx2};

/**
A collection tmperature (in kelvin) and radiant exitance values (in watt per square meter), both qualified to be positive and greater than 0.0.
Used as input to create `Blackbody` and `CIEDaylight` collections.
*/
#[derive(Debug,Clone)]
pub struct CctParameters (
	pub MatrixXx2<f64>
);

impl std::ops::Deref for CctParameters {
    type Target = MatrixXx2<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CctParameters {
	/**
		Construct a collection of correlated temperature and radiant exitance values.
		It accepts a range of arguments, such as a single `f64` absolute temperature value, arrays and vectors of `f64` temperature values, defaulting to radiant exitance values of 1W/m2.
		To specify temperature and power, use arrays and vectors of `\[f64;2\]' values, with the first value of the
		`\[f64;2\]` array being an absolute temperature, and the second being the radiante exitance specification.

		# Examples
		From a single temperature value, and using a default value for power of 1.0W:
		```
		use colorado::illuminants::CCTs;

		let ccts = CCTs::new(3000.0);
		assert_eq!(ccts.0[(0,0)], 3000.0);
		assert_eq!(ccts.0[(0,1)], 1.0);
		```
		From an array of float values:
		```
		use colorado::illuminants::CCTs;

		let ccts = CCTs::new([3000.0, 4000.0, 5000.0]);
		assert_eq!(ccts.0[(0,0)], 3000.0); assert_eq!(ccts.0[(0,1)], 1.0);
		assert_eq!(ccts.0[(1,0)], 4000.0); assert_eq!(ccts.0[(1,1)], 1.0);
		assert_eq!(ccts.0[(2,0)], 5000.0); assert_eq!(ccts.0[(2,1)], 1.0);
		```
		And here we create a CCTs collection from a two-dimensional array with temperature and radiant exitance values:
		```
		use colorado::illuminants::CCTs;

		let ccts = CCTs::new([[3000.0, 3.0], [4000.0, 4.0], [5000.0, 5.0]]);
		assert_eq!(ccts.0[(0,0)], 3000.0); assert_eq!(ccts.0[(0,1)], 3.0);
		assert_eq!(ccts.0[(1,0)], 4000.0); assert_eq!(ccts.0[(1,1)], 4.0);
		assert_eq!(ccts.0[(2,0)], 5000.0); assert_eq!(ccts.0[(2,1)], 5.0);
		```
		And this works for vectors of \[f64;2\] too:
		```
		use colorado::illuminants::CCTs;

		let ccts = CCTs::new(vec![[3000.0, 3.0], [4000.0, 4.0], [5000.0, 5.0]]);
		assert_eq!(ccts.0[(0,0)], 3000.0); assert_eq!(ccts.0[(0,1)], 3.0);
		assert_eq!(ccts.0[(1,0)], 4000.0); assert_eq!(ccts.0[(1,1)], 4.0);
		assert_eq!(ccts.0[(2,0)], 5000.0); assert_eq!(ccts.0[(2,1)], 5.0);
		```
	*/

	pub fn new(t: impl Into<CctParameters>) -> CctParameters {
		t.into()

	}

	/**
	 */
	 pub fn keys(&self) -> Option<Vec<String>> {
		 if self.nrows() > 0 {
			let mut v : Vec<String> = Vec::with_capacity(self.nrows());
			for r in self.0.row_iter() {
				v.push(format!("{}", r[0]));
			}
			Some(v)
		 } else {
			 None
		 }
	 }

	/**
		The minimum value of temperatures in a CCT collection.
		# Example
		```
		use colorado::illuminants::CCTs;

		let cct_min = CCTs::new([5000.0, 8000.0, 3000.0]).min();
		assert_eq!(cct_min, 3000.0);
		```
	*/
	pub fn min(&self) -> f64 {
		self.column(0).min()
	}

	/**
		The maximum value of temperatures in a CCT collection.
		# Example
		```
		use colorado::illuminants::CCTs;

		let cct_max = CCTs::new([5000.0, 8000.0, 3000.0]).max();
		assert_eq!(cct_max, 8000.0);
		```
	*/
	pub fn max(&self) -> f64 {
		self.column(0).max()
	}

	pub fn len(&self) -> usize {
		self.0.nrows()
	}
}


impl<'a> IntoIterator for &'a CctParameters {
    type Item = (f64, f64);

    type IntoIter = CctIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
		CctIterator {
			ccts: &self.0,
			i: 0,
			end: self.len(),
		}
    }
}

pub struct CctIterator<'a> {
	ccts: &'a MatrixXx2<f64>,
	i: usize,
	end: usize,
}

impl<'a> Iterator for CctIterator<'a> {
	type Item = (f64, f64);

	fn next (&mut self) -> Option<Self::Item> {
		let c = self.i;
		if c< self.end {
			self.i += 1;
			Some((self.ccts[(c,0)], self.ccts[(c,1)]))
		} else {
			None
		}
	}

}



#[test]
fn test_cct_iterator(){
	let ccts = CctParameters::new([[3000.0,1.0],[4000.0,2.0],[5000.0,3.0]]);
	for (t,p) in &ccts {
		println!("{:?} {:?}", t, p);
	}
}

impl From<Vec<f64>> for CctParameters {

	/// Creates a CCTs array from a vector of temperatures, each with a power of 1W
	fn from(t: Vec<f64>) -> Self {
		let p = vec![1.0; t.len()];
		Self(MatrixXx2::from_iterator(t.len(), t.into_iter().chain(p.into_iter())))
	}
}

// From a float value
impl From<f64> for CctParameters {

	/// CCTs array from a single temperature
	fn from(t: f64) -> Self {
		Self::new([[t, 1.0]])
	}
}

// From a single, positive, integer value
impl From<usize> for CctParameters {

	/// CCTs array from a single temperature
	fn from(t: usize) -> Self {
		Self::new([[t as f64, 1.0]])
	}
}

// From array of temperature values
impl <const N: usize> From<[f64; N]> for CctParameters {

	fn from(t: [f64;N]) -> Self {
		Self(MatrixXx2::from_fn(N,  |r, c| {
			if c==0 {
				assert!(t[r]>0.0, "Correlated color temperature should be >0.0K");
				t[r]
			} else {
				1.0
			}
		}))
	}
}

// Array of 2 element temperature and power arrays
impl <const N: usize> From<[[f64; 2];N]> for CctParameters {

	fn from (t: [[f64;2];N]) -> Self {
		Self(MatrixXx2::from_fn(N, |r, c| {
			assert!(t[r][c]>0.0, "Correlated color temperature and radiant power should be >0.0K");
			t[r][c]
		}))
	}

}

// Vector of 2 elemente temperature and power arrays
impl From<Vec<[f64; 2]>> for CctParameters {

	fn from (t: Vec<[f64;2]>) -> Self {
		Self(MatrixXx2::from_fn(t.len(), |r, c| {
			assert!(t[r][c]>0.0, "Correlated color temperature and radiant power should be >0.0K");
			t[r][c]}
		))
	}
}



