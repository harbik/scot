

use nalgebra::{ArrayStorage, DMatrix, Matrix3xX, SMatrix, SVector};

use crate::spectra::{SpectralData};
use crate::illuminants::cct::{CCTs};
use crate::util::domain::Domain;
use crate::util::units::{WavelengthScale, Scale, NM5};
use crate::util::interpolate::{sprague_cols};

use super::Illuminant;

pub type D50 = CieIllD50;
pub type D55 = CieIllD55;
pub type D65 = CieIllD65;
pub type D75 = CieIllD75;
pub type CieD = CieDaylight;


/**
	Spectral distributions of one or multiple generic blackbody illuminants.
	
	Each of the blackbody sources is characterized by a temperature, in units of Kelvin, and radiant exitance
	with unit W/m<sup>2</sup>. Through a `CCTs` helper class, it accepts multiple ways to specify the 
	temperatures and exitance you want &mdash; see this class for examples.

	The spectral power distribution for blackbody radiators is calculated using Planck's law.
	The `values` method of the `SpectralDistribution` trait produces spectral radiant exitance values
	over the range of the input domain, and at equidistant spacing. Besides the usual wavelength domains,
	you can also use other domains with units which implement the Wavelength trait

	# Examples
	A blackbody radiator, with a temperature of 3000K, and a irradiance of 1W/m<sup>2</sup>.
	Here a single integer valued argument is used to specify a blackbody's temperature.

	```
	use colorado::illuminants::Blackbody;
	use colorado::observers::Cie1931;
	use colorado::cie::XYZ;
	use approx::assert_abs_diff_eq;

	let bb = Blackbody::new(3000);
	let xyz = XYZ::<Cie1931>::from(bb);
	```

	# Examples
	Y
	A blackbody radiator, with a temperature of 3000K, and an illuminance of 0.1W/m<sup>2</sup>.
	Here a single integer valued argument is used to specify a blackbody's temperature.

	```
	use colorado::illuminants::Blackbody;
	use colorado::observers::Cie1931;
	use colorado::util::domain::Domain;
	use colorado::spectra::SpectralDistribution;
	use colorado::cie::XYZ;
	use colorado::util::units::DEV; // dEv 
	use approx::assert_abs_diff_eq;

	let sdbb = Blackbody::new([[6500.0,0.1]]);
	let v = sdbb.values(Domain::new(15, 33, DEV)); // values for blackbody radiator from 1.5 (826.56nm) to 3.3 eV (375.709)
	let val : Vec<f64> = v.into_iter().cloned().collect();
	assert_eq!(val, vec![]);
	```
 */

#[derive(Debug)]
pub struct CieDaylight {
	pub ccts: CCTs,
}

impl CieDaylight {

	pub fn new(parameters: impl Into<CCTs>) -> CieDaylight
	{
		CieDaylight {
			ccts: parameters.into(),
		}
	}
}

impl SpectralData for CieDaylight {

	type ScaleType = WavelengthScale;

	/**
		Spectral values for the CIE D illuminant.

		Calculates CIE Daylight spectral values for a target domain.
		This `UnitValue` item type of target domain's Unit doesn't have to be a `Meter` value, but needs to be
		able to be converted into a `Meter` value, typically done by implementing a `From<X> for Meter` trait.
	 */
	fn values<L: Scale>(&self, dom: &Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<<Self as SpectralData>::ScaleType as Scale>::UnitType: From<<L>::UnitType>
	 {

		let s_interpolated = sprague_cols(&self.domain(), &dom, &SMatrix::<f64,107,3>::from_data(ArrayStorage(S)));


		let mut mvec : Vec<f64> = Vec::with_capacity(3 * dom.len());
		for (t,p) in &self.ccts {
			let cct = t.clamp(4000.0,25000.0);
			let xd = match cct {
				t if t<7000.0 => 0.244063 + 0.09911E3 / t + 2.9678E6 / t.powi(2) - 4.607E9 / t.powi(3),
				_ => 0.23704 + 0.24748E3 / t + 1.9018E6 / t.powi(2) - 2.0064E9 / t.powi(3)
			};
			let yd = -3. * xd.powi(2) + 2.87 * xd - 0.275;
			let m = 0.0241 + 0.2562 * xd - 0.7341 * yd;
			let m1 = (-1.3515 - 1.7703 * xd + 5.9114 * yd) / m;
			let m2 = (0.03 - 31.4424 * xd + 30.0717 * yd) / m;
			let sum_power = (8715.51 + m1 * 890.13 + m2 * 374.95) * 5.0;
			let scale = p / sum_power;
			mvec.push(scale);
			mvec.push(m1 * scale);
			mvec.push(m2 * scale);
		}
		let mmat = Matrix3xX::from_vec(mvec);
		s_interpolated * mmat
	//	DMatrix::from_vec(dom.len(), self.ccts.len(), v)

	}

	fn description(&self) -> Option<String> {
		Some("Daylight Illuminants".to_string())
	}

	/// String temperature values for each of the blackbody sources in the collection.
	fn keys(&self) -> Option<Vec<String>> {
		self.ccts.keys()
	}

	/// Domain which covering the visible part of the spectrum
	fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::new(60, 166, NM5)
	}
	
}

