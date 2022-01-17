use rcs::differences::CieDE2000;
use rcs::observers::CieObs1931;
//use rcs_testcharts::CheckerBabel;
use rcs::illuminants::D50;
use rcs::models::CieLab;
use rcs::swatches::Gray;

pub fn main() {
    let lab_babel : CieLab<D50, CieObs1931> = Gray::<50>.into();
 //   let de = CieDE2000::<D65, CieObs1931>::new(CheckerBabel, CheckerBabel);
}