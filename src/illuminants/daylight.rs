/*!

# CIE Standard Daylight Illuminants

When representing daylight in colorimetric calculations, CIE recommends using the D65 standard illuminant,
or, if another daylight at another temperature has to be used, the D50, D55, or D75 standard illuminants.

If these can not be used either, CIE provides data and an algorithm to calculate daylight spectral distributions
for any correlated color temperature in the range from 4000 to 25000K.
This generic daylight illuminant, called the CIE D standard illuminant, is used in various other colorimetric standards,
such as the CIE Color Rendering Index recommendation.

## CIE D50, D55, D65, and D75 Standard Illuminants

The D50, D55, D65, D75 illuminants – defined in the 1960s – represent daylight at correlated color temperatures at
5000K, 5500K, 6500K, and 7500K respectively, in the international Kelvin scale valid at that point of time.
They are defined by the CIE by spectral tables.

In 1968 the definition of the absolute temperature scale was slightly changed, affecting the definition of temperatures of these
daylight illuminants, as described by CIE\[2004\] in note 4:
> The correlated colour temperatures are affected by the numerical value of the radiation constant C2.
> In accordance with the <i>International Practical Temperature Scale, 1948, amended 1960</i> which was in use at the time when the procedure for
> calculating daylight illuminants was adopted by the CIE, the value of C2 was equal to 1,4380 x 10- m-K.
> With this value, the correlated colour temperature of illuminant d65 is approximately equal to 6500 K.
> The change of C2 to the value of 1,4388 x 10-2 m·K (International Practical Temperature Scale, 1968) increases the correlated colour
> temperatures of illuminant 065 by the factor 1,4388/1,4380.
> Thus the correlated colour temperature increases by approximately 4 K.

Here the CIE D50, D55, D65, and D75 illuminants are defined as spectral distribution tables, as opposed to using the generic
CIE D illumiant algorithm. To recreate the values as originally defined, the following method can be use (ref. CIE\[2004\], note 5):
> The method required to calculate the values for the relative spectral power distributions of illuminants D50, D55, D65,
> and D75, in Table T.1 is as follows
> 1. Multiply the nominal correlated colour temperature (5000 K, 5500 K, 6500 K or 7500 K) by 1,4388/1,4380.
> 2. Calculate x<sub>D</sub> and y<sub>D</sub> using the equations given in the text.
> 3. Calculate M<sub>1</sub> and M<sub>2</sub> using the equations given in the text.
> 4. Round M<sub>1</sub> and M<sub>2</sub> to three decimal places.
> 5. Calculate S(&lambda;) every 10 nm by S(&lambda;) = S<sub>0</sub>(&lambda;) + M<sub>1</sub> S<sub>1</sub>(&lambda;) + M<sub>2</sub>·S<sub>2</sub>(&lambda;)
> using values of S<sub>O</sub>(&lambda;), S<sub>1</sub>(&lambda;) and S<sub>2</sub>(&lambda;) from Table T.2.
> 6. Interpolate the 10 nm values of S(&lambda;) linearly to obtain values at intermediate wavelengths.

# CIE D Illuminant, for aribitrary CCTs in the range from 4000 to 25_000 Kelvin.

 */

use nalgebra::{Const, Dynamic, Matrix, Matrix3xX, SMatrixSlice, VecStorage};

use crate::illuminants::cct_parameters::CctParameters;
use crate::models::CieXYZ;
use crate::observers::StandardObserver;
use crate::Domain;
use crate::SpectralDistribution;
use crate::{WavelengthStep, NM5};

use super::Illuminant;

pub type D50 = CieIllD50;
pub type D55 = CieIllD55;
pub type D65 = CieIllD65;
pub type D75 = CieIllD75;
pub type CieD = CieDaylight;

#[derive(Debug, Clone)]
pub struct CieDaylight {
    pub ccts: CctParameters,
}

impl CieDaylight {
    pub fn new(parameters: impl Into<CctParameters>) -> CieDaylight {
        CieDaylight {
            ccts: parameters.into(),
        }
    }
}

impl Default for CieDaylight {
    fn default() -> Self {
        Self::new(6503.5)
    }
}