pub struct CieIllD50(SVector<f64, 97>);

impl SpectralData for CieIllD50 {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		sprague_cols(&self.domain(), &domain, &self.0)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::new(60, 156, NM5)
    }

    fn keys(&self) -> Option<Vec<String>> { 
		Some(vec!["D50".to_string()])
	 }

    fn description(&self) -> Option<String> { 
		Some("CIE D50 Standard Illuminant".to_string())
	}
}

impl Default for CieIllD50 {
    fn default() -> Self {
		Self(SVector::<f64, 97>::from_data(ArrayStorage::<f64,97,1>(D50_DATA)))
    }
}

impl Illuminant for CieIllD50 {}

#[test]
fn test_d50(){
	use crate::observers::CieObs1931;
	use crate::models;
	use approx::assert_abs_diff_eq;


	let d50xyz: models::CieYxy<CieObs1931> = CieIllD50::default().into();
	assert_abs_diff_eq!(d50xyz.data.column(0).y, 0.34567 , epsilon = 5E-5);  // CIE 15:2004, Table T.3. D50 x value
	assert_abs_diff_eq!(d50xyz.data.column(0).z, 0.35851 , epsilon = 5E-5);  // CIE 15:2004, Table T.3. D50 y value - there is a slight deviation here... 50 vs 51
} 
pub struct CieIllD55(SVector<f64, 97>);

impl SpectralData for CieIllD55 {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		sprague_cols(&self.domain(), &domain, &self.0)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::new(60, 156, NM5)
    }

    fn keys(&self) -> Option<Vec<String>> { 
		Some(vec!["D55".to_string()])
	 }

    fn description(&self) -> Option<String> { 
		Some("CIE D55 Standard Illuminant".to_string())
	}
}

impl Default for CieIllD55 {
    fn default() -> Self {
		Self(SVector::<f64, 97>::from_data(ArrayStorage::<f64,97,1>(D55_DATA)))
    }
}

impl Illuminant for CieIllD55 {}

#[test]
fn test_d55(){
	use crate::observers::CieObs1931;
	use crate::models;
	use approx::assert_abs_diff_eq;

	let c: models::CieXYZ<CieObs1931> = CieIllD55::default().into();
	println!("{}", c);

	let d: models::CieYxy<CieObs1931> = CieIllD55::default().into();
	assert_abs_diff_eq!(d.data.column(0).y, 0.33243 , epsilon = 5E-5);  // CIE 15:2004, Table T.3. D55 x value
	assert_abs_diff_eq!(d.data.column(0).z, 0.34744 , epsilon = 5E-5);  // CIE 15:2004, Table T.3. D55 y value - there is a slight deviation here... 50 vs 51
} 

pub struct CieIllD65(SVector<f64, 97>);

impl SpectralData for CieIllD65 {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		sprague_cols(&self.domain(), &domain, &self.0)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::new(60, 156, NM5)
    }

    fn keys(&self) -> Option<Vec<String>> { 
		Some(vec!["D65".to_string()])
	 }

    fn description(&self) -> Option<String> { 
		Some("CIE D65 Standard Illuminant".to_string())
	}
}

