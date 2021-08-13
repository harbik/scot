use approx::assert_abs_diff_eq;
use colorado::{self, ALL, illuminants::HP, models::CieYxy, models::YxyValues, observers::CieObs1931};

// x,y, cct, R1..R14, Ra
const HID_TEST_DATA: [[f64;18];5] = [
	[0.533, 0.415, 1959.0, -3.0, 61.0, 40.0, -27.0, -4.0, 52.0, 21.0, -75.0, -260.0, 43.0, -52.0, 27.0, 7.0, 61.0, 8.0],
	[0.4778, 0.4158, 2506.0, 98.0, 89.0, 73.0, 89.0, 88.0, 71.0, 81.0, 72.0, 52.0, 66.0, 66.0, 55.0, 90.0, 82.0, 83.0],
	[0.4302, 0.4075, 3144.0, 87.0, 92.0, 87.0, 89.0, 85.0, 90.0, 82.0, 50.0, -29.0, 71.0, 89.0, 72.0, 90.0, 91.0, 83.0],
	[0.3812, 0.3797, 4002.0, 75.0, 85.0, 84.0, 78.0, 75.0, 79.0, 77.0, 42.0, -60.0, 56.0, 77.0, 64.0, 79.0, 91.0, 74.0],
	[0.3776, 0.3713, 4039.0, 87.0, 94.0, 97.0, 89.0, 89.0, 94.0, 85.0, 64.0, 10.0, 85.0, 90.0, 90.0, 90.0, 98.0, 87.0]
];

#[test]
fn test_yxy(){
	for (YxyValues {l: _, x, y}, [xr,yr,..]) in CieYxy::<CieObs1931>::from(HP::<ALL>).into_iter().zip(HID_TEST_DATA.iter()) {
		assert_abs_diff_eq!(x, xr, epsilon=5E-5); // 4 decimal precision for x,y values given in reference table
		assert_abs_diff_eq!(y, yr, epsilon=5E-5);
		//println!("{} {} {} {}", x,y, xr, yr );

	}
}

