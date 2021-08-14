use std::collections::HashMap;
use colorado::{ALL, illuminants::CieIllLed, models::{CieYxy, YxyValues}};
use maplit::hashmap;
use colorado::spectra::SpectralData;

fn led_illuminant_data() -> HashMap<&'static str, [f64;2]> {
	hashmap!{
		"LED-B1" => [0.4560, 0.4078],
		"LED-B2" => [0.4357, 0.4012],
		"LED-B3" => [0.3756, 0.3723],
		"LED-B4" => [0.3422, 0.3502],
		"LED-B5" => [0.3118, 0.3236],
		"LED-BH1" => [0.4474, 0.4066],
		"LED-RGB1" => [0.4557, 0.4211],
		"LED-V1" => [0.4548, 0.4044],
		"LED-V2" => [0.3781, 0.3775]
	}

}

#[test]
fn test_xy(){
	use approx::assert_abs_diff_eq;
	let y_xy: CieYxy = CieIllLed::<ALL>.into();
	let testdata=led_illuminant_data();
	let testkeys = CieIllLed::<ALL>.keys().unwrap();
	for (i, YxyValues {l: _,x,y}) in y_xy.into_iter().enumerate(){
		let [xr,yr] = testdata[testkeys[i].as_str()];
		assert_abs_diff_eq!(x,xr,epsilon=5E-5); // precision of test data
		assert_abs_diff_eq!(y,yr,epsilon=5E-5);
		//println!("{} {} {} {}", x, y, xr, yr);
	}

}