const N: usize = 107;

impl SpectralDistribution for CieDaylight {
    type MatrixType = Matrix<f64, Const<N>, Dynamic, VecStorage<f64, Const<N>, Dynamic>>;
    type StepType = WavelengthStep;

    fn shape(&self) -> (usize, usize) {
        (N, self.ccts.len())
    }

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        let mut mvec: Vec<f64> = Vec::with_capacity(3 * N);
        for t in &self.ccts {
            let cct = t.clamp(4000.0, 25000.0);
            let xd = match cct {
                t if t < 7000.0 => {
                    0.244063 + 0.09911E3 / t + 2.9678E6 / t.powi(2) - 4.607E9 / t.powi(3)
                }
                _ => 0.23704 + 0.24748E3 / t + 1.9018E6 / t.powi(2) - 2.0064E9 / t.powi(3),
            };
            let yd = -3. * xd.powi(2) + 2.87 * xd - 0.275;
            let m = 0.0241 + 0.2562 * xd - 0.7341 * yd;
            let m1 = (-1.3515 - 1.7703 * xd + 5.9114 * yd) / m;
            let m2 = (0.03 - 31.4424 * xd + 30.0717 * yd) / m;
            mvec.push(1.0);
            mvec.push(m1);
            mvec.push(m2);
        }
        let mmat = Matrix3xX::from_vec(mvec);
        (
            Domain::new(60, 166, NM5),
            SMatrixSlice::<f64, NS, MS>::from_slice(&S) * mmat,
        )
    }
}

impl Illuminant for CieDaylight {}

impl<C: StandardObserver> From<CieDaylight> for CieXYZ<C> {
    fn from(d: CieDaylight) -> Self {
        d.xyz().normalize(100.0)
    }
}

#[derive(Default)]
pub struct D<const T: usize>;

impl<const T: usize> Illuminant for D<T> {}

impl<const T: usize> SpectralDistribution for D<T> {
    type MatrixType = Matrix<f64, Const<N>, Dynamic, VecStorage<f64, Const<N>, Dynamic>>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        CieDaylight::new(T).spd()
    }

    fn shape(&self) -> (usize, usize) {
        (N, 1)
    }
}

impl<C: StandardObserver, const T: usize> From<D<T>> for CieXYZ<C> {
    fn from(d: D<T>) -> Self {
        d.xyz().normalize(100.0)
    }
}

