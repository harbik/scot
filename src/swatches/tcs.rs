/*!
Test Color Samples as used for CRI Calculations.

CRI is the color rendering index, and defined by the CIE.
It is used to characterize light sources on their ability to render colors in comparison with a reference source such as 
daylight, or a blackbody source.
To test rendering, the CIE selected 14 representative color swatches as contained in this section of the Colorado library.
An additional, 15th, swatch, was added by a CIE comittee in Japan, to represent Japanese skin colors, and is added 
to the dataset below as well.
The data in this set is defined on a wavelength domain from 360 to 830nm, with steps of 5nm.

Source:
- Data obtaind from an "Excel spreadsheet with a cornucopia of data", published by the 
Lighting Laboratory of the Helsinki University of Technology, through Wikipedia

*/
use nalgebra::{ArrayStorage, SMatrix};

use crate::spectra::SpectralData;
use crate::util::domain::Domain;
use crate::util::units::{WavelengthScale, Scale, NM5};
use crate::util::interpolate::sprague_cols;

use super::Swatches;


const N: usize = 95; // number of points in a spectral distributions, and the number of rows in the column major spectral matrix
const M: usize = 15; // number of spectra in the set, or the number of columns in the spectral matrix

#[derive(Default)]
pub struct Tcs;

impl SpectralData for Tcs {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		sprague_cols(&self.domain(), &domain, &SMatrix::from_data(ArrayStorage(TCS)))
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(360/5, 830/5, NM5)
    }

	fn keys(&self) -> Option<Vec<String>> {
		Some(vec![
			"7.5 R 6/4|light greyish red".to_string(),
			"5 Y 6/4|Dark greyish yellow".to_string(),
			"5 GY 6/8|Strong yellow green".to_string(),
			"2.5 G 6/6|Moderate yellowish green".to_string(),
			"10 BG 6/4|Light bluish green".to_string(),
			"5 PB 6/8|Light blue".to_string(),
			"2.5 P 6/8|Light violet".to_string(),
			"10 P 6/8|Light reddish purple".to_string(),
			"4.5 R 4/13|Strong red".to_string(),
			"5 Y 8/10|Strong yellow".to_string(),
			"4.5 G 5/8|Strong green".to_string(),
			"3 PB 3/11|Strong blue".to_string(),
			"5 YR 8/4|Light yellowish pink".to_string(),
			"5 GY 4/4|Moderate olive green".to_string(),
			"|Japanese skin".to_string(),
		])
	}

	fn description(&self) -> Option<String> {
		Some("CRI Test Color Samples (TCS)".to_string())
	}
}

impl Swatches for Tcs {}

