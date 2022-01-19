
#![doc = include_str!("../README.md")]

#![allow(clippy::approx_constant)]

pub mod ohta;
pub use ohta::*;

pub mod babel;
pub use babel::*;

pub(crate) const M:usize = 24;

pub(crate) static CHECKER_KEYS: [&str; M] = [
		/* 1 */  "dark skin",
		/* 2 */	 "light skin",
		/* 3 */	 "blue sky",
		/* 4 */	 "foliage",
		/* 5 */	 "blue flower",
		/* 6 */	 "bluish green",
		/* 7 */	 "orange",
		/* 8 */	 "purplish blue",
		/* 9 */	 "moderate red",
		/* 10 */ "purple",
		/* 11 */ "yellow green",
		/* 12 */ "orange yellow",
		/* 13 */ "blue",
		/* 14 */ "green",
		/* 15 */ "red",
		/* 16 */ "yellow",
		/* 17 */ "magenta",
		/* 18 */ "cyan",
		/* 19 */ "white 9.5 (.05 D)",
		/* 20 */ "neutral 8 (.23 D)",
		/* 21 */ "neutral 6.5 (.44 D)",
		/* 22 */ "neutral 5 (.70 D)",
		/* 23 */ "neutral 3.5 (1.05 D)",
		/* 24 */ "black 2 (1.5 D)",

];

/**
CIE L\*a\*b\* Values for D50, and 2º observer.

As supplied by X-Rite on their website, for personal and educational use only.
Not for commercial use.

For estimated tolerances, see the [BabelColor Data][babel] page.

*/
pub static CHECKERLAB: [[f64;3];24] = [
    [37.986, 13.555, 14.059],
    [65.711, 18.13, 17.81],
    [49.927, -4.88, -21.925],
    [43.139, -13.095, 21.905],
    [55.112, 8.844, -25.399],
    [70.719, -33.397, -0.199],
    [62.661, 36.067, 57.096],
    [40.02, 10.41, -45.964],
    [51.124, 48.239, 16.248],
    [30.325, 22.976, -21.587],
    [72.532, -23.709, 57.255],
    [71.941, 19.363, 67.857],
    [28.778, 14.179, -50.297],
    [55.261, -38.342, 31.37],
    [42.101, 53.378, 28.19],
    [81.733, 4.039, 79.819],
    [51.935, 49.986, -14.574],
    [51.038, -28.631, -28.638],
    [96.539, -0.425, 1.186],
    [81.257, -0.638, -0.335],
    [66.766, -0.734, -0.504],
    [50.867, -0.153, -0.27],
    [35.656, -0.421, -1.231],
    [20.461, -0.079, -0.973],
];

#[test]
fn checker_ref() {
    use scot::models::CieLab;
    let lab: CieLab = (&CHECKERLAB).into();
    println!("{}", lab.data.transpose());
}