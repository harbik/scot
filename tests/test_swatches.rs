

#[test]
/**
    ColorChecker CieLab values calculation.

    The test values are from the Babel color spreadsheet, with spectral values defined at a domain from 380 to 730nm,
    with 10nm steps.  The values here use the illuminant D50 domain, which uses 5nm steps. This results in small deviations
    in the order of 0.1% in CieLab values.
*/
fn test_cielab_colorchecker() {
    use colorado::illuminants::CieIllD50;
    use colorado::models::CieLab;
    use colorado::observers::CieObs1931;
    use colorado::swatches::checker::ColorChecker;
    use colorado::swatches::{G18, White};
	use colorado::models::LabValues;

    use approx::assert_abs_diff_eq;
    use nalgebra::matrix;

    let white: CieLab<CieIllD50, CieObs1931> = White::default().into();
	let LabValues{l, a, b} = white.into_iter().next().unwrap();
    assert_abs_diff_eq!(l, 100.0, epsilon = 0.00001);
    assert_abs_diff_eq!(a, 0.0, epsilon = 0.00001);
    assert_abs_diff_eq!(b, 0.0, epsilon = 0.00001);
    //	println!("White {:.4}", white);

    let gray: CieLab<CieIllD50, CieObs1931> = G18::default().into();
	let LabValues{l, a, b} = gray.into_iter().next().unwrap();
	//println!("Gray {:.4}", l);
	assert_abs_diff_eq!(l, 49.496107, epsilon = 0.00001); // close to L50
	assert_abs_diff_eq!(a, 0.0, epsilon = 0.00001);
	assert_abs_diff_eq!(b, 0.0, epsilon = 0.00001);

    let checker_lab: CieLab<CieIllD50, CieObs1931> = ColorChecker::default().into();
    let babel = matrix![
        38.44, 13.61, 14.53;
        65.95, 17.91, 17.87;
        50.06, -4.52, -22.25;
        43.28, -13.21, 21.94;
        55.31, 8.82, -24.60;
        70.69, -33.03, -0.11;
        62.65, 35.35, 57.86;
        40.24, 9.74, -44.35;
        51.60, 47.80, 16.90;
        30.50, 21.07, -20.02;
        72.46, -23.30, 57.00;
        71.95, 19.46, 68.12;
        28.87, 14.81, -50.15;
        55.15, -37.80, 31.64;
        42.28, 54.12, 28.67;
        82.27, 4.02, 79.99;
        51.91, 49.80, -13.82;
        50.72, -28.11, -27.95;
        96.53, -0.47, 2.42;
        81.21, -0.64, 0.27;
        66.48, -0.53, 0.00;
        50.83, -0.64, -0.14;
        35.85, -0.54, -0.49;
        20.81, 0.03, -0.39
    ];

    for (i, cc) in checker_lab.data.column_iter().enumerate() {
        assert_abs_diff_eq!(cc.x, babel[(i, 0)], epsilon = 0.05);
        assert_abs_diff_eq!(cc.y, babel[(i, 1)], epsilon = 0.05);
        assert_abs_diff_eq!(cc.z, babel[(i, 2)], epsilon = 0.05);
    }
}