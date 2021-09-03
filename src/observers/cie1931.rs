use nalgebra::OMatrix;
use nalgebra::{Const, Dynamic, Matrix3xX, MatrixSlice, MatrixSlice3xX};
use crate::interp_lin_cmf;
use crate::interp_lin_cmf2;
use crate::{observers::StandardObserver};
use crate::util::{Domain};
use crate::util::{NM5, NM, WavelengthStep, Meter, Step};



#[derive(Debug,Clone,Default)]
pub struct CieObs1931Classic {}

impl StandardObserver for CieObs1931Classic {
	//const K: f64 = 683.0;
	const NAME: &'static str = "CIE1931 2º";

	fn domain() -> Domain<WavelengthStep> {
		Domain::new( 360/5, 830/5,  NM5)
	}

	fn cmf<'a>() -> MatrixSlice3xX<'a, f64> {
		MatrixSlice::from_slice_generic(&CIE1931NM5SLICE, Const::<3>, Dynamic::new(N5) )
	}

	fn values<L>(target: &Domain<L>) -> Matrix3xX<f64>
	where
		L: Step,
		Meter: From<<L>::UnitValueType>
	 {
//		calculate row interpolated values, and convert to Matrix3xX matrix... 
//		let data = SMatrix::from_data(ArrayStorage(CIE1931NM5));
		//	interp_lin_cmf(&Self::domain(), &target, 3, Self::cmf())
		interp_lin_cmf2(&Self::domain(), &target,Self::cmf())
//		convert(sprague_rows(&self.domain(), &target, &Self::cmf()))
	}

}

/**
CIE 1931 2º color matching functions, defined from 380 to 780nm, in steps of 1nm.

Source: ANSI/IES TM-30-18 Advanced Calculation Tool V2.0, Aug 10, 2018.

*/
#[derive( Debug,Clone,Default, PartialEq)]
pub struct CieObs1931 {}

impl StandardObserver for CieObs1931 {
	const K: f64 = 683.0;
	const NAME: &'static str = "CIE1931 2º (TM30)";

	fn domain() -> Domain<WavelengthStep> {
		Domain::new( 380, 780,  NM)
	}

	fn cmf<'a>() -> MatrixSlice3xX<'a, f64> {
		MatrixSlice::from_slice_generic(&CIE1931NM1, Const::<3>, Dynamic::new(N1) )
    }

	//fn values<L>(target: &Domain<L>) -> Matrix3xX<f64>
	fn values<L>(target: &Domain<L>) -> OMatrix<f64, Const::<3>, Dynamic>
	where
		L: Step,
		Meter: From<<L>::UnitValueType>
	 {
		//matrix_from_data_by_lin_row_int(&self.domain(), &target, &CIE1931NM1)
		//let m =  interp_lin_cmf(&Self::domain(), &target, 3, Self::cmf());
		let m =  interp_lin_cmf2(&Self::domain(), &target,Self::cmf());
		m
	}

}

const N5: usize = 95;


