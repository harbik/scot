use approx::assert_abs_diff_eq;
//use scot::illuminants::BB;
//use scot::models::{CieYxy,YxyValues};
//use scot::observers::CieObs1931;

// x,y, cct, R1..R14, Ra
//const INC_TEST_DATA: [[f64;18];5] = [
//];


#[test]
fn test_bb() {
	use scot::observers::{CieObs1931};
	use scot::models::{CieYxy, YxyValues};
	use scot::illuminants::BB;

	let pl_yxy = CieYxy::<CieObs1931>::from(BB::<2700>);
	let YxyValues { l: _, x, y} = pl_yxy.into_iter().next().unwrap();
	// println!("{} {}",x, y);

	let xr = 0.45984841; // values from CIE TM30 calculation tool
	let yr = 0.410598178;

	assert_abs_diff_eq!(x, xr, epsilon=2E-5);
	assert_abs_diff_eq!(y, yr, epsilon=2E-5);

}

