use colorado_tm30::IesTm30AllFluorescents as FL;
use colorado::models::{CieYxy, YxyValues};
use colorado::SpectralDistribution;


static FL_WANT: [(&str, [f64;2]);3] =
[
	("F32T8/930", [0.4401220, 0.3995537]),
	("F32T8/950", [0.3386684, 0.3510573]),
	("F34T12/CW/RS/EW", [0.3772841, 0.3932596]),
	
];

#[test]
fn test_colorpoints(){
	let xyz: CieYxy = FL.into();
	let keys = FL.keys().unwrap();
	for (i,YxyValues{l: _, x, y}) in xyz.into_iter().enumerate() {
		println!("{} x: {} y: {}", keys[i].split("|").next().unwrap(), x, y);
	}
}