pub(super) static CIE1931NM5SLICE: [f64;3*N5] = [ // 360-830-5
	0.0001299, 0.000003917, 0.0006061, 0.0002321, 0.000006965, 0.001086, 0.0004149, 0.00001239, 0.001946, 0.0007416, 0.00002202, 0.003486, 0.001368, 0.000039,
	0.006450001, 0.002236, 0.000064, 0.01054999, 0.004243, 0.00012, 0.02005001, 0.00765, 0.000217, 0.03621, 0.01431, 0.000396, 0.06785001, 0.02319, 0.00064,
	0.1102, 0.04351, 0.00121, 0.2074, 0.07763, 0.00218, 0.3713, 0.13438, 0.004, 0.6456, 0.21477, 0.0073, 1.0390501, 0.2839, 0.0116, 1.3856, 0.3285, 0.01684,
	1.62296, 0.34828, 0.023, 1.74706, 0.34806, 0.0298, 1.7826, 0.3362, 0.038, 1.77211, 0.3187, 0.048, 1.7441, 0.2908, 0.06, 1.6692, 0.2511, 0.0739, 1.5281,
	0.19536, 0.09098, 1.28764, 0.1421, 0.1126, 1.0419, 0.09564, 0.13902, 0.8129501, 0.05795001, 0.1693, 0.6162, 0.03201, 0.20802, 0.46518, 0.0147, 0.2586,
	0.3533, 0.0049, 0.323, 0.272, 0.0024, 0.4073, 0.2123, 0.0093, 0.503, 0.1582, 0.0291, 0.6082, 0.1117, 0.06327, 0.71, 0.07824999, 0.1096, 0.7932, 0.05725001,
	0.1655, 0.862, 0.04216, 0.2257499, 0.9148501, 0.02984, 0.2904, 0.954, 0.0203, 0.3597, 0.9803, 0.0134, 0.4334499, 0.9949501, 0.008749999, 0.5120501, 1.0,
	0.005749999, 0.5945, 0.995, 0.0039, 0.6784, 0.9786, 0.002749999, 0.7621, 0.952, 0.0021, 0.8425, 0.9154, 0.0018, 0.9163, 0.87, 0.001650001, 0.9786, 0.8163,
	0.0014, 1.0263, 0.757, 0.0011, 1.0567, 0.6949, 0.001, 1.0622, 0.631, 0.0008, 1.0456, 0.5668, 0.0006, 1.0026, 0.503, 0.00034, 0.9384, 0.4412, 0.00024,
	0.8544499, 0.381, 0.00019, 0.7514, 0.321, 0.0001, 0.6424, 0.265, 5E-05, 0.5419, 0.217, 0.00003, 0.4479, 0.175, 0.00002, 0.3608, 0.1382, 0.00001, 0.2835,
	0.107, 0.0, 0.2187, 0.0816, 0.0, 0.1649, 0.061, 0.0, 0.1212, 0.04458, 0.0, 0.0874, 0.032, 0.0, 0.0636, 0.0232, 0.0, 0.04677, 0.017, 0.0, 0.0329, 0.01192,
	0.0, 0.0227, 0.00821, 0.0, 0.01584, 0.005723, 0.0, 0.01135916, 0.004102, 0.0, 0.008110916, 0.002929, 0.0, 0.005790346, 0.002091, 0.0, 0.004109457, 0.001484,
	0.0, 0.002899327, 0.001047, 0.0, 0.00204919, 0.00074, 0.0, 0.001439971, 0.00052, 0.0, 0.000999949, 0.0003611, 0.0, 0.000690079, 0.0002492, 0.0, 0.000476021,
	0.0001719, 0.0, 0.000332301, 0.00012, 0.0, 0.000234826, 0.0000848, 0.0, 0.000166151, 0.00006, 0.0, 0.000117413, 0.0000424, 0.0, 8.30753E-05, 0.00003, 0.0,
	5.87065E-05, 0.0000212, 0.0, 4.15099E-05, 0.00001499, 0.0, 2.93533E-05, 0.0000106, 0.0, 2.06738E-05, 7.4657E-06, 0.0, 1.45598E-05, 5.2578E-06, 0.0,
	1.0254E-05, 3.7029E-06, 0.0, 7.22146E-06, 2.6078E-06, 0.0, 5.08587E-06, 1.8366E-06, 0.0, 3.58165E-06, 1.2934E-06, 0.0, 2.52253E-06, 9.1093E-07, 0.0,
	1.77651E-06, 6.4153E-07, 0.0, 1.25114E-06, 4.5181E-07, 0.0];


pub(super) const N1: usize = 401;

