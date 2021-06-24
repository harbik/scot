use nalgebra::{Matrix3xX, SMatrix, matrix};

use crate::{observers::StandardObserver, spectra::SpectralDomain};

const N: usize = 95;

pub struct Cie1931 {
	data: SMatrix<f64, 3, N>,
	low: usize,
	unit: usize,
}

pub static CIE1931: Cie1931  = Cie1931 {
	data: matrix![ 
		0.0001299, 0.0002321, 0.0004149, 0.0007416, 0.001368, 0.002236, 0.004243, 0.00765, 0.01431, 0.02319, 0.04351,
		0.07763, 0.13438, 0.21477, 0.2839, 0.3285, 0.34828, 0.34806, 0.3362, 0.3187, 0.2908, 0.2511, 0.19536, 0.1421,
		0.09564, 0.05795001, 0.03201, 0.0147, 0.0049, 0.0024, 0.0093, 0.0291, 0.06327, 0.1096, 0.1655, 0.2257499,
		0.2904, 0.3597, 0.4334499, 0.5120501, 0.5945, 0.6784, 0.7621, 0.8425, 0.9163, 0.9786, 1.0263, 1.0567, 1.0622,
		1.0456, 1.0026, 0.9384, 0.8544499, 0.7514, 0.6424, 0.5419, 0.4479, 0.3608, 0.2835, 0.2187, 0.1649, 0.1212,
		0.0874, 0.0636, 0.04677, 0.0329, 0.0227, 0.01584, 0.01135916, 0.008110916, 0.005790346, 0.004109457,
		0.002899327, 0.00204919, 0.001439971, 0.000999949, 0.000690079, 0.000476021, 0.000332301, 0.000234826,
		0.000166151, 0.000117413, 8.30753E-05, 5.87065E-05, 4.15099E-05, 2.93533E-05, 2.06738E-05, 1.45598E-05,
		1.0254E-05, 7.22146E-06, 5.08587E-06, 3.58165E-06, 2.52253E-06, 1.77651E-06, 1.25114E-06;
		0.000003917, 0.000006965, 0.00001239, 0.00002202, 0.000039, 0.000064, 0.00012, 0.000217, 0.000396, 0.00064,
		0.00121, 0.00218, 0.004, 0.0073, 0.0116, 0.01684, 0.023, 0.0298, 0.038, 0.048, 0.06, 0.0739, 0.09098, 0.1126,
		0.13902, 0.1693, 0.20802, 0.2586, 0.323, 0.4073, 0.503, 0.6082, 0.71, 0.7932, 0.862, 0.9148501, 0.954, 0.9803,
		0.9949501, 1.0, 0.995, 0.9786, 0.952, 0.9154, 0.87, 0.8163, 0.757, 0.6949, 0.631, 0.5668, 0.503, 0.4412, 0.381,
		0.321, 0.265, 0.217, 0.175, 0.1382, 0.107, 0.0816, 0.061, 0.04458, 0.032, 0.0232, 0.017, 0.01192, 0.00821,
		0.005723, 0.004102, 0.002929, 0.002091, 0.001484, 0.001047, 0.00074, 0.00052, 0.0003611, 0.0002492, 0.0001719,
		0.00012, 0.0000848, 0.00006, 0.0000424, 0.00003, 0.0000212, 0.00001499, 0.0000106, 7.4657E-06, 5.2578E-06,
		3.7029E-06, 2.6078E-06, 1.8366E-06, 1.2934E-06, 9.1093E-07, 6.4153E-07, 4.5181E-07;
		0.0006061, 0.001086, 0.001946, 0.003486, 0.006450001, 0.01054999, 0.02005001, 0.03621, 0.06785001, 0.1102,
		0.2074, 0.3713, 0.6456, 1.0390501, 1.3856, 1.62296, 1.74706, 1.7826, 1.77211, 1.7441, 1.6692, 1.5281, 1.28764,
		1.0419, 0.8129501, 0.6162, 0.46518, 0.3533, 0.272, 0.2123, 0.1582, 0.1117, 0.07824999, 0.05725001, 0.04216,
		0.02984, 0.0203, 0.0134, 0.008749999, 0.005749999, 0.0039, 0.002749999, 0.0021, 0.0018, 0.001650001, 0.0014,
		0.0011, 0.001, 0.0008, 0.0006, 0.00034, 0.00024, 0.00019, 0.0001, 5E-05, 0.00003, 0.00002, 0.00001, 0.0, 0.0,
		0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
		0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0;
		],
	low:	72,
	unit:	50

};



impl StandardObserver for Cie1931 {
	fn domain(&self) -> SpectralDomain {
		SpectralDomain { low: self.low, unit: self.unit, size: 95}
	}

	/// Color matching functions mapped to a spectral domain
	fn cmf(&self, domain: SpectralDomain) -> Matrix3xX<f64> {
//		let c = SMatrix::<f64, 95, 3>::from_array_storage(ArrayStorage(self.data));
		todo!()
		
	}

}

