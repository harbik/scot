/*!

Average spectral data, measured by Danny Pascale of BabelColor, for 30 samples of standard and mini Color Checker 
rendition cards, as produced by X-rite.

The spectral domain ranges from 380 to 780 nm, and spectral reflectance was measured in a 
Measurement geometry of 45 deg./0 deg. 
Of the 30 data sets, 24 were measured with Eye-One Pro spectrocolorimeters from X-Rite.

BabelColor is a Registered Trademark of The BabelColor Company.
ColorChecker and X-Rite are Trademarks of X-Rite Incorporated.
*/
use nalgebra::{ArrayStorage, SMatrix, SVectorSlice};

use crate::ALL;
use crate::spectra::SpectralData;
use crate::util::domain::Domain;
use crate::util::units::{WavelengthScale, Scale, NM10};
use crate::util::interpolate::sprague_cols;

use super::Swatches;


const N: usize = 36;
const M: usize = 24;

#[derive(Default)]
pub struct ColorChecker<const I:usize>;

impl<const I: usize> SpectralData for ColorChecker<I> {
    type ScaleType = WavelengthScale;

    fn values<L>(&self, domain: &Domain<L>) -> nalgebra::DMatrix<f64>
	where
		L: Scale,
		<Self::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
	{
		match I {
			ALL => {
				let data = SMatrix::<f64, N, M>::from_data(ArrayStorage::<f64, N, M>(COLORCHECKER));
				sprague_cols(&self.domain(), &domain, &data)
			}
			i@1..=M => {
				let data = SVectorSlice::<f64, N>::from_slice(&COLORCHECKER[i-1]);
				sprague_cols(&self.domain(), &domain, &data)
			}
			_ => panic!("Illegal Index in Fluorescent Illuminant")
		}
    }

    fn domain(&self) -> crate::util::domain::Domain<Self::ScaleType> {
        Domain::new(38, 73, NM10)
    }

	fn keys(&self) -> Option<Vec<String>> {
		Some(vec![
			"dark skin".to_string(),
			"light skin".to_string(),
			"blue sky".to_string(),
			"foliage".to_string(),
			"blue flower".to_string(),
			"bluish green".to_string(),
			"orange".to_string(),
			"purplish blue".to_string(),
			"moderate red".to_string(),
			"purple".to_string(),
			"yellow green".to_string(),
			"orange yellow".to_string(),
			"blue".to_string(),
			"green".to_string(),
			"red".to_string(),
			"yellow".to_string(),
			"magenta".to_string(),
			"cyan".to_string(),
			"white 9.5 (.05 D)".to_string(),
			"neutral 8 (.23 D)".to_string(),
			"neutral 6.5 (.44 D)".to_string(),
			"neutral 5 (.70 D)".to_string(),
			"neutral 3.5 (1.05 D)".to_string(),
			"black 2 (1.5 D)".to_string(),
		])
	}

	fn description(&self) -> Option<String> {
		Some("Approximate Color Checker Spectra".to_string())
	}
}


impl<const I:usize> Swatches for ColorChecker<I> {}