pub(super) static CIE1931NM1: [f64;3*N1] = [ //380-780-1
0.001368, 0.000039, 0.006450001, 0.001493555, 4.2623E-05, 0.00704267, 0.001635954, 4.67507E-05, 0.007714892, 0.001802374, 5.15669E-05, 0.008500777,
0.001999997, 5.72554E-05, 0.009434439, 0.002236, 0.000064, 0.01054999, 0.002537928, 0.0000724, 0.011977673, 0.002887504, 0.00008212, 0.013631438,
0.003286816, 0.00009324, 0.015521362, 0.003737952, 0.00010584, 0.017657526, 0.004243, 0.00012, 0.02005001, 0.004753104, 0.000134808, 0.022466969,
0.005334032, 0.000151584, 0.025220887, 0.006000608, 0.000170656, 0.028382324, 0.006767656, 0.000192352, 0.032021842, 0.00765, 0.000217, 0.03621,
0.008754816, 0.000246784, 0.041452242, 0.009981488, 0.000279712, 0.047275524, 0.011321752, 0.000315648, 0.053641686, 0.012767344, 0.000354456, 0.060512568,
0.01431, 0.000396, 0.06785001, 0.01561336, 0.000431248, 0.074050729, 0.01707928, 0.000471184, 0.081032967, 0.01878152, 0.000517896, 0.089149844,
0.02079384, 0.000573472, 0.098754482, 0.02319, 0.00064, 0.1102, 0.02626328, 0.000725552, 0.1248728, 0.02981304, 0.000824736, 0.141834399,
0.03385816, 0.000938144, 0.161179599, 0.03841752, 0.001066368, 0.1830032, 0.04351, 0.00121, 0.2074, 0.04894744, 0.0013576, 0.2334456,
0.05500752, 0.0015248, 0.2625088, 0.06176088, 0.0017152, 0.2949392, 0.06927816, 0.0019324, 0.3310864, 0.07763, 0.00218, 0.3713,
0.08713728, 0.00245584, 0.417047997, 0.09755784, 0.00277072, 0.467281994, 0.10889976, 0.00312968, 0.522071994, 0.12117112, 0.00353776, 0.581487995,
0.13438, 0.004, 0.6456, 0.1496836, 0.00455696, 0.720071622, 0.1656536, 0.00516928, 0.797980845, 0.1820108, 0.00583312, 0.877999267,
0.198476, 0.00654464, 0.958798486, 0.21477, 0.0073, 1.0390501, 0.22992144, 0.00808192, 1.114105366, 0.24451632, 0.00890336, 1.186786307,
0.25844848, 0.00976384, 1.256594605, 0.27161176, 0.01066288, 1.323031942, 0.2839, 0.0116, 1.3856, 0.29479168, 0.01257344, 1.441937435,
0.30469984, 0.01358432, 1.493874714, 0.31362216, 0.01463248, 1.541379274, 0.32155632, 0.01571776, 1.584418557, 0.3285, 0.01684, 1.62296,
0.33428736, 0.01800736, 1.6560504, 0.33912048, 0.01920928, 1.684808, 0.34303792, 0.02044352, 1.7094304, 0.34607824, 0.02170784, 1.7301152,
0.34828, 0.023, 1.74706, 0.34956848, 0.02428448, 1.75989184, 0.35012384, 0.02560064, 1.76952152, 0.35001296, 0.02695456, 1.77628928,
0.34930272, 0.02835232, 1.78053536, 0.34806, 0.0298, 1.7826, 0.3464272, 0.0313152, 1.78327208, 0.3443768, 0.0328896, 1.78233104,
0.3419568, 0.0345264, 1.78000496, 0.3392152, 0.0362288, 1.77652192, 0.3362, 0.038, 1.77211, 0.33330352, 0.0398496, 1.76884944,
0.33014336, 0.0417728, 1.76465312, 0.32668144, 0.0437712, 1.75928608, 0.32287968, 0.0458464, 1.75251336, 0.3187, 0.048, 1.7441,
0.3139968, 0.0502432, 1.73348912, 0.3088664, 0.0525656, 1.72084816, 0.3032976, 0.0549664, 1.70602264, 0.2972792, 0.0574448, 1.68885808,
0.2908, 0.06, 1.6692, 0.28393968, 0.06258704, 1.64733712, 0.27657344, 0.06526032, 1.62256096, 0.26866736, 0.06803008, 1.59460624,
0.26018752, 0.07090656, 1.56320768, 0.2511, 0.0739, 1.5281, 0.24064256, 0.07701808, 1.48494624, 0.22969168, 0.08027424, 1.43857072,
0.21839552, 0.08367936, 1.38972608, 0.20690224, 0.08724432, 1.33916496, 0.19536, 0.09098, 1.28764, 0.18437136, 0.09493248, 1.238208157,
0.17351648, 0.09906864, 1.188741674, 0.16282992, 0.10339056, 1.139417114, 0.15234624, 0.10790032, 1.090411035, 0.1421, 0.1126, 1.0419,
0.13220096, 0.11753008, 0.994273702, 0.122589679, 0.12264464, 0.947442285, 0.113281919, 0.12793616, 0.901529027, 0.10429344, 0.13339712, 0.856657206,
0.09564, 0.13902, 0.8129501, 0.087305042, 0.14462064, 0.770591126, 0.079344724, 0.15041232, 0.729628387, 0.071782887, 0.15643168, 0.690170125,
0.064643369, 0.16271536, 0.652324582, 0.05795001, 0.1693, 0.6162, 0.051921849, 0.17625936, 0.582548475, 0.046338727, 0.18358368, 0.550673434,
0.041175684, 0.19130032, 0.520522154, 0.036407762, 0.19943664, 0.492041917, 0.03201, 0.20802, 0.46518, 0.02789344, 0.21712448, 0.43994672,
0.024113119, 0.22671904, 0.41621056, 0.020660079, 0.23681936, 0.39390304, 0.01752536, 0.24744112, 0.37295568, 0.0147, 0.2586, 0.3533,
0.01214592, 0.27017984, 0.33488096, 0.00989056, 0.28236112, 0.31761328, 0.00793224, 0.29519248, 0.30142512, 0.00626928, 0.30872256, 0.28624464,
0.0049, 0.323, 0.272, 0.0037488, 0.33854, 0.258844, 0.0029064, 0.354808, 0.246424, 0.0023896, 0.371736, 0.234612,
0.0022152, 0.389256, 0.22328, 0.0024, 0.4073, 0.2123, 0.002916, 0.4255888, 0.200968, 0.003836, 0.4443184, 0.189876,
0.005188, 0.4634736, 0.17904, 0.007, 0.4830392, 0.168476, 0.0093, 0.503, 0.1582, 0.01218096, 0.5236928, 0.1481176,
0.01558968, 0.5446624, 0.138382801, 0.01953792, 0.5658056, 0.129039201, 0.02403744, 0.5870192, 0.1201304, 0.0291, 0.6082, 0.1117,
0.03485512, 0.6293184, 0.103985198, 0.04116736, 0.6501792, 0.096787595, 0.04801904, 0.6706608, 0.090102393, 0.05539248, 0.6906416, 0.083924791,
0.06327, 0.71, 0.07824999, 0.07164608, 0.7279936, 0.073263274, 0.08048784, 0.7452768, 0.068722238, 0.08977456, 0.7618832, 0.064574562,
0.09948552, 0.7778464, 0.060767926, 0.1096, 0.7932, 0.05725001, 0.120181443, 0.808161597, 0.053859689, 0.131103926, 0.822534794, 0.050680647,
0.142325686, 0.836307194, 0.047687765, 0.153804965, 0.849466395, 0.044855922, 0.1655, 0.862, 0.04216, 0.177200378, 0.873774022, 0.03947408,
0.189075155, 0.884928045, 0.036899039, 0.201124733, 0.895480067, 0.034434959, 0.213349514, 0.905448086, 0.03208192, 0.2257499, 0.9148501, 0.02984,
0.238319914, 0.923748886, 0.02771408, 0.251067933, 0.932106467, 0.02569824, 0.263995955, 0.939929645, 0.02379136, 0.277105978, 0.947225222, 0.02199232,
0.2904, 0.954, 0.0203, 0.303894408, 0.960249592, 0.01872128, 0.317573212, 0.965994788, 0.01724504, 0.331434812, 0.971245188, 0.01586816,
0.345477608, 0.976010392, 0.01458752, 0.3597, 0.9803, 0.0134, 0.374081175, 0.984096422, 0.0123092, 0.38864355, 0.987443245, 0.0113036,
0.403390326, 0.990356867, 0.010378399, 0.418324709, 0.992853686, 0.009528799, 0.4334499, 0.9949501, 0.008749999, 0.448813935, 0.996742486, 0.008033999,
0.464363978, 0.998147267, 0.007379999, 0.480092022, 0.999160845, 0.006783999, 0.495990065, 0.999779622, 0.006241999, 0.5120501, 1.0, 0.005749999,
0.528308891, 0.999847195, 0.005302399, 0.544702474, 0.999281594, 0.004897199, 0.56121165, 0.998292394, 0.0045308, 0.577817225, 0.996868797, 0.0041996,
0.5945, 0.995, 0.0039, 0.611216795, 0.9925936, 0.0036204, 0.627978394, 0.9897408, 0.0033672, 0.644771594, 0.9864512, 0.003138799,
0.661583197, 0.9827344, 0.002933599, 0.6784, 0.9786, 0.002749999, 0.6952552, 0.9740896, 0.002584799, 0.7120776, 0.9691728, 0.002438399,
0.7288424, 0.9638512, 0.0023096, 0.7455248, 0.9581264, 0.0021972, 0.7621, 0.952, 0.0021, 0.7785496, 0.9454416, 0.0020184,
0.7948408, 0.9384928, 0.0019492, 0.8109472, 0.9311632, 0.0018908, 0.8268424, 0.9234624, 0.0018416, 0.8425, 0.9154, 0.0018,
0.8579448, 0.907008, 0.001766, 0.8730864, 0.898268, 0.001736, 0.8878856, 0.889184, 0.001708001, 0.9023032, 0.87976, 0.001680001,
0.9163, 0.87, 0.001650001, 0.9297792, 0.8598376, 0.001606401, 0.9427736, 0.8493648, 0.001559201, 0.9552584, 0.8386032, 0.0015088,
0.9672088, 0.8275744, 0.0014556, 0.9786, 0.8163, 0.0014, 0.9893944, 0.8047984, 0.001336, 0.9995832, 0.7930952, 0.001272,
1.0091448, 0.7812128, 0.00121, 1.0180576, 0.7691736, 0.001152, 1.0263, 0.757, 0.0011, 1.0340072, 0.744772, 0.0010736,
1.0409616, 0.73244, 0.0010528, 1.0471024, 0.720012, 0.0010352, 1.0523688, 0.707496, 0.0010184, 1.0567, 0.6949, 0.001,
1.0597024, 0.682216, 0.0009648, 1.0617312, 0.669472, 0.0009264, 1.0628088, 0.65668, 0.0008856, 1.0629576, 0.643852, 0.0008432,
1.0622, 0.631, 0.0008, 1.0607856, 0.6181616, 0.00076192, 1.0584528, 0.6053168, 0.00072336, 1.0551672, 0.5924712, 0.00068384,
1.0508944, 0.5796304, 0.00064288, 1.0456, 0.5668, 0.0006, 1.0389456, 0.5539568, 0.00054576, 1.0312768, 0.5411424, 0.00049088,
1.0226352, 0.5283696, 0.00043712, 1.0130624, 0.5156512, 0.00038624, 1.0026, 0.503, 0.00034, 0.991409603, 0.4904928, 0.00031072,
0.979382806, 0.4780624, 0.00028696, 0.966531206, 0.4657056, 0.00026784, 0.952866405, 0.4534192, 0.00025248, 0.9384, 0.4412, 0.00024,
0.923169178, 0.4290768, 0.00022888, 0.907153555, 0.4170064, 0.00021904, 0.890358333, 0.4049776, 0.00020976, 0.872788714, 0.3929792, 0.00020032,
0.8544499, 0.381, 0.00019, 0.834947114, 0.3688624, 0.00017264, 0.814785533, 0.3567632, 0.00015432, 0.794070355, 0.3447328, 0.00013568,
0.772906778, 0.3328016, 0.00011736, 0.7514, 0.321, 0.0001, 0.729613605, 0.309352, 0.00008712, 0.707704806, 0.297896, 0.00007576,
0.685789206, 0.286664, 6.584E-05, 0.663982403, 0.275688, 5.728E-05, 0.6424, 0.265, 5E-05, 0.621684, 0.254824, 4.424E-05,
0.601292, 0.244952, 3.952E-05, 0.581208, 0.235368, 0.00003568, 0.561416, 0.226056, 0.00003256, 0.5419, 0.217, 0.00003,
0.5225672, 0.2081456, 0.00002752, 0.5034976, 0.1995248, 0.00002536, 0.4846944, 0.1911312, 0.00002344, 0.4661608, 0.1829584, 0.00002168,
0.4479, 0.175, 0.00002, 0.4298352, 0.1672112, 0.000018, 0.4120696, 0.1596336, 0.000016, 0.3946264, 0.1522704, 0.000014,
0.3775288, 0.1451248, 0.000012, 0.3608, 0.1382, 0.00001, 0.3444696, 0.1315056, 0.00000768, 0.3285528, 0.1250368, 0.00000544,
0.3130712, 0.1187952, 0.00000336, 0.2980464, 0.1127824, 0.00000152, 0.2835, 0.107, 0.0, 0.269588, 0.101488, -0.00000048,
0.256164, 0.0962, -0.00000064, 0.243216, 0.091128, -0.00000056, 0.230732, 0.086264, -0.00000032, 0.2187, 0.0816, 0.0,
0.2070888, 0.07711584, 0.0, 0.1959104, 0.07281872, 0.0, 0.1851576, 0.06870368, 0.0, 0.1748232, 0.06476576, 0.0,
0.1649, 0.061, 0.0, 0.1553584, 0.05739248, 0.0, 0.1462192, 0.05394944, 0.0, 0.1374808, 0.05066816, 0.0,
0.1291416, 0.04754592, 0.0, 0.1212, 0.04458, 0.0, 0.1136448, 0.04175872, 0.0, 0.1064864, 0.03909056, 0.0,
0.0997256, 0.03657504, 0.0, 0.0933632, 0.03421168, 0.0, 0.0874, 0.032, 0.0, 0.08193696, 0.02997536, 0.0,
0.07684968, 0.02809248, 0.0, 0.07211392, 0.02634192, 0.0, 0.06770544, 0.02471424, 0.0, 0.0636, 0.0232, 0.0,
0.05980472, 0.02179936, 0.0, 0.05625616, 0.02049088, 0.0, 0.05292224, 0.01926272, 0.0, 0.04977088, 0.01810304, 0.0,
0.04677, 0.017, 0.0, 0.04373648, 0.0158864, 0.0, 0.04082704, 0.0148196, 0.0, 0.03804736, 0.0138016, 0.0,
0.03540312, 0.0128344, 0.0, 0.0329, 0.01192, 0.0, 0.03057696, 0.011073104, 0.0, 0.02839808, 0.010279832, 0.0,
0.02636072, 0.009539008, 0.0, 0.02446224, 0.008849456, 0.0, 0.0227, 0.00821, 0.0, 0.021091547, 0.007626184, 0.0,
0.019609007, 0.007088432, 0.0, 0.018244694, 0.006593888, 0.0, 0.01699092, 0.006139696, 0.0, 0.01584, 0.005723, 0.0,
0.014790189, 0.005342896, 0.0, 0.013826372, 0.004994088, 0.0, 0.012939377, 0.004673232, 0.0, 0.01212003, 0.004376984, 0.0,
0.01135916, 0.004102, 0.0, 0.010620661, 0.003835176, 0.0, 0.009929027, 0.003585368, 0.0, 0.009281817, 0.003351672, 0.0,
0.008676593, 0.003133184, 0.0, 0.008110916, 0.002929, 0.0, 0.007581804, 0.002737928, 0.0, 0.007087495, 0.002559424, 0.0,
0.006625685, 0.002392656, 0.0, 0.00619407, 0.002236792, 0.0, 0.005790346, 0.002091, 0.0, 0.005408399, 0.001953072, 0.0,
0.005050688, 0.001823896, 0.0, 0.004715862, 0.001702984, 0.0, 0.004402569, 0.001589848, 0.0, 0.004109457, 0.001484, 0.0,
0.003833315, 0.00138428, 0.0, 0.003575117, 0.00129104, 0.0, 0.003333977, 0.00120396, 0.0, 0.003109009, 0.00112272, 0.0,
0.002899327, 0.001047, 0.0, 0.002704311, 0.000976576, 0.0, 0.002522741, 0.000911008, 0.0, 0.002353666, 0.000849952, 0.0,
0.002196134, 0.000793064, 0.0, 0.00204919, 0.00074, 0.0, 0.001910368, 0.000689869, 0.0, 0.001780609, 0.00064301, 0.0,
0.001659339, 0.000599218, 0.0, 0.001545984, 0.000558283, 0.0, 0.001439971, 0.00052, 0.0, 0.00133968, 0.000483783, 0.0,
0.001245845, 0.000449898, 0.0, 0.001158153, 0.00041823, 0.0, 0.001076292, 0.000388669, 0.0, 0.000999949, 0.0003611, 0.0,
0.000928662, 0.000335357, 0.0, 0.000862306, 0.000311394, 0.0, 0.000800606, 0.000289114, 0.0, 0.000743289, 0.000268415, 0.0,
0.000690079, 0.0002492, 0.0, 0.000640417, 0.000231266, 0.0, 0.000594385, 0.000214643, 0.0, 0.000551777, 0.000199257, 0.0,
0.000512391, 0.000185034, 0.0, 0.000476021, 0.0001719, 0.0, 0.000442421, 0.000159766, 0.0, 0.000411442, 0.000148579, 0.0,
0.000382891, 0.000138269, 0.0, 0.000356575, 0.000128766, 0.0, 0.000332301, 0.00012, 0.0, 0.000309665, 0.000111826, 0.0,
0.000288739, 0.000104269, 0.0, 0.000269383, 9.72792E-05, 0.0, 0.000251459, 9.08064E-05, 0.0, 0.000234826, 0.0000848, 0.0,
0.000219071, 7.91104E-05, 0.0, 0.000204396, 7.38112E-05, 0.0, 0.000190732, 6.88768E-05, 0.0, 0.000178007, 6.42816E-05, 0.0,
0.000166151, 0.00006, 0.0, 0.000154985, 0.000055968, 0.0, 0.000144573, 0.000052208, 0.0, 0.00013487, 0.000048704, 0.0,
0.000125831, 0.00004544, 0.0, 0.000117413, 0.0000424, 0.0, 0.000109535, 3.95552E-05, 0.0, 0.000102198, 3.69056E-05, 0.0,
9.5366E-05, 3.44384E-05, 0.0, 8.90035E-05, 3.21408E-05, 0.0, 8.30753E-05, 0.00003, 0.0, 7.74935E-05, 2.79843E-05, 0.0,
7.22881E-05, 2.61046E-05, 0.0, 6.74367E-05, 2.43526E-05, 0.0, 0.000062917, 2.27205E-05, 0.0, 5.87065E-05, 0.0000212, 0.0,
5.47617E-05, 1.97754E-05, 0.0, 5.10866E-05, 1.84483E-05, 0.0, 4.76644E-05, 1.72125E-05, 0.0, 4.44778E-05, 1.60618E-05, 0.0,
4.15099E-05, 0.00001499, 0.0];


