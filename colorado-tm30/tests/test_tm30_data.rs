#[test]
fn test_f1(){
	use colorado_tm30::samples::{TM30Illuminant, CieF1};

	use colorado::models::{CieYxy, YxyValues};
	let xy: CieYxy = CieF1::default().into();
	let YxyValues{l:_, x, y} = xy.into_iter().next().unwrap();

	use approx::assert_abs_diff_eq;
	assert_abs_diff_eq!(x,0.313100, epsilon=1E-6);
	assert_abs_diff_eq!(y,0.337279, epsilon=1E-6);

}



#[test]
fn ces_ucs(){
    use colorado::observers::CieObs1964;
    use colorado::models::cam02::{CieCamUcs, VcTm30};
    //use colorado::illuminants::D50;
    use colorado_tm30::ces::Ces;
    use colorado_tm30::samples::{TM30Illuminant, CieF1};
    use nalgebra::{SMatrix, matrix};
    
    let jab: CieCamUcs<VcTm30, CieF1, CieObs1964> = Ces::default().into();
    println!("{:.2}", jab.data.transpose());



    // Data for F1 Illuminant, from ANSI-IES-TM30-18 Advanced Calculation Tool V2.0
    let want: SMatrix<f64, 3, 99> = matrix![
        84.47803641, 58.31960581, 29.81095962, 68.11998788, 44.68788472, 49.23607619, 37.24951104, 36.69455937,
        28.58219827, 71.20466107, 54.3859311, 59.86628863, 40.50774198, 73.56095324, 69.73652655, 45.01716981,
        47.98295161, 54.75447471, 69.86896244, 62.78187795, 84.31411887, 76.16073204, 91.13147253, 89.6422972,
        70.25888136, 88.38355476, 73.06513023, 41.81202674, 85.86137723, 63.89726055, 87.70679434, 74.6181675,
        89.27919636, 62.16007991, 42.74527913, 80.59932493, 48.62355452, 86.26227812, 35.24502096, 49.90162495,
        88.25847443, 69.75319481, 73.51775667, 28.83562709, 59.76657877, 67.80586264, 70.37040451, 79.36355611,
        47.50061303, 55.68552185, 59.26237764, 42.2548901, 51.80125275, 88.2019067, 84.76206247, 73.51714715,
        60.22941446, 56.51213475, 87.80835511, 88.64604293, 77.61636813, 67.43711094, 34.841582, 60.70406838,
        41.81636687, 46.24732202, 49.57802048, 64.8638065, 53.45335072, 73.62881319, 35.98316771, 77.11671513,
        59.16667274, 28.42196417, 33.99731582, 48.78959735, 44.92701287, 45.11216417, 73.97377098, 65.93551894,
        43.52982703, 87.42784876, 73.19536816, 27.75362407, 40.92189749, 64.69929093, 38.391907, 60.96794914,
        36.93020811, 42.27145664, 80.44440261, 54.69669951, 66.44978332, 36.86969989, 68.621536, 77.10916112,
        51.17890476, 61.72218156, 48.27789082;
        14.07393212, 26.00405014, 7.894779592, 19.42339154, 25.68169866, 14.21296352, 22.41748488, 18.73788499,
        0.570089376, 24.6779308, 24.67000289, 25.49322178, 11.49964252, 4.524518305, 6.768747584, 8.991385794,
        10.3664288, 6.414691859, 11.67564615, 8.885988805, 10.5751372, 10.144275, 2.264285837, 4.735270818, 5.74190334,
        3.480524472, -0.228869232, 0.047701377, -1.845353277, -2.09891246, -2.894411905, -3.553955947, -2.963961743,
        -3.301997214, -1.821704938, -1.973224907, -3.44446846, -5.29017686, -3.58598019, -6.743893322, -4.937835469,
        -13.64539766, -14.99430575, -0.992277803, -14.55886805, -11.29547304, -11.38378414, -14.45115422, -14.86823774,
        -16.33055289, -15.43362012, -18.97861559, -26.30657344, -12.44170255, -15.21372888, -25.55404987, -27.34071602,
        -27.23313059, -13.28898048, -6.578784576, -13.16474757, -7.822051063, -2.766592132, -26.00864188, -13.81228362,
        -24.05339758, -24.18497775, -16.05528294, -9.485019053, -15.49168135, -15.17629365, -5.74407039, -17.66284079,
        -2.744863682, -11.6249031, -12.92167769, -10.49955436, -9.909791222, -5.255391969, -1.338732082, -1.554536834,
        0.463031314, 2.96833095, -0.018383139, 0.490033604, 2.576642601, 7.101160879, 11.91568882, 11.66411213,
        15.29752232, -0.242643296, 6.096554416, 7.469736563, 13.53121863, 10.4926492, 12.09955015, 22.29766299,
        26.04079846, 24.47794156;
        -1.358663773, -0.911529935, 0.402798249, 3.161487514, 3.6465807, 4.55911958, 6.662978139, 6.078235723,
        0.529282386, 16.05256487, 17.01519302, 18.58558141, 13.4701049, 5.608161711, 10.67209878, 16.25477453,
        17.20175597, 13.63454945, 23.3810808, 30.64091807, 28.621612, 30.2014407, 12.92414605, 25.06746315, 33.86001697,
        30.43233351, 16.2892414, 16.0397798, 32.03233821, 20.2799065, 31.80441913, 34.28412941, 22.26317883,
        30.25694741, 16.09702352, 8.992520437, 19.98949427, 19.82048838, 7.713638007, 14.26507257, 10.21101607,
        30.99379357, 32.66001386, 1.510128568, 23.47146052, 19.14446326, 14.4585723, 19.62314319, 17.07311504,
        10.45726475, 8.78452773, 10.87238864, 9.431625329, 2.940970115, 1.62947169, 2.066852734, 2.235444479,
        -0.368849169, -0.955939402, -1.219690761, -3.699974055, -0.987320896, -0.35551609, -11.14939836, -7.394886895,
        -13.58627349, -15.20972188, -13.88665898, -9.655441333, -20.18860266, -18.09631426, -7.625133436, -25.34131934,
        -2.658593059, -20.12715967, -30.49679377, -23.00368945, -30.25424415, -20.10755049, -23.44214308, -28.77059471,
        -10.09175372, -17.28737924, -7.967458308, -13.92517308, -24.42032553, -23.68755178, -20.6916185, -18.45757911,
        -19.45114432, -1.387634592, -12.82765777, -9.389378234, -13.37693116, -9.451799332, -7.889454495, -6.547897988,
        -5.562518279, -6.937316356;
    ];
}