const TCS: [[f64;N];M] = [
	[0.116, 0.136, 0.159, 0.19, 0.219, 0.239, 0.252, 0.256, 0.256, 0.254, 0.252, 0.248, 0.244, 0.24, 0.237, 0.232, 0.23,
	0.226, 0.225, 0.222, 0.22, 0.218, 0.216, 0.214, 0.214, 0.214, 0.216, 0.218, 0.223, 0.225, 0.226, 0.226, 0.225,
	0.225, 0.227, 0.23, 0.236, 0.245, 0.253, 0.262, 0.272, 0.283, 0.298, 0.318, 0.341, 0.367, 0.39, 0.409, 0.424, 0.435,
	0.442, 0.448, 0.45, 0.451, 0.451, 0.451, 0.451, 0.451, 0.45, 0.45, 0.451, 0.451, 0.453, 0.454, 0.455, 0.457, 0.458,
	0.46, 0.462, 0.463, 0.464, 0.465, 0.466, 0.466, 0.466, 0.466, 0.467, 0.467, 0.467, 0.467, 0.467, 0.467, 0.467,
	0.467, 0.467, 0.467, 0.467, 0.466, 0.466, 0.466, 0.466, 0.466, 0.465, 0.464, 0.464],
	[0.053, 0.055, 0.059, 0.064, 0.07, 0.079, 0.089, 0.101, 0.111, 0.116, 0.118, 0.12, 0.121, 0.122, 0.122, 0.122,
	0.123, 0.124, 0.127, 0.128, 0.131, 0.134, 0.138, 0.143, 0.15, 0.159, 0.174, 0.19, 0.207, 0.225, 0.242, 0.253, 0.26,
	0.264, 0.267, 0.269, 0.272, 0.276, 0.282, 0.289, 0.299, 0.309, 0.322, 0.329, 0.335, 0.339, 0.341, 0.341, 0.342,
	0.342, 0.342, 0.341, 0.341, 0.339, 0.339, 0.338, 0.338, 0.337, 0.336, 0.335, 0.334, 0.332, 0.332, 0.331, 0.331,
	0.33, 0.329, 0.328, 0.328, 0.327, 0.326, 0.325, 0.324, 0.324, 0.324, 0.323, 0.322, 0.321, 0.32, 0.318, 0.316, 0.315,
	0.315, 0.314, 0.314, 0.313, 0.313, 0.312, 0.312, 0.311, 0.311, 0.311, 0.311, 0.311, 0.31],
	[0.058, 0.059, 0.061, 0.063, 0.065, 0.068, 0.07, 0.072, 0.073, 0.073, 0.074, 0.074, 0.074, 0.073, 0.073, 0.073,
	0.073, 0.073, 0.074, 0.075, 0.077, 0.08, 0.085, 0.094, 0.109, 0.126, 0.148, 0.172, 0.198, 0.221, 0.241, 0.26, 0.278,
	0.302, 0.339, 0.37, 0.392, 0.399, 0.4, 0.393, 0.38, 0.365, 0.349, 0.332, 0.315, 0.299, 0.285, 0.272, 0.264, 0.257,
	0.252, 0.247, 0.241, 0.235, 0.229, 0.224, 0.22, 0.217, 0.216, 0.216, 0.219, 0.224, 0.23, 0.238, 0.251, 0.269, 0.288,
	0.312, 0.34, 0.366, 0.39, 0.412, 0.431, 0.447, 0.46, 0.472, 0.481, 0.488, 0.493, 0.497, 0.5, 0.502, 0.505, 0.51,
	0.516, 0.52, 0.524, 0.527, 0.531, 0.535, 0.539, 0.544, 0.548, 0.552, 0.555],
	[0.057, 0.059, 0.062, 0.067, 0.074, 0.083, 0.093, 0.105, 0.116, 0.121, 0.124, 0.126, 0.128, 0.131, 0.135, 0.139,
	0.144, 0.151, 0.161, 0.172, 0.186, 0.205, 0.229, 0.254, 0.281, 0.308, 0.332, 0.352, 0.37, 0.383, 0.39, 0.394, 0.395,
	0.392, 0.385, 0.377, 0.367, 0.354, 0.341, 0.327, 0.312, 0.296, 0.28, 0.263, 0.247, 0.229, 0.214, 0.198, 0.185,
	0.175, 0.169, 0.164, 0.16, 0.156, 0.154, 0.152, 0.151, 0.149, 0.148, 0.148, 0.148, 0.149, 0.151, 0.154, 0.158,
	0.162, 0.165, 0.168, 0.17, 0.171, 0.17, 0.168, 0.166, 0.164, 0.164, 0.165, 0.168, 0.172, 0.177, 0.181, 0.185, 0.189,
	0.192, 0.194, 0.197, 0.2, 0.204, 0.21, 0.218, 0.225, 0.233, 0.243, 0.254, 0.264, 0.274],
	[0.143, 0.187, 0.233, 0.269, 0.295, 0.306, 0.31, 0.312, 0.313, 0.315, 0.319, 0.322, 0.326, 0.33, 0.334, 0.339,
	0.346, 0.352, 0.36, 0.369, 0.381, 0.394, 0.403, 0.41, 0.415, 0.418, 0.419, 0.417, 0.413, 0.409, 0.403, 0.396, 0.389,
	0.381, 0.372, 0.363, 0.353, 0.342, 0.331, 0.32, 0.308, 0.296, 0.284, 0.271, 0.26, 0.247, 0.232, 0.22, 0.21, 0.2,
	0.194, 0.189, 0.185, 0.183, 0.18, 0.177, 0.176, 0.175, 0.175, 0.175, 0.175, 0.177, 0.18, 0.183, 0.186, 0.189, 0.192,
	0.195, 0.199, 0.2, 0.199, 0.198, 0.196, 0.195, 0.195, 0.196, 0.197, 0.2, 0.203, 0.205, 0.208, 0.212, 0.215, 0.217,
	0.219, 0.222, 0.226, 0.231, 0.237, 0.243, 0.249, 0.257, 0.265, 0.273, 0.28],
	[0.079, 0.081, 0.089, 0.113, 0.151, 0.203, 0.265, 0.339, 0.41, 0.464, 0.492, 0.508, 0.517, 0.524, 0.531, 0.538,
	0.544, 0.551, 0.556, 0.556, 0.554, 0.549, 0.541, 0.531, 0.519, 0.504, 0.488, 0.469, 0.45, 0.431, 0.414, 0.395,
	0.377, 0.358, 0.341, 0.325, 0.309, 0.293, 0.279, 0.265, 0.253, 0.241, 0.234, 0.227, 0.225, 0.222, 0.221, 0.22, 0.22,
	0.22, 0.22, 0.22, 0.223, 0.227, 0.233, 0.239, 0.244, 0.251, 0.258, 0.263, 0.268, 0.273, 0.278, 0.281, 0.283, 0.286,
	0.291, 0.296, 0.302, 0.313, 0.325, 0.338, 0.351, 0.364, 0.376, 0.389, 0.401, 0.413, 0.425, 0.436, 0.447, 0.458,
	0.469, 0.477, 0.485, 0.493, 0.5, 0.506, 0.512, 0.517, 0.521, 0.525, 0.529, 0.532, 0.535],
	[0.15, 0.177, 0.218, 0.293, 0.378, 0.459, 0.524, 0.546, 0.551, 0.555, 0.559, 0.56, 0.561, 0.558, 0.556, 0.551,
	0.544, 0.535, 0.522, 0.506, 0.488, 0.469, 0.448, 0.429, 0.408, 0.385, 0.363, 0.341, 0.324, 0.311, 0.301, 0.291,
	0.283, 0.273, 0.265, 0.26, 0.257, 0.257, 0.259, 0.26, 0.26, 0.258, 0.256, 0.254, 0.254, 0.259, 0.27, 0.284, 0.302,
	0.324, 0.344, 0.362, 0.377, 0.389, 0.4, 0.41, 0.42, 0.429, 0.438, 0.445, 0.452, 0.457, 0.462, 0.466, 0.468, 0.47,
	0.473, 0.477, 0.483, 0.489, 0.496, 0.503, 0.511, 0.518, 0.525, 0.532, 0.539, 0.546, 0.553, 0.559, 0.565, 0.57,
	0.575, 0.578, 0.581, 0.583, 0.585, 0.587, 0.588, 0.589, 0.59, 0.59, 0.59, 0.591, 0.592],
	[0.075, 0.078, 0.084, 0.09, 0.104, 0.129, 0.17, 0.24, 0.319, 0.416, 0.462, 0.482, 0.49, 0.488, 0.482, 0.473, 0.462,
	0.45, 0.439, 0.426, 0.413, 0.397, 0.382, 0.366, 0.352, 0.337, 0.325, 0.31, 0.299, 0.289, 0.283, 0.276, 0.27, 0.262,
	0.256, 0.251, 0.25, 0.251, 0.254, 0.258, 0.264, 0.269, 0.272, 0.274, 0.278, 0.284, 0.295, 0.316, 0.348, 0.384,
	0.434, 0.482, 0.528, 0.568, 0.604, 0.629, 0.648, 0.663, 0.676, 0.685, 0.693, 0.7, 0.705, 0.709, 0.712, 0.715, 0.717,
	0.719, 0.721, 0.72, 0.719, 0.722, 0.725, 0.727, 0.729, 0.73, 0.73, 0.73, 0.73, 0.73, 0.73, 0.73, 0.73, 0.73, 0.73,
	0.73, 0.731, 0.731, 0.731, 0.731, 0.731, 0.731, 0.731, 0.731, 0.731],
	[0.069, 0.072, 0.073, 0.07, 0.066, 0.062, 0.058, 0.055, 0.052, 0.052, 0.051, 0.05, 0.05, 0.049, 0.048, 0.047, 0.046,
	0.044, 0.042, 0.041, 0.038, 0.035, 0.033, 0.031, 0.03, 0.029, 0.028, 0.028, 0.028, 0.029, 0.03, 0.03, 0.031, 0.031,
	0.032, 0.032, 0.033, 0.034, 0.035, 0.037, 0.041, 0.044, 0.048, 0.052, 0.06, 0.076, 0.102, 0.136, 0.19, 0.256, 0.336,
	0.418, 0.505, 0.581, 0.641, 0.682, 0.717, 0.74, 0.758, 0.77, 0.781, 0.79, 0.797, 0.803, 0.809, 0.814, 0.819, 0.824,
	0.828, 0.83, 0.831, 0.833, 0.835, 0.836, 0.836, 0.837, 0.838, 0.839, 0.839, 0.839, 0.839, 0.839, 0.839, 0.839,
	0.839, 0.839, 0.839, 0.839, 0.839, 0.839, 0.838, 0.837, 0.837, 0.836, 0.836],
	[0.042, 0.043, 0.045, 0.047, 0.05, 0.054, 0.059, 0.063, 0.066, 0.067, 0.068, 0.069, 0.069, 0.07, 0.072, 0.073,
	0.076, 0.078, 0.083, 0.088, 0.095, 0.103, 0.113, 0.125, 0.142, 0.162, 0.189, 0.219, 0.262, 0.305, 0.365, 0.416,
	0.465, 0.509, 0.546, 0.581, 0.61, 0.634, 0.653, 0.666, 0.678, 0.687, 0.693, 0.698, 0.701, 0.704, 0.705, 0.705,
	0.706, 0.707, 0.707, 0.707, 0.708, 0.708, 0.71, 0.711, 0.712, 0.714, 0.716, 0.718, 0.72, 0.722, 0.725, 0.729, 0.731,
	0.735, 0.739, 0.742, 0.746, 0.748, 0.749, 0.751, 0.753, 0.754, 0.755, 0.755, 0.755, 0.755, 0.756, 0.757, 0.758,
	0.759, 0.759, 0.759, 0.759, 0.759, 0.759, 0.759, 0.759, 0.759, 0.758, 0.757, 0.757, 0.756, 0.756],
	[0.074, 0.079, 0.086, 0.098, 0.111, 0.121, 0.127, 0.129, 0.127, 0.121, 0.116, 0.112, 0.108, 0.105, 0.104, 0.104,
	0.105, 0.106, 0.11, 0.115, 0.123, 0.134, 0.148, 0.167, 0.192, 0.219, 0.252, 0.291, 0.325, 0.347, 0.356, 0.353,
	0.346, 0.333, 0.314, 0.294, 0.271, 0.248, 0.227, 0.206, 0.188, 0.17, 0.153, 0.138, 0.125, 0.114, 0.106, 0.1, 0.096,
	0.092, 0.09, 0.087, 0.085, 0.082, 0.08, 0.079, 0.078, 0.078, 0.078, 0.078, 0.081, 0.083, 0.088, 0.093, 0.102, 0.112,
	0.125, 0.141, 0.161, 0.182, 0.203, 0.223, 0.242, 0.257, 0.27, 0.282, 0.292, 0.302, 0.31, 0.314, 0.317, 0.323, 0.33,
	0.334, 0.338, 0.343, 0.348, 0.353, 0.359, 0.365, 0.372, 0.38, 0.388, 0.396, 0.403],
	[0.189, 0.175, 0.158, 0.139, 0.12, 0.103, 0.09, 0.082, 0.076, 0.068, 0.064, 0.065, 0.075, 0.093, 0.123, 0.16, 0.207,
	0.256, 0.3, 0.331, 0.346, 0.347, 0.341, 0.328, 0.307, 0.282, 0.257, 0.23, 0.204, 0.178, 0.154, 0.129, 0.109, 0.09,
	0.075, 0.062, 0.051, 0.041, 0.035, 0.029, 0.025, 0.022, 0.019, 0.017, 0.017, 0.017, 0.016, 0.016, 0.016, 0.016,
	0.016, 0.016, 0.016, 0.016, 0.018, 0.018, 0.018, 0.018, 0.019, 0.02, 0.023, 0.024, 0.026, 0.03, 0.035, 0.043, 0.056,
	0.074, 0.097, 0.128, 0.166, 0.21, 0.257, 0.305, 0.354, 0.401, 0.446, 0.485, 0.52, 0.551, 0.577, 0.599, 0.618, 0.633,
	0.645, 0.656, 0.666, 0.674, 0.68, 0.686, 0.691, 0.694, 0.697, 0.7, 0.702],
	[0.071, 0.076, 0.082, 0.09, 0.104, 0.127, 0.161, 0.211, 0.264, 0.313, 0.341, 0.352, 0.359, 0.361, 0.364, 0.365,
	0.367, 0.369, 0.372, 0.374, 0.376, 0.379, 0.384, 0.389, 0.397, 0.405, 0.416, 0.429, 0.443, 0.454, 0.461, 0.466,
	0.469, 0.471, 0.474, 0.476, 0.483, 0.49, 0.506, 0.526, 0.553, 0.582, 0.618, 0.651, 0.68, 0.701, 0.717, 0.729, 0.736,
	0.742, 0.745, 0.747, 0.748, 0.748, 0.748, 0.748, 0.748, 0.748, 0.748, 0.748, 0.747, 0.747, 0.747, 0.747, 0.747,
	0.747, 0.747, 0.746, 0.746, 0.746, 0.745, 0.744, 0.743, 0.744, 0.745, 0.748, 0.75, 0.75, 0.749, 0.748, 0.748, 0.747,
	0.747, 0.747, 0.747, 0.746, 0.746, 0.746, 0.746, 0.745, 0.745, 0.745, 0.745, 0.745, 0.745],
	[0.036, 0.036, 0.036, 0.036, 0.036, 0.036, 0.037, 0.038, 0.039, 0.039, 0.04, 0.041, 0.042, 0.042, 0.043, 0.044,
	0.044, 0.045, 0.045, 0.046, 0.047, 0.048, 0.05, 0.052, 0.055, 0.057, 0.062, 0.067, 0.075, 0.083, 0.092, 0.1, 0.108,
	0.121, 0.133, 0.142, 0.15, 0.154, 0.155, 0.152, 0.147, 0.14, 0.133, 0.125, 0.118, 0.112, 0.106, 0.101, 0.098, 0.095,
	0.093, 0.09, 0.089, 0.087, 0.086, 0.085, 0.084, 0.084, 0.084, 0.084, 0.085, 0.087, 0.092, 0.096, 0.102, 0.11, 0.123,
	0.137, 0.152, 0.169, 0.188, 0.207, 0.226, 0.243, 0.26, 0.277, 0.294, 0.31, 0.325, 0.339, 0.353, 0.366, 0.379, 0.39,
	0.399, 0.408, 0.416, 0.422, 0.428, 0.434, 0.439, 0.444, 0.448, 0.451, 0.454],
	[0.131, 0.131, 0.131, 0.131, 0.131, 0.139, 0.147, 0.153, 0.158, 0.162, 0.164, 0.167, 0.17, 0.175, 0.182, 0.192,
	0.203, 0.212, 0.221, 0.229, 0.236, 0.243, 0.249, 0.254, 0.259, 0.264, 0.269, 0.276, 0.284, 0.291, 0.296, 0.298,
	0.296, 0.289, 0.282, 0.276, 0.274, 0.276, 0.281, 0.286, 0.291, 0.289, 0.286, 0.28, 0.285, 0.314, 0.354, 0.398, 0.44,
	0.47, 0.494, 0.511, 0.524, 0.535, 0.544, 0.552, 0.559, 0.565, 0.571, 0.576, 0.581, 0.586, 0.59, 0.594, 0.599, 0.603,
	0.606, 0.61, 0.612, 0.614, 0.616, 0.616, 0.616, 0.616, 0.615, 0.613, 0.612, 0.61, 0.609, 0.608, 0.607, 0.607, 0.609,
	0.61, 0.611, 0.611, 0.611, 0.611, 0.611, 0.611, 0.611, 0.611, 0.611, 0.611, 0.611],

];


#[test]
fn test_tcs(){
	use crate::models::Lab;
	use crate::illuminants::D65;
	use crate::observers::Cie1931;

	let tcs_lab: Lab<Cie1931,D65> = Tcs::default().into();

	println!("{:.4}", tcs_lab.data.transpose());


}