impl Default for CieIllD65 {
    fn default() -> Self {
		Self(SVector::<f64, 97>::from_data(ArrayStorage::<f64,97,1>(D65_DATA)))
    }
}

impl Illuminant for CieIllD65 {}

#[test]
fn test_d65(){
	use crate::observers::CieObs1931;
	use crate::models;
	use approx::assert_abs_diff_eq;


	let xyz: models::CieXYZ<CieObs1931> = CieIllD65::default().into();
	assert_abs_diff_eq!(xyz.data.column(0).x, 6.859677E-3 , epsilon = 2E-6);  //  CIE 15:2004 Tables, calculated in Excel
	assert_abs_diff_eq!(xyz.data.column(0).y, 7.217449E-3, epsilon = 2E-6);  // 
	assert_abs_diff_eq!(xyz.data.column(0).z, 7.858362E-3, epsilon = 2E-6);  // 

	let yxy: models::CieYxy<CieObs1931> = CieIllD65::default().into();
	assert_abs_diff_eq!(yxy.data.column(0).y, 0.31272 , epsilon = 5E-5);  // CIE 15:2004, Table T.3. D65 x value
	assert_abs_diff_eq!(yxy.data.column(0).z, 0.32903 , epsilon = 5E-5);  // CIE 15:2004, Table T.3. D65 y value
} 

pub struct CieIllD75(SVector<f64, 97>);

impl SpectralData for CieIllD75 {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		sprague_cols(&self.domain(), &domain, &self.0)
    }

    fn domain(&self) -> Domain<Self::ScaleType> {
		Domain::new(60, 156, NM5)
    }

    fn keys(&self) -> Option<Vec<String>> { 
		Some(vec!["D75".to_string()])
	 }

    fn description(&self) -> Option<String> { 
		Some("CIE D75 Standard Illuminant".to_string())
	}
}

impl Default for CieIllD75 {
    fn default() -> Self {
		Self(SVector::<f64, 97>::from_data(ArrayStorage::<f64,97,1>(D75_DATA)))
    }
}
impl Illuminant for CieIllD75 {}

#[test]
fn test_d75(){
	use crate::observers::CieObs1931;
	use crate::models;
	use approx::assert_abs_diff_eq;


	let xyz: models::CieYxy<CieObs1931> = CieIllD75::default().into();
	assert_abs_diff_eq!(xyz.data.column(0).y, 0.29903, epsilon = 5E-5);  // CIE 15:2004, Table T.3. D75 x value
	assert_abs_diff_eq!(xyz.data.column(0).z, 0.31488, epsilon = 5E-5);  // CIE 15:2004, Table T.3. D75 y value
} 


/**
	Data below from CIE 15:2004 Excel tables.
 */