static COLORCHECKER: [[f64;N];M] = [
[0.05475, 0.05833, 0.06116, 0.06238, 0.06231, 0.06207, 0.06183, 0.06159, 0.06154, 0.06162, 0.06203, 0.06296, 0.06518, 0.07027, 0.07640, 0.07949, 0.08128, 0.08429, 0.09058, 0.10290, 0.11905, 0.13426, 0.14320, 0.14688, 0.15078, 0.15810, 0.16819, 0.17890, 0.18755, 0.18964, 0.18577, 0.18149, 0.18161, 0.18721, 0.19605, 0.20949],
[0.11713, 0.14345, 0.17453, 0.19093, 0.19560, 0.19900, 0.20423, 0.21318, 0.22842, 0.25127, 0.28005, 0.30878, 0.32945, 0.33336, 0.31460, 0.28628, 0.27349, 0.27646, 0.27720, 0.28930, 0.33938, 0.42022, 0.48779, 0.52511, 0.54574, 0.56156, 0.57788, 0.59497, 0.61180, 0.62475, 0.63810, 0.65596, 0.67822, 0.69958, 0.71709, 0.73382],
[0.13036, 0.17707, 0.25101, 0.30625, 0.32392, 0.32993, 0.33283, 0.33097, 0.32342, 0.31134, 0.29823, 0.28533, 0.26943, 0.25037, 0.23144, 0.21426, 0.19942, 0.18451, 0.16938, 0.15729, 0.14911, 0.14482, 0.14186, 0.14057, 0.14067, 0.14109, 0.14257, 0.14654, 0.15184, 0.15351, 0.15009, 0.14395, 0.13639, 0.13235, 0.13496, 0.14673],
[0.05124, 0.05423, 0.05599, 0.05704, 0.05786, 0.05895, 0.06030, 0.06131, 0.06228, 0.06325, 0.06478, 0.06738, 0.07531, 0.10120, 0.14536, 0.17826, 0.18394, 0.17011, 0.14938, 0.13274, 0.12186, 0.11517, 0.10948, 0.10536, 0.10434, 0.10599, 0.10891, 0.11189, 0.11406, 0.11395, 0.11240, 0.11215, 0.11482, 0.11977, 0.12459, 0.13030],
[0.14423, 0.19827, 0.29443, 0.37544, 0.40837, 0.42095, 0.42618, 0.42609, 0.41932, 0.40343, 0.37927, 0.34636, 0.31112, 0.28124, 0.25388, 0.22889, 0.21420, 0.20835, 0.20162, 0.19440, 0.19257, 0.20018, 0.21441, 0.22952, 0.24058, 0.25396, 0.27851, 0.31322, 0.34779, 0.36587, 0.36579, 0.35942, 0.35799, 0.36493, 0.37723, 0.39783],
[0.13627, 0.17946, 0.24689, 0.29682, 0.32028, 0.33708, 0.35550, 0.38119, 0.41913, 0.46596, 0.51048, 0.54581, 0.56719, 0.57426, 0.56908, 0.55068, 0.52351, 0.48843, 0.44521, 0.39987, 0.35043, 0.29939, 0.25243, 0.22096, 0.20431, 0.19579, 0.19088, 0.18823, 0.19072, 0.19942, 0.21159, 0.22310, 0.23164, 0.23332, 0.22941, 0.22935],
[0.05381, 0.05369, 0.05326, 0.05370, 0.05402, 0.05452, 0.05495, 0.05516, 0.05568, 0.05664, 0.05840, 0.06122, 0.06822, 0.08942, 0.12461, 0.15350, 0.17379, 0.19944, 0.24827, 0.33542, 0.44399, 0.53847, 0.58667, 0.59484, 0.59059, 0.58662, 0.58417, 0.58386, 0.58975, 0.60251, 0.62039, 0.63880, 0.65481, 0.66255, 0.66255, 0.66681],
[0.12236, 0.16448, 0.22850, 0.28608, 0.32730, 0.36108, 0.38757, 0.39963, 0.39157, 0.36243, 0.31612, 0.26024, 0.20858, 0.16831, 0.13768, 0.11656, 0.10425, 0.09637, 0.08980, 0.08551, 0.08372, 0.08396, 0.08432, 0.08411, 0.08386, 0.08517, 0.08977, 0.09785, 0.10912, 0.12346, 0.14269, 0.16930, 0.20465, 0.24395, 0.28719, 0.33249],
[0.09600, 0.11466, 0.13058, 0.13508, 0.13345, 0.13159, 0.13021, 0.12811, 0.12505, 0.12048, 0.11512, 0.10985, 0.10494, 0.09982, 0.09516, 0.09265, 0.09247, 0.09319, 0.09621, 0.10812, 0.15557, 0.26539, 0.39871, 0.50008, 0.55632, 0.57945, 0.58773, 0.59063, 0.59251, 0.59445, 0.59785, 0.60219, 0.60690, 0.60925, 0.60896, 0.61024],
[0.09199, 0.11601, 0.14561, 0.16853, 0.17847, 0.17301, 0.15797, 0.13878, 0.11913, 0.10140, 0.08695, 0.07518, 0.06609, 0.06032, 0.05646, 0.05312, 0.05121, 0.05124, 0.05195, 0.05187, 0.05120, 0.05242, 0.05841, 0.07318, 0.09552, 0.11893, 0.14139, 0.16554, 0.19405, 0.22706, 0.26539, 0.30892, 0.35455, 0.39577, 0.43584, 0.47847],
[0.06103, 0.06125, 0.06192, 0.06291, 0.06397, 0.06593, 0.06921, 0.07473, 0.08549, 0.10506, 0.13867, 0.19209, 0.27073, 0.37611, 0.47578, 0.53122, 0.54916, 0.54571, 0.52807, 0.50446, 0.47052, 0.42764, 0.38125, 0.34680, 0.32744, 0.31771, 0.31247, 0.30994, 0.31441, 0.32741, 0.34523, 0.36255, 0.37622, 0.38054, 0.37767, 0.37941],
[0.06282, 0.06284, 0.06334, 0.06354, 0.06371, 0.06442, 0.06536, 0.06599, 0.06694, 0.06841, 0.07128, 0.07571, 0.08722, 0.12531, 0.20583, 0.30526, 0.38315, 0.43094, 0.46915, 0.51789, 0.56793, 0.60688, 0.62805, 0.63703, 0.63999, 0.64198, 0.64545, 0.64824, 0.65102, 0.65307, 0.65736, 0.66403, 0.67265, 0.67970, 0.68376, 0.68829],
[0.06624, 0.07864, 0.10159, 0.14554, 0.19951, 0.24440, 0.28250, 0.30936, 0.30759, 0.27781, 0.23087, 0.17754, 0.12971, 0.09428, 0.06954, 0.05399, 0.04582, 0.04167, 0.03944, 0.03831, 0.03775, 0.03775, 0.03797, 0.03852, 0.03925, 0.03987, 0.04080, 0.04232, 0.04418, 0.04547, 0.04584, 0.04642, 0.04837, 0.05217, 0.05731, 0.06498],
[0.05195, 0.05306, 0.05420, 0.05545, 0.05677, 0.05856, 0.06137, 0.06576, 0.07483, 0.09269, 0.12488, 0.17789, 0.24579, 0.30725, 0.33716, 0.33354, 0.31653, 0.29299, 0.26186, 0.22999, 0.19765, 0.16504, 0.13501, 0.11490, 0.10397, 0.09791, 0.09439, 0.09235, 0.09277, 0.09653, 0.10240, 0.10842, 0.11345, 0.11533, 0.11392, 0.11427],
[0.04992, 0.04877, 0.04759, 0.04724, 0.04716, 0.04735, 0.04742, 0.04692, 0.04607, 0.04518, 0.04444, 0.04429, 0.04468, 0.04560, 0.04678, 0.04764, 0.04859, 0.05037, 0.05385, 0.05986, 0.07212, 0.10356, 0.17752, 0.31207, 0.46683, 0.58083, 0.64443, 0.67484, 0.69018, 0.69824, 0.70592, 0.71495, 0.72370, 0.73010, 0.73371, 0.73841],
[0.05798, 0.05442, 0.05216, 0.05198, 0.05263, 0.05398, 0.05608, 0.05942, 0.06659, 0.08068, 0.10688, 0.15204, 0.22507, 0.33553, 0.46239, 0.55873, 0.61573, 0.64973, 0.67222, 0.69387, 0.70995, 0.72319, 0.73144, 0.73904, 0.74620, 0.75180, 0.75816, 0.76394, 0.76869, 0.77098, 0.77551, 0.78240, 0.79018, 0.79619, 0.79930, 0.80366],
[0.14455, 0.19511, 0.28259, 0.34577, 0.36182, 0.35432, 0.33361, 0.30571, 0.27623, 0.24756, 0.21805, 0.18988, 0.16786, 0.14896, 0.12697, 0.10723, 0.09962, 0.10189, 0.10356, 0.10907, 0.13680, 0.19963, 0.29013, 0.40006, 0.51580, 0.61486, 0.68655, 0.73177, 0.75975, 0.77433, 0.78314, 0.79256, 0.80337, 0.81155, 0.81718, 0.82541],
[0.10773, 0.14119, 0.19247, 0.23641, 0.26085, 0.28550, 0.31740, 0.35313, 0.39024, 0.42597, 0.44561, 0.44423, 0.42321, 0.38549, 0.33672, 0.28273, 0.23128, 0.18506, 0.14554, 0.11807, 0.10053, 0.08958, 0.08156, 0.07640, 0.07406, 0.07305, 0.07294, 0.07381, 0.07559, 0.07675, 0.07648, 0.07499, 0.07277, 0.07200, 0.07374, 0.07935],
[0.18936, 0.25464, 0.42260, 0.66021, 0.81098, 0.86212, 0.87658, 0.88417, 0.89104, 0.89566, 0.89932, 0.90370, 0.90718, 0.90908, 0.91091, 0.91005, 0.91122, 0.91402, 0.91343, 0.91602, 0.91548, 0.91584, 0.91433, 0.91547, 0.91764, 0.91863, 0.92101, 0.92291, 0.92386, 0.92199, 0.92242, 0.92477, 0.92749, 0.92977, 0.93041, 0.93329],
[0.17085, 0.23206, 0.36507, 0.50656, 0.56749, 0.58270, 0.58770, 0.59009, 0.59099, 0.58977, 0.58841, 0.58843, 0.58898, 0.58948, 0.59059, 0.59002, 0.58990, 0.59030, 0.58929, 0.59094, 0.59031, 0.58971, 0.58713, 0.58515, 0.58304, 0.57996, 0.57779, 0.57595, 0.57440, 0.57221, 0.57061, 0.56922, 0.56828, 0.56797, 0.56648, 0.56631],
[0.14421, 0.19246, 0.27184, 0.33081, 0.35042, 0.35692, 0.36123, 0.36326, 0.36297, 0.36081, 0.35874, 0.35811, 0.35846, 0.35919, 0.36041, 0.36046, 0.36056, 0.36083, 0.36043, 0.36185, 0.36175, 0.36133, 0.35932, 0.35753, 0.35543, 0.35241, 0.34990, 0.34764, 0.34547, 0.34272, 0.34017, 0.33760, 0.33531, 0.33383, 0.33180, 0.33054],
[0.10519, 0.13133, 0.16260, 0.18017, 0.18592, 0.18953, 0.19286, 0.19423, 0.19378, 0.19233, 0.19106, 0.19085, 0.19125, 0.19158, 0.19205, 0.19210, 0.19217, 0.19231, 0.19206, 0.19263, 0.19238, 0.19193, 0.19059, 0.18939, 0.18799, 0.18587, 0.18398, 0.18232, 0.18085, 0.17925, 0.17777, 0.17604, 0.17434, 0.17337, 0.17219, 0.17139],
[0.06796, 0.07672, 0.08388, 0.08741, 0.08888, 0.09044, 0.09187, 0.09204, 0.09135, 0.09039, 0.08975, 0.08965, 0.08981, 0.08988, 0.08997, 0.08996, 0.09001, 0.09006, 0.08986, 0.08990, 0.08951, 0.08916, 0.08852, 0.08806, 0.08749, 0.08645, 0.08551, 0.08478, 0.08420, 0.08368, 0.08321, 0.08254, 0.08176, 0.08143, 0.08093, 0.08068],
[0.03102, 0.03199, 0.03228, 0.03256, 0.03275, 0.03282, 0.03282, 0.03262, 0.03248, 0.03240, 0.03233, 0.03231, 0.03228, 0.03215, 0.03209, 0.03198, 0.03196, 0.03198, 0.03192, 0.03195, 0.03182, 0.03179, 0.03173, 0.03185, 0.03193, 0.03190, 0.03191, 0.03194, 0.03202, 0.03212, 0.03222, 0.03223, 0.03224, 0.03233, 0.03237, 0.03250]
];

// for test see lab.rs, where the colorchecker's cielab coordinates are calculated, and checked against BabelColor's data.
