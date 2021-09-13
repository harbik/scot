
use colorado::{SpectralDistribution, observers::CieObsF10, swatches::Tcs};



#[test]
/**
    ColorChecker CieLab values calculation.

    The test values are from the Babel color spreadsheet, with spectral values defined at a domain from 380 to 730nm,
    with 10nm steps.  The values here use the illuminant D50 domain, which uses 5nm steps. This results in small deviations
    in the order of 0.1% in CieLab values.
*/
fn test_munsell_matt() {
    use colorado::illuminants::{D50};
    use colorado::models::CieLab;
    use colorado::observers::CieObs1931;
    use colorado_munsell::MunsellMatt;
	use colorado::differences::{CieDE1994, DeltaEValues};


    let munsell_lab: CieLab<D50, CieObs1931> = MunsellMatt::default().into();
	println!{"{}", munsell_lab.data.transpose()};

	let d: CieDE1994<D50,  CieObsF10> = (Tcs, MunsellMatt).into();
//	let d: CieDE1976 = (Tcs, MunsellMatt).into();
//	let d: CieDE1976 = (Tcs, MunsellMatt).into();
	let im = d.matches();
	//println!("{}", d.matches().column(0));
	let k = MunsellMatt::default().keys().unwrap();
	let matched: Vec<String> =im.row(0).iter().map(|i|k[*i].clone()).collect();
	println!("{:?}", matched);
}

#[test]
fn test_munsell_gloss() {
    use colorado::illuminants::{D50};
    use colorado::models::CieLab;
    use colorado::observers::CieObs1931;
    use colorado_munsell::MunsellGloss;
    use colorado_munsell::MunsellMatt;
	use colorado::differences::{CieDE1994, DeltaEValues};
	use colorado_munsell::munsell_renotation_data;



	let d: CieDE1994<D50,  CieObs1931> = (Tcs, MunsellGloss).into();
//	let d: CieDE1976 = (Tcs, MunsellMatt).into();
//	let d: CieDE1976 = (Tcs, MunsellMatt).into();
	let im = d.matches();
	//println!("{}", d.matches().column(0));
	let k = MunsellGloss::default().keys().unwrap();
	let matched: Vec<String> =im.row(0).iter().map(|i|k[*i].clone()).collect();
	println!("{:?}", matched);
	

	let mrd = munsell_renotation_data();
    let munsell_lab: CieLab<D50, CieObs1931> = MunsellGloss::default().into();
	println!{"{}", munsell_lab.data.transpose()};

	let keys = MunsellMatt::default().keys().unwrap();
	for k in keys.iter() {
		println!("{:?}", k);
		if mrd.contains_key(k.as_str())  {
			println!("{:?}", mrd[k.as_str()]);
		} else {
			println!("no key found");

		}
			

	}
}


static TCS_MUNSELL_VALUES : [(&str, [&str;2]);14] = [
		("TCS01" , 	["7,5 R 6/4", "Light greyish red"]),
		("TCS02" , 	["5 Y 6/4", "Dark greyish yellow"]),
		("TCS03" , 	["5 GY 6/8", "Strong yellow green"]),
		("TCS04" , 	["2,5 G 6/6", "Moderate yellowish green"]),
		("TCS05" , 	["10 BG 6/4", "Light bluish green"]),
		("TCS06" , 	["5 PB 6/8", "Light blue"]),
		("TCS07" , 	["2,5 P 6/8", "Light violet"]),
		("TCS08" , 	["10 P 6/8", "Light reddish purple"]),
		("TCS09" , 	["4,5 R 4/13", "Strong red"]),
		("TCS10" , 	["5 Y 8/10", "Strong yellow"]),
		("TCS11" , 	["4,5 G 5/8", "Strong green"]),
		("TCS12" , 	["3 PB 3/11", "Strong blue"]),
		("TCS13" , 	["5 YR 8/4", "Light yellowish pink"]),
		("TCS14" , 	["5 GY 4/4", "Moderate olive green (leaf"]),
	
];