const S : [[f64; 107]; 3] = [
	 [
		 0.04, 3.02, 6.0, 17.8, 29.6, 42.45, 55.3, 56.3, 57.3, 59.55, 61.8, 61.65, 61.5, 65.15, 68.8, 66.1, 63.4, 64.6,
		65.8, 80.3, 94.8, 99.8, 104.8, 105.35, 105.9, 101.35, 96.8, 105.35, 113.9, 119.75, 125.6, 125.55, 125.5, 123.4,
		121.3, 121.3, 121.3, 117.4, 113.5, 113.3, 113.1, 111.95, 110.8, 108.65, 106.5, 107.65, 108.8, 107.05, 105.3,
		104.85, 104.4, 102.2, 100.0, 98.0, 96.0, 95.55, 95.1, 92.1, 89.1, 89.8, 90.5, 90.4, 90.3, 89.35, 88.4, 86.2, 84.0,
		84.55, 85.1, 83.5, 81.9, 82.25, 82.6, 83.75, 84.9, 83.1, 81.3, 76.6, 71.9, 73.1, 74.3, 75.35, 76.4, 69.85, 63.3,
		67.5, 71.7, 74.35, 77.0, 71.1, 65.2, 56.45, 47.7, 58.15, 68.6, 66.8, 65.0, 65.5, 66.0, 63.5, 61.0, 57.15, 53.3,
		56.1, 58.9, 60.4, 61.9
	 ],[
		 0.02, 2.26, 4.5, 13.45, 22.4, 32.2, 42.0, 41.3, 40.6, 41.1, 41.6, 39.8, 38.0, 40.2, 42.4, 40.45, 38.5, 36.75,
		 35.0, 39.2, 43.4, 44.85, 46.3, 45.1, 43.9, 40.5, 37.1, 36.9, 36.7, 36.3, 35.9, 34.25, 32.6, 30.25, 27.9, 26.1,
		 24.3, 22.2, 20.1, 18.15, 16.2, 14.7, 13.2, 10.9, 8.6, 7.35, 6.1, 5.15, 4.2, 3.05, 1.9, 0.95, 0.0, -0.8, -1.6,
		 -2.55, -3.5, -3.5, -3.5, -4.65, -5.8, -6.5, -7.2, -7.9, -8.6, -9.05, -9.5, -10.2, -10.9, -10.8, -10.7, -11.35,
		 -12.0, -13.0, -14.0, -13.8, -13.6, -12.8, -12.0, -12.65, -13.3, -13.1, -12.9, -11.75, -10.6, -11.1, -11.6,
		 -11.9, -12.2, -11.2, -10.2, -9.0, -7.8, -9.5, -11.2, -10.8, -10.4, -10.5, -10.6, -10.15, -9.7, -9.0, -8.3,
		 -8.8, -9.3, -9.55, -9.8
	 ],[
		 0.0, 1.0, 2.0, 3.0, 4.0, 6.25, 8.5, 8.15, 7.8, 7.25, 6.7, 6.0, 5.3, 5.7, 6.1, 4.55, 3.0, 2.1, 1.2, 0.05, -1.1,
		 -0.8, -0.5, -0.6, -0.7, -0.95, -1.2, -1.9, -2.6, -2.75, -2.9, -2.85, -2.8, -2.7, -2.6, -2.6, -2.6, -2.2, -1.8,
		 -1.65, -1.5, -1.4, -1.3, -1.25, -1.2, -1.1, -1.0, -0.75, -0.5, -0.4, -0.3, -0.15, 0.0, 0.1, 0.2, 0.35, 0.5,
		 1.3, 2.1, 2.65, 3.2, 3.65, 4.1, 4.4, 4.7, 4.9, 5.1, 5.9, 6.7, 7.0, 7.3, 7.95, 8.6, 9.2, 9.8, 10.0, 10.2, 9.25,
		 8.3, 8.95, 9.6, 9.05, 8.5, 7.75, 7.0, 7.3, 7.6, 7.8, 8.0, 7.35, 6.7, 5.95, 5.2, 6.3, 7.4, 7.1, 6.8, 6.9, 7.0,
		 6.7, 6.4, 5.95, 5.5, 5.8, 6.1, 6.3, 6.5
	]
];

const D50_DATA: [[f64; 97];1] = [[
	0.019, 1.035, 2.051, 4.914, 7.778, 11.263, 14.748, 16.348, 17.948, 19.479, 21.010, 22.476, 23.942, 25.451, 26.961,
	25.724, 24.488, 27.179, 29.871, 39.589, 49.308, 52.910, 56.513, 58.273, 60.034, 58.926, 57.818, 66.321, 74.825,
	81.036, 87.247, 88.930, 90.612, 90.990, 91.368, 93.238, 95.109, 93.536, 91.963, 93.843, 95.724, 96.169, 96.613,
	96.871, 97.129, 99.614, 102.099, 101.427, 100.755, 101.536, 102.317, 101.159, 100.000, 98.868, 97.735, 98.327,
	98.918, 96.208, 93.499, 95.593, 97.688, 98.478, 99.269, 99.155, 99.042, 97.382, 95.722, 97.290, 98.857, 97.262,
	95.667, 96.929, 98.190, 100.597, 103.003, 101.068, 99.133, 93.257, 87.381, 89.492, 91.604, 92.246, 92.889, 84.872,
	76.854, 81.683, 86.511, 89.546, 92.580, 85.405, 78.230, 67.961, 57.692, 70.307, 82.923, 80.599, 78.274
]];