/**
   Data below from CIE 15:2004 Excel tables.
*/
const NS: usize = 107;
const MS: usize = 3;
const S: [f64; NS * MS] = [
    // S0
    0.04, 3.02, 6.0, 17.8, 29.6, 42.45, 55.3, 56.3, 57.3, 59.55, 61.8, 61.65, 61.5, 65.15, 68.8,
    66.1, 63.4, 64.6, 65.8, 80.3, 94.8, 99.8, 104.8, 105.35, 105.9, 101.35, 96.8, 105.35, 113.9,
    119.75, 125.6, 125.55, 125.5, 123.4, 121.3, 121.3, 121.3, 117.4, 113.5, 113.3, 113.1, 111.95,
    110.8, 108.65, 106.5, 107.65, 108.8, 107.05, 105.3, 104.85, 104.4, 102.2, 100.0, 98.0, 96.0,
    95.55, 95.1, 92.1, 89.1, 89.8, 90.5, 90.4, 90.3, 89.35, 88.4, 86.2, 84.0, 84.55, 85.1, 83.5,
    81.9, 82.25, 82.6, 83.75, 84.9, 83.1, 81.3, 76.6, 71.9, 73.1, 74.3, 75.35, 76.4, 69.85, 63.3,
    67.5, 71.7, 74.35, 77.0, 71.1, 65.2, 56.45, 47.7, 58.15, 68.6, 66.8, 65.0, 65.5, 66.0, 63.5,
    61.0, 57.15, 53.3, 56.1, 58.9, 60.4, 61.9, // S1
    0.02, 2.26, 4.5, 13.45, 22.4, 32.2, 42.0, 41.3, 40.6, 41.1, 41.6, 39.8, 38.0, 40.2, 42.4,
    40.45, 38.5, 36.75, 35.0, 39.2, 43.4, 44.85, 46.3, 45.1, 43.9, 40.5, 37.1, 36.9, 36.7, 36.3,
    35.9, 34.25, 32.6, 30.25, 27.9, 26.1, 24.3, 22.2, 20.1, 18.15, 16.2, 14.7, 13.2, 10.9, 8.6,
    7.35, 6.1, 5.15, 4.2, 3.05, 1.9, 0.95, 0.0, -0.8, -1.6, -2.55, -3.5, -3.5, -3.5, -4.65, -5.8,
    -6.5, -7.2, -7.9, -8.6, -9.05, -9.5, -10.2, -10.9, -10.8, -10.7, -11.35, -12.0, -13.0, -14.0,
    -13.8, -13.6, -12.8, -12.0, -12.65, -13.3, -13.1, -12.9, -11.75, -10.6, -11.1, -11.6, -11.9,
    -12.2, -11.2, -10.2, -9.0, -7.8, -9.5, -11.2, -10.8, -10.4, -10.5, -10.6, -10.15, -9.7, -9.0,
    -8.3, -8.8, -9.3, -9.55, -9.8, // S2
    0.0, 1.0, 2.0, 3.0, 4.0, 6.25, 8.5, 8.15, 7.8, 7.25, 6.7, 6.0, 5.3, 5.7, 6.1, 4.55, 3.0, 2.1,
    1.2, 0.05, -1.1, -0.8, -0.5, -0.6, -0.7, -0.95, -1.2, -1.9, -2.6, -2.75, -2.9, -2.85, -2.8,
    -2.7, -2.6, -2.6, -2.6, -2.2, -1.8, -1.65, -1.5, -1.4, -1.3, -1.25, -1.2, -1.1, -1.0, -0.75,
    -0.5, -0.4, -0.3, -0.15, 0.0, 0.1, 0.2, 0.35, 0.5, 1.3, 2.1, 2.65, 3.2, 3.65, 4.1, 4.4, 4.7,
    4.9, 5.1, 5.9, 6.7, 7.0, 7.3, 7.95, 8.6, 9.2, 9.8, 10.0, 10.2, 9.25, 8.3, 8.95, 9.6, 9.05, 8.5,
    7.75, 7.0, 7.3, 7.6, 7.8, 8.0, 7.35, 6.7, 5.95, 5.2, 6.3, 7.4, 7.1, 6.8, 6.9, 7.0, 6.7, 6.4,
    5.95, 5.5, 5.8, 6.1, 6.3, 6.5,
];

const NDATA: usize = 97;

illuminant!(
    CieIllD50,
    NDATA,
    "CIE D50 Illuminant",
    Domain::new(300 / 5, 780 / 5, NM5),
    D50_DATA
);

static D50_DATA: [f64; NDATA] = [
    0.019, 1.035, 2.051, 4.914, 7.778, 11.263, 14.748, 16.348, 17.948, 19.479, 21.010, 22.476,
    23.942, 25.451, 26.961, 25.724, 24.488, 27.179, 29.871, 39.589, 49.308, 52.910, 56.513, 58.273,
    60.034, 58.926, 57.818, 66.321, 74.825, 81.036, 87.247, 88.930, 90.612, 90.990, 91.368, 93.238,
    95.109, 93.536, 91.963, 93.843, 95.724, 96.169, 96.613, 96.871, 97.129, 99.614, 102.099,
    101.427, 100.755, 101.536, 102.317, 101.159, 100.000, 98.868, 97.735, 98.327, 98.918, 96.208,
    93.499, 95.593, 97.688, 98.478, 99.269, 99.155, 99.042, 97.382, 95.722, 97.290, 98.857, 97.262,
    95.667, 96.929, 98.190, 100.597, 103.003, 101.068, 99.133, 93.257, 87.381, 89.492, 91.604,
    92.246, 92.889, 84.872, 76.854, 81.683, 86.511, 89.546, 92.580, 85.405, 78.230, 67.961, 57.692,
    70.307, 82.923, 80.599, 78.274,
];

illuminant_single_test!(test_d50, CieIllD65, 0.34567, 5E-5, 0.35851, 5E-5);

