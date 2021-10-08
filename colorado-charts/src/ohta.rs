/*!

# Ohta Dataset

Dat from N. Ohta, "The Basis of Color Reproduction Engineering (Japanese)", published by Corona-sha Co of Japan in 1997.
Reflectivity measured over a spectral domain from 380 to 780nm, with steps of 5nm.

*/

use colorado as cd;
use cd::{SpectralDistribution, Domain, WavelengthStep, NM5};
use cd::illuminants::Illuminant;
use cd::models::CieLab;
use cd::observers::StandardObserver;
use cd::swatches::Swatch;
use nalgebra::{ 
    SMatrix, 
    matrix
};
use crate::{
    M, // nr of swatches
};


const N: usize = 81;


#[derive(Debug, Default)]
pub struct CheckerOhta;

impl SpectralDistribution for CheckerOhta {
    type MatrixType = SMatrix<f64,N,M>;
    type StepType = WavelengthStep;

    fn shape(&self) -> (usize, usize) {
        (N, M)
    }

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (
            Domain::new(380/5, 780/5, NM5),
            CHECKER_OHTA
        )
    }
}
impl Swatch for CheckerOhta{}

impl<I:Illuminant, C:StandardObserver> From<CheckerOhta> for CieLab<I,C> 
where
    <<I as cd::SpectralDistribution>::StepType as cd::Step>::UnitValueType: From<cd::Meter>,
{   fn from(chk: CheckerOhta) -> Self  {
        chk.lab()
    }
}

#[test]
fn test_ohta(){
    use colorado::{illuminants::D50, models::{CieLab, CieCamUcs, VcAvg}, observers::CieObs1931};
    let ohtalab : CieLab<D50, CieObs1931> = CheckerOhta.into();
    let ohtacam : CieCamUcs<VcAvg, D50, CieObs1931> = CheckerOhta.into();
    println!("{}", ohtalab.data.transpose());
    println!("{}", ohtacam.data.transpose());
}