const D55_DATA: [[f64; 97];1] = [[
	0.024, 1.048, 2.072, 6.648, 11.224, 15.936, 20.647, 22.266, 23.885, 25.851, 27.817, 29.219, 30.621, 32.464, 34.308,
	33.446, 32.584, 35.335, 38.087, 49.518, 60.949, 64.751, 68.554, 70.065, 71.577, 69.746, 67.914, 76.760, 85.605, 91.799,
	97.993, 99.228, 100.463, 100.188, 99.913, 101.326, 102.739, 100.409, 98.078, 99.379, 100.680, 100.688, 100.695, 100.341,
	99.987, 102.098, 104.210, 103.156, 102.102, 102.535, 102.968, 101.484, 100.000, 98.608, 97.216, 97.482, 97.749, 94.590,
	91.432, 92.926, 94.419, 94.780, 95.140, 94.680, 94.220, 92.334, 90.448, 91.389, 92.330, 90.592, 88.854, 89.586, 90.317,
	92.133, 93.950, 91.953, 89.956, 84.817, 79.677, 81.258, 82.840, 83.842, 84.844, 77.539, 70.235, 74.768, 79.301, 82.147,
	84.993, 78.437, 71.880, 62.337, 52.793, 64.360, 75.927, 73.872, 71.818
]];

const D65_DATA: [[f64; 97];1] = [[
	0.034100, 1.664300, 3.294500, 11.765200, 20.236000, 28.644700, 37.053500, 38.501100, 39.948800, 42.430200, 44.911700, 45.775000, 46.638300, 49.363700,
	52.089100, 51.032300, 49.975500, 52.311800, 54.648200, 68.701500, 82.754900, 87.120400, 91.486000, 92.458900, 93.431800, 90.057000, 86.682300, 95.773600,
	104.865000, 110.936000, 117.008000, 117.410000, 117.812000, 116.336000, 114.861000, 115.392000, 115.923000, 112.367000, 108.811000, 109.082000,
	109.354000, 108.578000, 107.802000, 106.296000, 104.790000, 106.239000, 107.689000, 106.047000, 104.405000, 104.225000, 104.046000, 102.023000,
	100.000000, 98.167100, 96.334200, 96.061100, 95.788000, 92.236800, 88.685600, 89.345900, 90.006200, 89.802600, 89.599100, 88.648900, 87.698700, 85.493600,
	83.288600, 83.493900, 83.699200, 81.863000, 80.026800, 80.120700, 80.214600, 81.246200, 82.277800, 80.281000, 78.284200, 74.002700, 69.721300, 70.665200,
	71.609100, 72.979000, 74.349000, 67.976500, 61.604000, 65.744800, 69.885600, 72.486300, 75.087000, 69.339800, 63.592700, 55.005400, 46.418200, 56.611800,
	66.805400, 65.094100, 63.382800
	
]];

const D75_DATA: [[f64; 97];1] = [[
	0.043, 2.588, 5.133, 17.470, 29.808, 42.369, 54.930, 56.095, 57.259, 60.000, 62.740, 62.861, 62.982, 66.647, 70.312,
	68.507, 66.703, 68.333, 69.963, 85.946, 101.929, 106.911, 111.894, 112.346, 112.798, 107.945, 103.092, 112.145, 121.198,
	127.104, 133.010, 132.682, 132.355, 129.838, 127.322, 127.061, 126.800, 122.291, 117.783, 117.186, 116.589, 115.146,
	113.702, 111.181, 108.659, 109.552, 110.445, 108.367, 106.289, 105.596, 104.904, 102.452, 100.000, 97.808, 95.616,
	94.914, 94.213, 90.605, 86.997, 87.112, 87.227, 86.684, 86.140, 84.861, 83.581, 81.164, 78.747, 78.587, 78.428, 76.614,
	74.801, 74.562, 74.324, 74.873, 75.422, 73.499, 71.576, 67.714, 63.852, 64.464, 65.076, 66.573, 68.070, 62.256, 56.443,
	60.343, 64.242, 66.697, 69.151, 63.890, 58.629, 50.623, 42.617, 51.985, 61.352, 59.838, 58.324
]];