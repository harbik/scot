/*!
Color matching functions from Standard Observers, such as the CIE 1931 standard observer.

These observers, and their color matching functions, play a key role in Colorimetry, and are used througout this
library.  In particular, references to standard observers are used in many color models and collections of
chromaticity coordinates, to maintain consistency between all the different models and datasets, and to implement 
automatic transformations between the different mathematical representations of color.
*/


pub mod cie1931;

use nalgebra::{Matrix3xX};
use crate::util::{domain::{Domain}, units::Meter};

pub use crate::observers::cie1931::{Cie1931}; // allow use as observers::Cie1931 instead of observers::cie1931::Cie1931



/**
	Color matching functions mapped to a spectral data domain.

	A trait to get a standard observer chromatic responses, referred to as color matching functions
	x&#772;(&lambda;), y&#772;(&lambda;) z&#772;(&lambda;) by the CIE, as a matrix over target domain, typically the default
	domain for a spectral distribution. The mapping is typically done using a quadratic interpolation algorithm. Also
	analytical models of the CIE standard observers exist, which allows to do the mapping by a straightforward
	function evaluation.


	
 */
pub trait StandardObserver : 'static {

	/**
		Global, static reference to the standard observer, used in color model transformations.
	 */
	
	fn global() -> &'static Self; 

	/**
		Chromatic response mapped to a spectral domain, as a matrix with the x,y, and z color matching fuctions 
		as row vectors, with their length being dynamic, and determined by the standard's wavelength domain.
	*/
	fn cmf(&self, domain: Domain<Meter>) -> Matrix3xX<f64>;

	/// Domain associated with the data for the standard observer itself, as defined in their standard. 
	fn domain(&self) -> Domain<Meter>;
}



/**
	CIE1964 10º standard observer (data from [Coulour & Vision Research Laboratory UK](http://www.cvrl.org/cmfs.htm])
pub fn cie1964() -> Observer {
	let data = vec![
		1.222E-07, 1.3398E-08, 5.35027E-07, 9.1927E-07, 1.0065E-07, 4.0283E-06, 5.9586E-06, 6.511E-07, 2.61437E-05,
		0.000033266, 0.000003625, 0.00014622, 0.000159952, 0.000017364, 0.000704776, 0.00066244, 0.00007156, 0.0029278,
		0.0023616, 0.0002534, 0.0104822, 0.0072423, 0.0007685, 0.032344, 0.0191097, 0.0020044, 0.0860109,
		0.0434, 0.004509, 0.19712, 0.084736, 0.008756, 0.389366, 0.140638, 0.014456, 0.65676,
		0.204492, 0.021391, 0.972542, 0.264737, 0.029497, 1.2825, 0.314679, 0.038676, 1.55348,
		0.357719, 0.049602, 1.7985, 0.383734, 0.062077, 1.96728, 0.386726, 0.074704, 2.0273,
		0.370702, 0.089456, 1.9948, 0.342957, 0.106256, 1.9007, 0.302273, 0.128201, 1.74537,
		0.254085, 0.152761, 1.5549, 0.195618, 0.18519, 1.31756, 0.132349, 0.21994, 1.0302,
		0.080507, 0.253589, 0.772125, 0.041072, 0.297665, 0.57006, 0.016172, 0.339133, 0.415254,
		0.005132, 0.395379, 0.302356, 0.003816, 0.460777, 0.218502, 0.015444, 0.53136, 0.159249,
		0.037465, 0.606741, 0.112044, 0.071358, 0.68566, 0.082248, 0.117749, 0.761757, 0.060709,
		0.172953, 0.82333, 0.04305, 0.236491, 0.875211, 0.030451, 0.304213, 0.92381, 0.020584,
		0.376772, 0.961988, 0.013676, 0.451584, 0.9822, 0.007918, 0.529826, 0.991761, 0.003988,
		0.616053, 0.99911, 0.001091, 0.705224, 0.99734, 0.0, 0.793832, 0.98238, 0.0,
		0.878655, 0.955552, 0.0, 0.951162, 0.915175, 0.0, 1.01416, 0.868934, 0.0,
		1.0743, 0.825623, 0.0, 1.11852, 0.777405, 0.0, 1.1343, 0.720353, 0.0,
		1.12399, 0.658341, 0.0, 1.0891, 0.593878, 0.0, 1.03048, 0.527963, 0.0,
		0.95074, 0.461834, 0.0, 0.856297, 0.398057, 0.0, 0.75493, 0.339554, 0.0,
		0.647467, 0.283493, 0.0, 0.53511, 0.228254, 0.0, 0.431567, 0.179828, 0.0,
		0.34369, 0.140211, 0.0, 0.268329, 0.107633, 0.0, 0.2043, 0.081187, 0.0,
		0.152568, 0.060281, 0.0, 0.11221, 0.044096, 0.0, 0.0812606, 0.0318004, 0.0,
		0.05793, 0.0226017, 0.0, 0.0408508, 0.0159051, 0.0, 0.028623, 0.0111303, 0.0,
		0.0199413, 0.0077488, 0.0, 0.013842, 0.0053751, 0.0, 0.00957688, 0.00371774, 0.0,
		0.0066052, 0.00256456, 0.0, 0.00455263, 0.00176847, 0.0, 0.0031447, 0.00122239, 0.0,
		0.00217496, 0.00084619, 0.0, 0.0015057, 0.00058644, 0.0, 0.00104476, 0.00040741, 0.0,
		0.00072745, 0.000284041, 0.0, 0.000508258, 0.00019873, 0.0, 0.00035638, 0.00013955, 0.0,
		0.000250969, 0.000098428, 0.0, 0.00017773, 0.000069819, 0.0, 0.00012639, 0.000049737, 0.0,
		0.000090151, 3.55405E-05, 0.0, 6.45258E-05, 0.000025486, 0.0, 0.000046339, 1.83384E-05, 0.0,
		3.34117E-05, 0.000013249, 0.0, 0.000024209, 9.6196E-06, 0.0, 1.76115E-05, 7.0128E-06, 0.0,
		0.000012855, 5.1298E-06, 0.0, 9.41363E-06, 3.76473E-06, 0.0, 0.000006913, 2.77081E-06, 0.0,
		5.09347E-06, 2.04613E-06, 0.0, 3.7671E-06, 1.51677E-06, 0.0, 2.79531E-06, 1.12809E-06, 0.0,
		0.000002082, 8.4216E-07, 0.0, 1.55314E-06, 6.297E-07, 0.0
	];
	Observer::new(360/5, 50, data)
}

 */

#[test]
 fn test_cie1931(){
 }