static CHECKER_OHTA: SMatrix<f64,81, 24> = matrix!{
0.048, 0.103, 0.113, 0.048, 0.123, 0.110, 0.053, 0.099, 0.096, 0.101, 0.056, 0.060, 0.069, 0.055, 0.052, 0.054, 0.118, 0.093, 0.153, 0.150, 0.138, 0.113, 0.074, 0.032;
0.051, 0.120, 0.138, 0.049, 0.152, 0.133, 0.054, 0.120, 0.108, 0.115, 0.058, 0.061, 0.081, 0.056, 0.052, 0.053, 0.142, 0.110, 0.189, 0.184, 0.167, 0.131, 0.079, 0.033;
0.055, 0.141, 0.174, 0.049, 0.197, 0.167, 0.054, 0.150, 0.123, 0.135, 0.059, 0.063, 0.096, 0.057, 0.052, 0.054, 0.179, 0.134, 0.245, 0.235, 0.206, 0.150, 0.084, 0.033;
0.060, 0.163, 0.219, 0.049, 0.258, 0.208, 0.054, 0.189, 0.135, 0.157, 0.059, 0.064, 0.114, 0.058, 0.052, 0.053, 0.228, 0.164, 0.319, 0.299, 0.249, 0.169, 0.088, 0.034;
0.065, 0.182, 0.266, 0.050, 0.328, 0.252, 0.054, 0.231, 0.144, 0.177, 0.060, 0.065, 0.136, 0.058, 0.051, 0.053, 0.283, 0.195, 0.409, 0.372, 0.289, 0.183, 0.091, 0.035;
0.068, 0.192, 0.300, 0.049, 0.385, 0.284, 0.054, 0.268, 0.145, 0.191, 0.061, 0.065, 0.156, 0.058, 0.051, 0.053, 0.322, 0.220, 0.536, 0.459, 0.324, 0.193, 0.093, 0.035;
0.068, 0.197, 0.320, 0.049, 0.418, 0.303, 0.053, 0.293, 0.144, 0.199, 0.061, 0.064, 0.175, 0.059, 0.050, 0.053, 0.343, 0.238, 0.671, 0.529, 0.346, 0.199, 0.094, 0.036;
0.067, 0.199, 0.330, 0.050, 0.437, 0.314, 0.053, 0.311, 0.141, 0.203, 0.061, 0.064, 0.193, 0.059, 0.050, 0.052, 0.354, 0.249, 0.772, 0.564, 0.354, 0.201, 0.094, 0.036;
0.064, 0.201, 0.336, 0.050, 0.446, 0.322, 0.052, 0.324, 0.138, 0.206, 0.062, 0.064, 0.208, 0.059, 0.049, 0.052, 0.359, 0.258, 0.840, 0.580, 0.357, 0.202, 0.094, 0.036;
0.062, 0.203, 0.337, 0.051, 0.448, 0.329, 0.052, 0.335, 0.134, 0.198, 0.063, 0.064, 0.224, 0.060, 0.049, 0.052, 0.357, 0.270, 0.868, 0.584, 0.358, 0.203, 0.094, 0.036;
0.059, 0.205, 0.337, 0.052, 0.448, 0.336, 0.052, 0.348, 0.132, 0.190, 0.064, 0.064, 0.244, 0.062, 0.049, 0.053, 0.350, 0.281, 0.878, 0.585, 0.359, 0.203, 0.094, 0.036;
0.057, 0.208, 0.337, 0.053, 0.447, 0.344, 0.052, 0.361, 0.132, 0.179, 0.066, 0.065, 0.265, 0.063, 0.049, 0.053, 0.339, 0.296, 0.882, 0.587, 0.360, 0.204, 0.095, 0.036;
0.055, 0.212, 0.335, 0.054, 0.444, 0.353, 0.052, 0.373, 0.131, 0.168, 0.068, 0.065, 0.290, 0.065, 0.049, 0.053, 0.327, 0.315, 0.883, 0.587, 0.361, 0.205, 0.095, 0.035;
0.054, 0.217, 0.334, 0.056, 0.440, 0.363, 0.052, 0.383, 0.131, 0.156, 0.071, 0.066, 0.316, 0.067, 0.049, 0.054, 0.313, 0.334, 0.885, 0.588, 0.362, 0.205, 0.095, 0.035;
0.053, 0.224, 0.331, 0.058, 0.434, 0.375, 0.052, 0.387, 0.129, 0.144, 0.075, 0.067, 0.335, 0.070, 0.049, 0.055, 0.298, 0.352, 0.886, 0.588, 0.362, 0.205, 0.095, 0.035;
0.053, 0.231, 0.327, 0.060, 0.428, 0.390, 0.052, 0.383, 0.128, 0.132, 0.079, 0.068, 0.342, 0.074, 0.048, 0.056, 0.282, 0.370, 0.886, 0.587, 0.361, 0.205, 0.094, 0.035;
0.052, 0.240, 0.322, 0.061, 0.421, 0.408, 0.052, 0.374, 0.126, 0.120, 0.085, 0.069, 0.338, 0.078, 0.048, 0.059, 0.267, 0.391, 0.887, 0.586, 0.361, 0.204, 0.094, 0.035;
0.052, 0.251, 0.316, 0.063, 0.413, 0.433, 0.052, 0.361, 0.126, 0.110, 0.093, 0.073, 0.324, 0.084, 0.047, 0.065, 0.253, 0.414, 0.888, 0.585, 0.359, 0.204, 0.094, 0.035;
0.052, 0.262, 0.310, 0.064, 0.405, 0.460, 0.053, 0.345, 0.125, 0.101, 0.104, 0.077, 0.302, 0.091, 0.047, 0.075, 0.239, 0.434, 0.888, 0.583, 0.358, 0.203, 0.094, 0.035;
0.053, 0.273, 0.302, 0.065, 0.394, 0.492, 0.054, 0.325, 0.123, 0.093, 0.118, 0.084, 0.273, 0.101, 0.046, 0.093, 0.225, 0.449, 0.888, 0.582, 0.358, 0.203, 0.093, 0.035;
0.054, 0.282, 0.293, 0.067, 0.381, 0.523, 0.055, 0.301, 0.119, 0.086, 0.135, 0.092, 0.239, 0.113, 0.045, 0.121, 0.209, 0.458, 0.888, 0.581, 0.357, 0.202, 0.093, 0.034;
0.055, 0.289, 0.285, 0.068, 0.372, 0.548, 0.056, 0.275, 0.114, 0.080, 0.157, 0.100, 0.205, 0.125, 0.045, 0.157, 0.195, 0.461, 0.888, 0.580, 0.356, 0.202, 0.093, 0.034;
0.057, 0.293, 0.276, 0.070, 0.362, 0.566, 0.057, 0.247, 0.109, 0.075, 0.185, 0.107, 0.172, 0.140, 0.044, 0.202, 0.182, 0.457, 0.888, 0.580, 0.356, 0.202, 0.093, 0.034;
0.059, 0.296, 0.268, 0.072, 0.352, 0.577, 0.059, 0.223, 0.105, 0.070, 0.221, 0.115, 0.144, 0.157, 0.044, 0.252, 0.172, 0.447, 0.888, 0.580, 0.356, 0.202, 0.092, 0.034;
0.061, 0.301, 0.260, 0.078, 0.342, 0.582, 0.061, 0.202, 0.103, 0.067, 0.269, 0.123, 0.120, 0.180, 0.044, 0.303, 0.163, 0.433, 0.887, 0.580, 0.356, 0.202, 0.092, 0.034;
0.062, 0.310, 0.251, 0.088, 0.330, 0.583, 0.064, 0.184, 0.102, 0.063, 0.326, 0.133, 0.101, 0.208, 0.044, 0.351, 0.155, 0.414, 0.887, 0.580, 0.356, 0.202, 0.093, 0.034;
0.065, 0.321, 0.243, 0.106, 0.314, 0.580, 0.068, 0.167, 0.100, 0.061, 0.384, 0.146, 0.086, 0.244, 0.044, 0.394, 0.146, 0.392, 0.887, 0.580, 0.356, 0.202, 0.093, 0.034;
0.067, 0.326, 0.234, 0.130, 0.294, 0.576, 0.076, 0.152, 0.097, 0.059, 0.440, 0.166, 0.074, 0.286, 0.044, 0.436, 0.135, 0.366, 0.887, 0.581, 0.356, 0.202, 0.093, 0.034;
0.070, 0.322, 0.225, 0.155, 0.271, 0.569, 0.086, 0.137, 0.094, 0.058, 0.484, 0.193, 0.066, 0.324, 0.044, 0.475, 0.124, 0.339, 0.887, 0.581, 0.357, 0.202, 0.093, 0.034;
0.072, 0.310, 0.215, 0.173, 0.249, 0.560, 0.101, 0.125, 0.091, 0.056, 0.516, 0.229, 0.059, 0.351, 0.044, 0.512, 0.113, 0.310, 0.887, 0.582, 0.357, 0.202, 0.093, 0.034;
0.074, 0.298, 0.208, 0.181, 0.231, 0.549, 0.120, 0.116, 0.089, 0.054, 0.534, 0.273, 0.054, 0.363, 0.044, 0.544, 0.106, 0.282, 0.887, 0.582, 0.357, 0.203, 0.093, 0.034;
0.075, 0.291, 0.203, 0.182, 0.219, 0.535, 0.143, 0.110, 0.090, 0.053, 0.542, 0.323, 0.051, 0.363, 0.044, 0.572, 0.102, 0.255, 0.887, 0.582, 0.358, 0.203, 0.093, 0.034;
0.076, 0.292, 0.198, 0.177, 0.211, 0.519, 0.170, 0.106, 0.092, 0.052, 0.545, 0.374, 0.048, 0.355, 0.045, 0.597, 0.102, 0.228, 0.887, 0.583, 0.358, 0.203, 0.093, 0.034;
0.078, 0.297, 0.195, 0.168, 0.209, 0.501, 0.198, 0.103, 0.096, 0.052, 0.541, 0.418, 0.046, 0.342, 0.046, 0.615, 0.105, 0.204, 0.886, 0.583, 0.358, 0.203, 0.093, 0.034;
0.079, 0.300, 0.191, 0.157, 0.209, 0.480, 0.228, 0.099, 0.102, 0.053, 0.533, 0.456, 0.045, 0.323, 0.047, 0.630, 0.107, 0.180, 0.886, 0.583, 0.358, 0.203, 0.093, 0.034;
0.082, 0.298, 0.188, 0.147, 0.207, 0.458, 0.260, 0.094, 0.106, 0.054, 0.524, 0.487, 0.044, 0.303, 0.048, 0.645, 0.107, 0.159, 0.887, 0.584, 0.358, 0.203, 0.092, 0.034;
0.087, 0.295, 0.183, 0.137, 0.201, 0.436, 0.297, 0.090, 0.108, 0.055, 0.513, 0.512, 0.043, 0.281, 0.050, 0.660, 0.106, 0.141, 0.887, 0.584, 0.359, 0.203, 0.093, 0.033;
0.092, 0.295, 0.177, 0.129, 0.196, 0.414, 0.338, 0.086, 0.109, 0.055, 0.501, 0.534, 0.042, 0.260, 0.053, 0.673, 0.107, 0.126, 0.887, 0.585, 0.359, 0.203, 0.093, 0.033;
0.100, 0.305, 0.172, 0.126, 0.196, 0.392, 0.380, 0.083, 0.112, 0.054, 0.487, 0.554, 0.041, 0.238, 0.057, 0.686, 0.112, 0.114, 0.888, 0.586, 0.360, 0.204, 0.093, 0.033;
0.107, 0.326, 0.167, 0.125, 0.199, 0.369, 0.418, 0.083, 0.126, 0.053, 0.472, 0.570, 0.041, 0.217, 0.063, 0.698, 0.123, 0.104, 0.888, 0.587, 0.361, 0.204, 0.093, 0.033;
0.115, 0.358, 0.163, 0.122, 0.206, 0.346, 0.452, 0.083, 0.157, 0.052, 0.454, 0.584, 0.040, 0.196, 0.072, 0.708, 0.141, 0.097, 0.887, 0.588, 0.361, 0.205, 0.093, 0.033;
0.122, 0.397, 0.160, 0.119, 0.215, 0.324, 0.481, 0.085, 0.208, 0.052, 0.436, 0.598, 0.040, 0.177, 0.086, 0.718, 0.166, 0.092, 0.886, 0.588, 0.361, 0.205, 0.093, 0.033;
0.129, 0.435, 0.157, 0.115, 0.223, 0.302, 0.503, 0.086, 0.274, 0.053, 0.416, 0.609, 0.040, 0.158, 0.109, 0.726, 0.198, 0.088, 0.886, 0.588, 0.361, 0.205, 0.093, 0.033;
0.134, 0.468, 0.153, 0.109, 0.229, 0.279, 0.520, 0.087, 0.346, 0.055, 0.394, 0.617, 0.040, 0.140, 0.143, 0.732, 0.235, 0.083, 0.886, 0.588, 0.361, 0.205, 0.092, 0.033;
0.138, 0.494, 0.150, 0.104, 0.235, 0.260, 0.532, 0.087, 0.415, 0.059, 0.374, 0.624, 0.039, 0.124, 0.192, 0.737, 0.279, 0.080, 0.887, 0.588, 0.360, 0.204, 0.092, 0.033;
0.142, 0.514, 0.147, 0.100, 0.241, 0.245, 0.543, 0.086, 0.473, 0.065, 0.358, 0.630, 0.039, 0.111, 0.256, 0.742, 0.333, 0.077, 0.888, 0.587, 0.360, 0.204, 0.092, 0.033;
0.146, 0.530, 0.144, 0.098, 0.245, 0.234, 0.552, 0.085, 0.517, 0.074, 0.346, 0.635, 0.040, 0.101, 0.332, 0.746, 0.394, 0.075, 0.889, 0.586, 0.359, 0.204, 0.092, 0.033;
0.150, 0.541, 0.141, 0.097, 0.245, 0.226, 0.560, 0.084, 0.547, 0.086, 0.337, 0.640, 0.040, 0.094, 0.413, 0.749, 0.460, 0.074, 0.890, 0.586, 0.358, 0.203, 0.091, 0.033;
0.154, 0.550, 0.137, 0.098, 0.243, 0.221, 0.566, 0.084, 0.567, 0.099, 0.331, 0.645, 0.040, 0.089, 0.486, 0.753, 0.522, 0.073, 0.891, 0.585, 0.357, 0.203, 0.091, 0.033;
0.158, 0.557, 0.133, 0.100, 0.243, 0.217, 0.572, 0.085, 0.582, 0.113, 0.328, 0.650, 0.040, 0.086, 0.550, 0.757, 0.580, 0.073, 0.891, 0.584, 0.356, 0.202, 0.091, 0.033;
0.163, 0.564, 0.130, 0.100, 0.247, 0.215, 0.578, 0.088, 0.591, 0.126, 0.325, 0.654, 0.041, 0.084, 0.598, 0.761, 0.628, 0.073, 0.891, 0.583, 0.355, 0.201, 0.090, 0.033;
0.167, 0.569, 0.126, 0.099, 0.254, 0.212, 0.583, 0.092, 0.597, 0.138, 0.322, 0.658, 0.041, 0.082, 0.631, 0.765, 0.666, 0.073, 0.891, 0.581, 0.354, 0.201, 0.090, 0.033;
0.173, 0.574, 0.123, 0.097, 0.269, 0.210, 0.587, 0.098, 0.601, 0.149, 0.320, 0.662, 0.042, 0.080, 0.654, 0.768, 0.696, 0.073, 0.890, 0.580, 0.353, 0.200, 0.090, 0.033;
0.180, 0.582, 0.120, 0.096, 0.291, 0.209, 0.593, 0.105, 0.604, 0.161, 0.319, 0.667, 0.042, 0.078, 0.672, 0.772, 0.722, 0.073, 0.889, 0.579, 0.352, 0.199, 0.090, 0.033;
0.188, 0.590, 0.118, 0.095, 0.318, 0.208, 0.599, 0.111, 0.607, 0.172, 0.319, 0.672, 0.042, 0.077, 0.686, 0.777, 0.742, 0.074, 0.889, 0.578, 0.351, 0.198, 0.089, 0.033;
0.196, 0.597, 0.115, 0.095, 0.351, 0.209, 0.602, 0.118, 0.608, 0.182, 0.320, 0.675, 0.043, 0.076, 0.694, 0.779, 0.756, 0.075, 0.889, 0.577, 0.350, 0.198, 0.089, 0.033;
0.204, 0.605, 0.112, 0.095, 0.384, 0.211, 0.604, 0.123, 0.607, 0.193, 0.324, 0.676, 0.043, 0.075, 0.700, 0.780, 0.766, 0.076, 0.889, 0.576, 0.349, 0.197, 0.089, 0.033;
0.213, 0.614, 0.110, 0.097, 0.417, 0.215, 0.606, 0.126, 0.606, 0.205, 0.330, 0.677, 0.043, 0.075, 0.704, 0.780, 0.774, 0.076, 0.889, 0.575, 0.348, 0.197, 0.088, 0.033;
0.222, 0.624, 0.108, 0.101, 0.446, 0.220, 0.608, 0.126, 0.605, 0.217, 0.337, 0.678, 0.044, 0.075, 0.707, 0.781, 0.780, 0.077, 0.888, 0.574, 0.346, 0.196, 0.088, 0.033;
0.231, 0.637, 0.106, 0.110, 0.470, 0.227, 0.611, 0.124, 0.605, 0.232, 0.345, 0.681, 0.044, 0.077, 0.712, 0.782, 0.785, 0.076, 0.888, 0.573, 0.346, 0.195, 0.088, 0.033;
0.242, 0.652, 0.105, 0.125, 0.490, 0.233, 0.615, 0.120, 0.605, 0.248, 0.354, 0.685, 0.044, 0.078, 0.718, 0.785, 0.791, 0.075, 0.888, 0.572, 0.345, 0.195, 0.087, 0.033;
0.251, 0.668, 0.104, 0.147, 0.504, 0.239, 0.619, 0.117, 0.604, 0.266, 0.362, 0.688, 0.044, 0.080, 0.721, 0.785, 0.794, 0.074, 0.888, 0.571, 0.344, 0.194, 0.087, 0.033;
0.261, 0.682, 0.104, 0.174, 0.511, 0.244, 0.622, 0.115, 0.605, 0.282, 0.368, 0.690, 0.045, 0.082, 0.724, 0.787, 0.798, 0.074, 0.888, 0.570, 0.343, 0.194, 0.087, 0.032;
0.271, 0.697, 0.103, 0.210, 0.517, 0.249, 0.625, 0.115, 0.606, 0.301, 0.375, 0.693, 0.046, 0.085, 0.727, 0.789, 0.801, 0.073, 0.888, 0.569, 0.342, 0.193, 0.087, 0.032;
0.282, 0.713, 0.103, 0.247, 0.520, 0.252, 0.628, 0.116, 0.606, 0.319, 0.379, 0.696, 0.048, 0.088, 0.729, 0.792, 0.804, 0.072, 0.888, 0.568, 0.341, 0.192, 0.086, 0.032;
0.294, 0.728, 0.102, 0.283, 0.522, 0.252, 0.630, 0.118, 0.604, 0.338, 0.381, 0.698, 0.050, 0.089, 0.730, 0.792, 0.806, 0.072, 0.887, 0.567, 0.340, 0.192, 0.086, 0.032;
0.305, 0.745, 0.102, 0.311, 0.523, 0.250, 0.633, 0.120, 0.602, 0.355, 0.379, 0.698, 0.051, 0.089, 0.730, 0.793, 0.807, 0.071, 0.886, 0.566, 0.339, 0.191, 0.086, 0.032;
0.318, 0.753, 0.102, 0.329, 0.522, 0.248, 0.633, 0.124, 0.601, 0.371, 0.376, 0.698, 0.053, 0.090, 0.729, 0.792, 0.807, 0.073, 0.886, 0.565, 0.338, 0.191, 0.086, 0.032;
0.334, 0.762, 0.102, 0.343, 0.521, 0.244, 0.633, 0.128, 0.599, 0.388, 0.373, 0.698, 0.056, 0.090, 0.727, 0.790, 0.807, 0.075, 0.886, 0.564, 0.337, 0.190, 0.085, 0.032;
0.354, 0.774, 0.102, 0.353, 0.521, 0.245, 0.636, 0.133, 0.598, 0.406, 0.372, 0.700, 0.060, 0.090, 0.728, 0.792, 0.810, 0.078, 0.885, 0.562, 0.336, 0.189, 0.085, 0.032;
0.372, 0.783, 0.102, 0.358, 0.522, 0.245, 0.637, 0.139, 0.596, 0.422, 0.375, 0.701, 0.064, 0.089, 0.729, 0.792, 0.813, 0.082, 0.885, 0.562, 0.335, 0.189, 0.085, 0.032;
0.392, 0.788, 0.104, 0.362, 0.521, 0.251, 0.639, 0.149, 0.595, 0.436, 0.382, 0.701, 0.070, 0.092, 0.729, 0.790, 0.814, 0.090, 0.885, 0.560, 0.334, 0.188, 0.085, 0.032;
0.409, 0.791, 0.104, 0.364, 0.521, 0.260, 0.638, 0.162, 0.593, 0.451, 0.392, 0.701, 0.079, 0.094, 0.727, 0.787, 0.813, 0.100, 0.884, 0.560, 0.333, 0.188, 0.085, 0.032;
0.420, 0.787, 0.104, 0.360, 0.516, 0.269, 0.633, 0.178, 0.587, 0.460, 0.401, 0.695, 0.091, 0.097, 0.723, 0.782, 0.810, 0.116, 0.884, 0.558, 0.332, 0.187, 0.084, 0.032;
0.436, 0.789, 0.104, 0.362, 0.514, 0.278, 0.633, 0.197, 0.584, 0.471, 0.412, 0.694, 0.104, 0.102, 0.721, 0.778, 0.808, 0.133, 0.883, 0.557, 0.331, 0.187, 0.084, 0.032;
0.450, 0.794, 0.106, 0.364, 0.514, 0.288, 0.636, 0.219, 0.584, 0.481, 0.422, 0.696, 0.120, 0.106, 0.724, 0.780, 0.811, 0.154, 0.882, 0.556, 0.330, 0.186, 0.084, 0.032;
0.462, 0.801, 0.106, 0.368, 0.517, 0.297, 0.641, 0.242, 0.586, 0.492, 0.433, 0.700, 0.138, 0.110, 0.728, 0.782, 0.814, 0.176, 0.882, 0.555, 0.329, 0.185, 0.084, 0.032;
0.465, 0.799, 0.107, 0.368, 0.515, 0.301, 0.639, 0.259, 0.584, 0.495, 0.436, 0.698, 0.154, 0.111, 0.727, 0.781, 0.813, 0.191, 0.881, 0.554, 0.328, 0.185, 0.084, 0.032;
0.448, 0.771, 0.110, 0.355, 0.500, 0.297, 0.616, 0.275, 0.566, 0.482, 0.426, 0.673, 0.168, 0.112, 0.702, 0.752, 0.785, 0.200, 0.880, 0.553, 0.327, 0.184, 0.083, 0.032;
0.432, 0.747, 0.115, 0.346, 0.491, 0.296, 0.598, 0.294, 0.551, 0.471, 0.413, 0.653, 0.186, 0.112, 0.680, 0.728, 0.765, 0.208, 0.880, 0.551, 0.326, 0.184, 0.083, 0.032;
0.421, 0.734, 0.120, 0.341, 0.487, 0.296, 0.582, 0.316, 0.540, 0.467, 0.404, 0.639, 0.204, 0.112, 0.664, 0.710, 0.752, 0.214, 0.879, 0.550, 0.325, 0.183, 0.083, 0.032;

};