illuminant!(
    CieIllD55,
    NDATA,
    "CIE D55 Illuminant",
    Domain::new(300 / 5, 780 / 5, NM5),
    D55_DATA
);

static D55_DATA: [f64; NDATA] = [
    0.024, 1.048, 2.072, 6.648, 11.224, 15.936, 20.647, 22.266, 23.885, 25.851, 27.817, 29.219,
    30.621, 32.464, 34.308, 33.446, 32.584, 35.335, 38.087, 49.518, 60.949, 64.751, 68.554, 70.065,
    71.577, 69.746, 67.914, 76.760, 85.605, 91.799, 97.993, 99.228, 100.463, 100.188, 99.913,
    101.326, 102.739, 100.409, 98.078, 99.379, 100.680, 100.688, 100.695, 100.341, 99.987, 102.098,
    104.210, 103.156, 102.102, 102.535, 102.968, 101.484, 100.000, 98.608, 97.216, 97.482, 97.749,
    94.590, 91.432, 92.926, 94.419, 94.780, 95.140, 94.680, 94.220, 92.334, 90.448, 91.389, 92.330,
    90.592, 88.854, 89.586, 90.317, 92.133, 93.950, 91.953, 89.956, 84.817, 79.677, 81.258, 82.840,
    83.842, 84.844, 77.539, 70.235, 74.768, 79.301, 82.147, 84.993, 78.437, 71.880, 62.337, 52.793,
    64.360, 75.927, 73.872, 71.818,
];

illuminant_single_test!(test_d55, CieIllD55, 0.33243, 5E-5, 0.34744, 5E-5);

illuminant!(
    CieIllD65,
    NDATA,
    "CIE D65 Illuminant",
    Domain::new(300 / 5, 780 / 5, NM5),
    D65_DATA
);

static D65_DATA: [f64; NDATA] = [
    0.034100, 1.664300, 3.294500, 11.765200, 20.236000, 28.644700, 37.053500, 38.501100, 39.948800,
    42.430200, 44.911700, 45.775000, 46.638300, 49.363700, 52.089100, 51.032300, 49.975500,
    52.311800, 54.648200, 68.701500, 82.754900, 87.120400, 91.486000, 92.458900, 93.431800,
    90.057000, 86.682300, 95.773600, 104.865000, 110.936000, 117.008000, 117.410000, 117.812000,
    116.336000, 114.861000, 115.392000, 115.923000, 112.367000, 108.811000, 109.082000, 109.354000,
    108.578000, 107.802000, 106.296000, 104.790000, 106.239000, 107.689000, 106.047000, 104.405000,
    104.225000, 104.046000, 102.023000, 100.000000, 98.167100, 96.334200, 96.061100, 95.788000,
    92.236800, 88.685600, 89.345900, 90.006200, 89.802600, 89.599100, 88.648900, 87.698700,
    85.493600, 83.288600, 83.493900, 83.699200, 81.863000, 80.026800, 80.120700, 80.214600,
    81.246200, 82.277800, 80.281000, 78.284200, 74.002700, 69.721300, 70.665200, 71.609100,
    72.979000, 74.349000, 67.976500, 61.604000, 65.744800, 69.885600, 72.486300, 75.087000,
    69.339800, 63.592700, 55.005400, 46.418200, 56.611800, 66.805400, 65.094100, 63.382800,
];

#[test]
fn test_d65() {
    use crate::models::{self, CieYxy, YxyValues};
    use crate::observers::CieObs1931;
    use approx::assert_abs_diff_eq;

    let yxy: models::CieYxy = CieIllD65.into();
    assert_abs_diff_eq!(yxy.data.column(0).y, 0.31272, epsilon = 1E-6); // CIE 15:2004, Table T.3. D65 x value
    assert_abs_diff_eq!(yxy.data.column(0).z, 0.32903, epsilon = 1E-6); // CIE 15:2004, Table T.3. D65 y value

    let YxyValues { l: _, x, y } = CieYxy::<CieObs1931>::from(D::<6504>)
        .into_iter()
        .next()
        .unwrap();
    assert_abs_diff_eq!(x, 0.31272, epsilon = 2E-5); // CIE 15:2004, Table T.3. D65 x value
    assert_abs_diff_eq!(y, 0.32903, epsilon = 2E-5); // CIE 15:2004, Table T.3. D65 y value

    let YxyValues { l: _, x, y } = CieYxy::<CieObs1931>::from(CieDaylight::default())
        .into_iter()
        .next()
        .unwrap();
    println!("{} {}", x, y);
    assert_abs_diff_eq!(x, 0.31272, epsilon = 1E-5); // CIE 15:2004, Table T.3. D65 x value
    assert_abs_diff_eq!(y, 0.32903, epsilon = 1E-5); // CIE 15:2004, Table T.3. D65 y value
}

illuminant!(
    CieIllD75,
    NDATA,
    "CIE D75 Illuminant",
    Domain::new(300 / 5, 780 / 5, NM5),
    D75_DATA
);

static D75_DATA: [f64; NDATA] = [
    0.043, 2.588, 5.133, 17.470, 29.808, 42.369, 54.930, 56.095, 57.259, 60.000, 62.740, 62.861,
    62.982, 66.647, 70.312, 68.507, 66.703, 68.333, 69.963, 85.946, 101.929, 106.911, 111.894,
    112.346, 112.798, 107.945, 103.092, 112.145, 121.198, 127.104, 133.010, 132.682, 132.355,
    129.838, 127.322, 127.061, 126.800, 122.291, 117.783, 117.186, 116.589, 115.146, 113.702,
    111.181, 108.659, 109.552, 110.445, 108.367, 106.289, 105.596, 104.904, 102.452, 100.000,
    97.808, 95.616, 94.914, 94.213, 90.605, 86.997, 87.112, 87.227, 86.684, 86.140, 84.861, 83.581,
    81.164, 78.747, 78.587, 78.428, 76.614, 74.801, 74.562, 74.324, 74.873, 75.422, 73.499, 71.576,
    67.714, 63.852, 64.464, 65.076, 66.573, 68.070, 62.256, 56.443, 60.343, 64.242, 66.697, 69.151,
    63.890, 58.629, 50.623, 42.617, 51.985, 61.352, 59.838, 58.324,
];

illuminant_single_test!(test_d75, CieIllD75, 0.29903, 5E-5, 0.31488, 5E-5);

illuminant!(
    CieIllC,
    NDATA,
    "C Illuminant",
    Domain::new(300 / 5, 780 / 5, NM5),
    C_DATA
);

static C_DATA: [f64; NDATA] = [
    0.00, 0.00, 0.00, 0.00, 0.01, 0.20, 0.40, 1.55, 2.70, 4.85, 7.00, 9.95, 12.90, 17.20, 21.40,
    27.50, 33.00, 39.92, 47.40, 55.17, 63.30, 71.81, 80.60, 89.53, 98.10, 105.80, 112.40, 117.75,
    121.50, 123.45, 124.00, 123.60, 123.10, 123.30, 123.80, 124.09, 123.90, 122.92, 120.70, 116.90,
    112.10, 106.98, 102.30, 98.81, 96.90, 96.78, 98.00, 99.94, 102.10, 103.95, 105.20, 105.67,
    105.30, 104.11, 102.30, 100.15, 97.80, 95.43, 93.20, 91.22, 89.70, 88.83, 88.40, 88.19, 88.10,
    88.06, 88.00, 87.86, 87.80, 87.99, 88.20, 88.20, 87.90, 87.22, 86.30, 85.30, 84.00, 82.21,
    80.20, 78.24, 76.30, 74.36, 72.40, 70.40, 68.30, 66.30, 64.40, 62.80, 61.50, 60.20, 59.20,
    58.50, 58.10, 58.00, 58.20, 58.50, 59.10,
];

illuminant_single_test!(test_c_illuminant, CieIllC, 0.31006, 5E-5, 0.31616, 5E-5);

#[test]
fn c_xyx() {
    use crate::models::CieXYZ;
    let c_xyz: CieXYZ<crate::observers::CieObs1931> = CieIllC.into();
    println!("{}", c_xyz.data);
}
