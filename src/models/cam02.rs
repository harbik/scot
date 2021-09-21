/*!

# CIECAM02 Color Model


This is a complex model, and to get satisfactory results requires a good understanding of all its – sometimes confusing – input and output parameters.

The output of the model are six color appearance attributes, or correlates,
which are described by [By Luo et all in "CIECAM02 and Its Recent Developments"][CIECAM02LUO] as follows:

<blockquote><i>

- Brightness (Q)

  This is a visual perception according to which an area appears to exhibit more or less light. This is an openended scale with a zero origin defining the black.
  The brightness of a sample is affected by the luminance of the light source used. A surface colour illuminated by a higher luminance would appear brighter than the same surface illuminated by a lower luminance. \[...\]
  Brightness is an absolute quantity, for example, a colour appears much brighter when it is viewed under bright outdoor sunlight than under moonlight. Hence, their Q values could be largely different

- Lightness (J)

  This is the brightness of an area judged relative to the brightness of a similarly illuminated reference white.
  It is a relative quantity, for example, thinking a saturated red colour printed onto a paper. The paper is defined as reference white having a lightness of 100. By comparing the light reflected from both surfaces in the bright sunlight, the red has a lightness of about 40% of the reference white (J value of 40). When assessing the lightness of the same red colour under the moonlight against the same reference white paper, the lightness remains more or less the same with a J of 40.
  It can be expressed by J = Q<sub>S</sub>/Q<sub>W</sub>, where Q<sub>S</sub> and Q<sub>W</sub> are the brightness values for the sample and reference white, respectively.

- Colourfulness (M)

  Colourfulness is that attribute of a visual sensation according to which an area appears to exhibit more or less chromatic content.
  This is an open-ended scale with a zero origin defining the neutral colours. Similar to the brightness attribute, the colourfulness of a sample is also affected by luminance. An object illuminated under bright sunlight would appear more colourful than when viewed under moonlight, such as M value changes from 2000 to 1 with a ratio of 2000.

- Chroma (C)

  This is the colourfulness of an area judged as a proportion of the brightness of a similarly illuminated reference
  white. This is an open-ended scale with a zero origin representing neutral colours. It can be expressed by C = M/Q<sub>W</sub>.
  The same example is given here, a saturated red printed on a white paper. It has a colourfulness of 50 against the white paper having a brightness of 250 when viewed under sunlight. When viewed under dim light, colourfulness reduces to 25 and brightness of paper also reduces to half. Hence, the C value remains unchanged.

- Saturation (S)

  This is the colourfulness of an area judged in proportion to its brightness as expressed by s = M/Q, or s = C/J. This scale runs from zero, representing neutral colours, with an open end.
  Taking Figs. 2.3–2.5 as an example, the green grass under sunlight is bright and colourful. In contrast, those under the tree appear dark and less colourful. Because they are the same grass in the field, we know that they have the same colour, but their brightness and colourfulness values are largely different. However, their saturation values will be very close because it is the ratio between brightness and colourfulness. Similar example can also be found in the image on the brick wall. Hence, saturation could be a good measure for detecting the number and size of objects in an image.

- Hue (h  and H)

  Hue is the attribute of a visual sensation according to which an area appears to be similar to one, or to proportions of two, of the perceived colours red, yellow, green and blue.
  CIECAM02 predicts hue with two measures: hue angle (h) ranging from 0º to 360º, and hue composition (H) ranging from 0, through 100, 200, 300, to 400 corresponding to the psychological hues of red, yellow, green, blue and back to red. These four hues are the psychological hues, which cannot be described in terms of any combinations of the other colour names. All other hues can be described as a mixture of them. For example, an orange colour should be described as mixtures of red and yellow, such as 60% of red and 40% of yellow.

</i></blockquote>

Similar to CIELAB, also here two additional correlates are defined:

- Redness-Greenness (a)

- Yellowness-Blueness (b)


# Viewing Conditions Input Parameters

CIECAM02 takes into account the viewing conditions of the model target.
To define these viewing condtions, the following terms are used:

- A "Stimulus", or "Target", is the element for which a CIECAM02 values is determined. This can take different forms: it can be for example a color swatch, or color sample of a material, which is in the model assumed to have a 2º angular extend, and assumed to be uniform. For a display, it is a bit more confusing to defined the target: is it a single pixel, or a collection of uniform pixels, with an angular size of 2º, or is it the full display, with all the pixels set to the same RGB pixel values?

- The next is "Reference White".
For the model we need the its tristimulus values X<sub>W</sub>, Y<sub>W</sub>, and Z<sub>W</sub>, and its absolute luminance, L<sub>W</sub>.
What is this Reference White, and its values?
Are the Referenfe White tristimulus values obtained by measuring a perfectly white reflecting sample, instead of the "colored' target sample?
Or is it the "Adopted White", as defined by ISO12231 as <i>"a stimulus that an observer who is adapted to the viewing environment would judge to be perfectly achromatic and to have a reflectance factor of unity (i.e., have absolute colorimetric coordinates that an observer would consider to be the perfect white diffuser)"</i>.
Very confusingly, sometimes the term "Adapted White" is used as well, which is the white point an observers assigns to a scene,
for example	an image viewed on a laptop computer, with the "adapted white" being the white in the scene of the image on the display, which can outdoor daylight scene, with a correlated color temperature of 6500K.
Looking at a image scene on a monitor, it is not always clear what the white point in the image is supposed to be; a very clear example of this is the famous picture of a dress, which can be interpreted as white-yellow, or blue-black, depending on the white point adapted by the observer. In that particular case, it is impossible to to describe its color appearance by the CIECAM model, or for that matter any color model at all.

- The "Proximal Field" is the area extending 2º around the Stimulus. This area is currently
ignored in the model, and are considered to be part of the Background.

- The "Background" in a model test set-up is the area around the Proximal Field, extending over a field angle of about 10º.
When considering color swatches, this background area is typically well defined: for images not;
sometimes the average of all the pixels in an image is taken as a representation of its background.
In CIECAM, only the relative luminance of the background area is used, with the symbol Y<sub>B</sub>, and is often set to a value of 20.

- "Surround" is the angular field outside the background, extending to an eye's full view.

# CIECAM Model Input Parameters
The following parameters are used in CIECAM02:


The first two are the same as used in the CIELAB model:
- the tristimulus values (X,Y,Z) of the target (e.g. surface, or pixel) to be described,
- and the tristimulus values (X<sub>W</sub>,Y<sub>W</sub>,Z<sub>W</sub>) of the refetrence white (perfect white surface, or white pixel).
For the model, the tristimulus values are normalized to Y<sub>W</sub>=100.


- The absolute luminance of the adapting field (L<sub>A</sub>), in cd/m<sup>2</sup>, which should be ideally measured with a photometer,
but which can be approximated by setting it to 20% of the luminance of the reference white L<sub>W</sub>, assuming an average reflectivity
of 20% for the objects in the adapting field.
For example, in a brightly lit room, average illuminance is 1000lux, which is approximately 318 cd/m<sup>2</sup>, so L<sub>W</sub>=318 cd/m2<sup>2</sup>,
resulting in an L<sub>A</sub>=64 cd/m<sup>2</sup>.
- The background relative luminance (Y<sub>b</sub>), which is 20 for the average "world grey" assumption in the previous item, but ideally be derived from
the absolute luminances of the background, and the absolute luminance of the reference white.
- A surround ratio (S<sub>R</sub>), the relative luminance of the target's surround, an area of approximately 10º in field around the target, which is assumed to have an angular extend of 2º. It is typically approximated by values described as "average", "dim", and
"dark", and is used through its derived dimensionless parameters impact factor c, chromatic induction factor N<sub>c</sub>, and degree of
adaptation F.
- The last parameter is the parameter D, representing in how far color constancy is in effect, also known as discounting-the-illuminant phenomenom.
It can either be approximated by using the other parameters, or set manually; D is set to 1.0 when the illuminant is fully discounted, for example when viewing surface colors in a bright environment,
or is set to 0.0, at the other end of its range, when not in effect at all, such as when viewing pixels on a display in a complete dark environment,
In practice the value of D is never 0.0, as there is always some adaptation taking place, and is in higher than 0.6 in almost all cases.
If D is larger than 0, a chromatic adaptation transform is applied to a target's tristimulus values, described by the CIECAT02, the CIE recommended Chromatic Adaptation Transform as defined in 2002.


[CIECAM02LUO]: https://link.springer.com/chapter/10.1007/978-1-4419-6190-7_2 "C. Fernandez-Maloigne (ed.), Advanced Color Image Processing and Analysis,  DOI 10.1007/978-1-4419-6190-7 2, Springer Science+Business Media New York 2013"

 */

use core::panic;
use std::{marker::PhantomData, };

use nalgebra::{Matrix3x1, Matrix3xX, SMatrix, matrix};

use crate::{DefaultObserver, illuminants::{D65}, linterp, models::XYZValues, observers::{StandardObserver}};

use super::{CieLab, CieXYZ};

/*
   TODO:
   - CIECAT02 transform for non-cie1931 observer?
*/

pub trait CieCamParameters 
where
	Self: Default
{

    fn sr_value(&self) -> f64;

    fn yb_value(&self) -> f64;

    fn la_value(&self) -> f64;

    fn d_value(&self) -> DFactor;

    /*
         Sr		 F		  c		 Nc
        >=0.15 	1.0		0.69	1.0	Average
        0.075	0.9		0.59	0.9	Dim // this is listed as 0 < Sr < 0.15
        0.0		0.8		0.525	0.8	Dark
    */

    fn c(&self) -> f64 {
        let s = self.sr_value();
        if s >= 0.15 {
            0.69
        } else if s > 0.075 {
			linterp(s, 0.075, 0.59, 0.15, 0.69)
        } else {
			linterp(s, 0.0, 0.525, 0.075, 0.59)
        }
    }

    fn f(&self) -> f64 {
		let c = self.c();
        if c > 0.59 {
			linterp(c, 0.59, 0.9, 0.69, 1.0)
        } else {
			linterp(c, 0.525, 0.8, 0.59, 0.9)
        }
    }

	fn nc(&self) ->f64 {
		self.f()
	}

    fn k(&self) -> f64 {
        1. / (5. * self.la_value() + 1.)
    }

    fn fl(&self) -> f64 {
        let k = self.k();
        k.powi(4) * self.la_value()
            + (1. - k.powi(4)).powi(2) / 10. * (5.0 * self.la_value()).powf(1. / 3.)
    }
    fn d(&self) -> f64 {
        match self.d_value() {
            DFactor::Auto => {
                self.f() * (1.0 - (1.0 / 3.6) * ((-1.0 * self.la_value()) - 42.0) / 92.0).exp()
            }
            DFactor::D(v) => v,
        }
    }
}

pub static MHPE: SMatrix<f64, 3, 3> = matrix![
     0.38971, 0.68898, -0.07868;
    -0.22981, 1.18340,  0.04641;
     0.00000, 0.00000,  1.00000;
];

pub static MHPEINV: SMatrix<f64, 3, 3> = matrix![
    1.910197, -1.112124,  0.201908;
    0.370950,  0.629054, -0.000008;
    0.000000,  0.000000,  1.000000;
];

pub static MCAT02: SMatrix<f64, 3, 3> = matrix![
     0.7328,  0.4296,  -0.1624;
    -0.7036,  1.6975,   0.0061;
     0.0030,  0.0136,   0.9834;
];
pub static MCAT02INV: SMatrix<f64, 3, 3> = matrix![
     1.096124, -0.278869, 0.182745;
     0.454369,  0.473533, 0.072098;
    -0.009628, -0.005698, 1.015326;
];
/*
pub static MCAT02: SMatrix<f64,3,3> = SMatrix::from_array_storage(ArrayStorage([
    [0.7328, -0.7036, 0.003],
    [0.4296, 1.6975, 0.0136],
    [-0.1624, 0.0061, 0.9834]
]));
 */

#[test]
fn test_mcat02() {
    println!("{}", MCAT02);
    println!("{}", MCAT02INV);
    println!("{}", MCAT02.try_inverse().unwrap());
    println!("{}", MHPE);
    println!("{}", MHPE * MHPEINV);
}

/**
# CieCam View conditions

- SR100 = Sr * 100, Surround Ratio
- YB: Relative background luminance, relative to YW=100.0,
  20.0 is a common number, being 'world grey'.
- LA: Absolute Luminance (in cd/m<sup>2</sup>) of the adapting field.
  If unsure, use 20% of Lw; for example 20% * 300 cd/m<sup>2</sup> = 60 cd/m<sup>2</sup>.
- D100: Discounting-the-illuminant parameter, or color constancy parameter.
  This parameter determines if chromatic adaptation is applied to calculate the ciecam values of the target.
  This compensates for the chromatic adaptation of our eyes to ambient light.

  - D_AUTO: Calculate discounting factor automatically, using the other viewconditions parameters.
    Use this if you are not sure what value to use.
  - D100=0: No dicounting, so no chromatic adaptation applied.
    This can be used in case looking at a display, in a very dark room.
  - D100=100: Full discounting, full chromatic adaptation used.
    This value is used if you watching colorsamples in full daylight for example.
  - D100=0..100: discounting factor of D/100 is used.
    Set your own value, if you want to experiment.

*/
#[derive(Default)]
pub struct ViewConditions<const LA: usize, const YB: usize, const SR1000: usize, const D100: isize>;

type VcAvg = ViewConditions<100, 20, SR_AVG, D_AUTO>;

pub const SR_AVG: usize = 0.150E3 as usize;
pub const SR_DIM: usize = 0.075E3 as usize;
pub const SR_DARK: usize = 0;

pub const D_AUTO: isize = -1;

pub enum DFactor {
    Auto,
    D(f64),
}

impl<
	const LA: usize, 
	const YB: usize, 
	const SR1000: usize, 
	const D100: isize
	> CieCamParameters for ViewConditions<LA, YB, SR1000, D100>
{
    fn sr_value(&self) -> f64 {
        SR1000 as f64 / 1000.0
    }

    fn yb_value(&self) -> f64 {
        YB as f64
    }

    fn la_value(&self) -> f64 {
        LA as f64
    }

    fn d_value(&self) -> DFactor {
        if D100 == D_AUTO {
            DFactor::Auto
        } else {
            DFactor::D((D100 as f64 / 100.0).clamp(0.0, 1.0))
        }
    }
}


// Illumiant reference white Lw


// Table 16.4, Mark Fairchild, Color Appearance Models
// /*X	Y	Z*/	Xw	Yw	Zw	LA	F	D	Yb	Nc	Fl	Nbb,Ncb	h	H	/*Hc*/	J	Q	S	C	M	ac	bc	am	bm	as	bs

pub static CIECAM_WANT: SMatrix<f64, 4, 23> = matrix![
/*19.01, 20.0, 21.78,*/ 95.05, 100.0, 108.88, 318.31, 1.0, 0.994, 20.0, 1.0, 1.17, 1.0, 219.0, 278.1, /*"78B, 22G",*/ 41.73, 195.37, 2.36, 0.1, 0.11, -0.08, -0.07, -0.08, -0.07, -1.83, -1.49;
/*57.06, 43.06, 31.96,*/ 95.05, 100.0, 108.88, 31.83, 1.0, 0.875, 20.0, 1.0, 0.54, 1.0, 19.6, 399.6, /*100R,*/ 65.96, 152.67, 52.25, 48.57, 41.67, 45.77, 16.26, 39.27, 13.95, 49.23, 17.49;
/*3.53, 6.56, 2.14,*/ 109.85, 100.0, 35.58, 318.31, 1.0, 0.994, 20.0, 1.0, 1.17, 1.0, 177.1, 220.4, /*"80G, 20B",*/ 21.79, 141.17, 58.79, 46.94, 48.8, -46.89, 2.43, -48.74, 2.43, -58.72, 2.93;
/*19.01, 20.0, 21.78,*/ 109.85, 100.0, 35.58, 31.83, 1.0, 0.875, 20.0, 1.0, 0.54, 1.0, 248.9, 305.8, /*"94B, 6R",*/ 42.53, 122.83, 60.22, 51.92, 44.54, -18.69, -48.44, -16.03, -41.56, -21.67, -56.18;
];

pub struct CieCam<V = VcAvg, I = D65, C = DefaultObserver> {
	pub data: Matrix3xX<f64>, // JCh
	v: PhantomData<*const V>,
	i: PhantomData<*const I>,
	c: PhantomData<*const C>,
}

impl<V, I, C> CieCam<V, I, C> {

    pub fn new(data: Matrix3xX<f64>) -> Self { 
		Self { data, i:PhantomData, c:PhantomData, v:PhantomData } 
	}

	pub fn len(&self) -> usize {
        self.data.ncols()
    }
}

impl<V, I,C> From<CieLab<I,C>> for CieCam<V,I,C>
where
	I: Default + Into<CieXYZ<C>>,
	C: StandardObserver,
	V: CieCamParameters,
{
    fn from(lab: CieLab<I,C>) -> Self {
		let vc = V::default();
		let xyz_w: CieXYZ<C> = I::default().into().normalize(100.0);
		let y_w = xyz_w.data[(1,0)]; // = 100.0
		let n = vc.yb_value()/y_w;
		let c = vc.c();
		let z = n.sqrt() + 1.48;
		let n_bb = 0.725 * n.powf(-0.2);
		let n_cb = n_bb;
		let d = vc.d();
		let f_l = vc.fl();

		let rgb_w =  &MCAT02 * &xyz_w.data;
		
		let nom = Matrix3x1::from_element(d * y_w);
		let mut d_rgb = nom.component_div(&rgb_w);
		d_rgb.add_scalar_mut(1.0 + d);
		let rgb_wc = d_rgb.component_mul(&rgb_w);
		let rgb_p_w = &MHPE * &MCAT02INV * rgb_wc;
		let rgb_p_aw = rgb_p_w.map(|r|{
			let t = (f_l * r/100.0).powf(0.42);
			400.0 * (t/(t+27.13)) + 0.1
		});
		let a_w = (2.0 * rgb_p_aw.x + rgb_p_aw.y + rgb_p_aw.z/20.0-0.305)*n_bb;
		let xyz: CieXYZ<C> = lab.into();

		let rgb = &MCAT02 * xyz.data;
		let d_rgbs = Matrix3xX::from_fn(rgb.ncols(), |i,j|
			match i {
				0 => d_rgb.x,
				1 => d_rgb.y,
				2 => d_rgb.z,
				_ => panic!("index error!"),
			}
		);
		let rgb_c = d_rgbs.component_mul(&rgb);
		let rgb_p = MHPE * MCAT02INV * rgb_c;
		let rgb_p_a = rgb_p.map(|r|{
			if r>=0.0 {
				let t = (f_l * r/100.0).powf(0.42);
				400.0 * (t/(t+27.13)) + 0.1
			} else {
				let t = (-f_l * r/100.0).powf(0.42);
				-400.0 * (t/(t+27.13)) + 0.1
			}
		});
		let jab = Matrix3xX::from_fn(rgb.ncols(),|i,j|{
			match i {
				0 => { // Step 8: J
					let a = (2.0 * rgb_p_a.column(j).x + rgb_p_a.column(j).y + rgb_p_a.column(j).z/20.0 - 0.305) * n_bb;
					100.0 * (a/a_w).powf(c*z)
				},
				1 => { // Step 5: a
					rgb_p_a.column(j).x	- 12.0/11.0 * rgb_p_a.column(j).y + rgb_p_a.column(j).z / 11.0
				},
				2 => {
					(rgb_p_a.column(j).x + rgb_p_a.column(j).y - 2.0 *  rgb_p_a.column(j).z) / 9.0

				},
				_ => panic!("index error!"),


			}
		});

		Self::new(jab)
    }
}

#[test]
fn test_from_lab(){
	use crate::observers::CieObs1931;
	let lab: CieLab<D65> = CieLab::new(Matrix3xX::from_vec(vec![
		19.01, 20.0, 21.78, 
	//	57.06, 43.06, 31.96,
		3.53, 6.56, 2.14,
//		19.01, 20.0, 21.78,
	]));
	let cam: CieCam<ViewConditions<318, 20, SR_AVG, D_AUTO>, D65, CieObs1931> = lab.into();
	println!("{}", cam.data);
}

/*
pub type VcAverage = ViewConditions<AVG, 100, 690, 100, false>;
pub type VcDim= ViewConditions<DIM, 90, 590, 90, false>;
pub type VcDark = ViewConditions<DARK, 80, 525, 80, false>;
 */

/*
use std::f64::consts::PI;
use std::iter::Iterator;

use crate::cie::XYZ;
use crate::observers::ObserverKey;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ViewConditions {
    pub D_opt: Option<f64>,
    pub F: f64,
    pub La: f64,
    pub Nc: f64,
    pub Yb: f64,
    pub c: f64,
}

impl ViewConditions {
    pub fn new(Yb: f64, F: f64, Nc: f64, c: f64, La: f64, D_opt: Option<f64>) -> ViewConditions {
        ViewConditions {
            Yb,
            F,
            Nc,
            c,
            La,
            D_opt,
        }
    }

    #[inline]
    pub fn k(&self) -> f64 {
        1. / (5. * self.La + 1.)
    }

    #[inline]
    pub fn Fl(&self) -> f64 {
        let k = self.k();
        k.powi(4) * self.La + (1. - k.powi(4)).powi(2) / 10. * (5.0 * self.La).powf(1. / 3.)
    }

    pub fn D(&self) -> f64 {
        let D: f64;
        if let Some(d) = self.D_opt {
            D = d;
        } else {
            D = self.F * (1.0 - (1.0 / 3.6) * ((-1.0 * self.La - 42.0) / 92.0).exp());
        }

        if D < 0.0 {
            0.0
        } else if D > 1.0 {
            1.0
        } else {
            D
        }
    }

    pub fn lum_adapt(&self, rgb: [f64; 3]) -> [f64; 3] {
        let fl = self.Fl();
        let mut rgb_adapt = rgb.clone();
        rgb_adapt.iter_mut().for_each(|v| {
            if *v >= 0.0 {
                let t = (fl * *v / 100.).powf(0.42);
                *v = 400. * t / (27.13 + t) + 0.1
            } else {
                let t = (-fl * *v / 100.).powf(0.42);
                *v = -400. * t / (27.13 + t) + 0.1
            }
        });
        rgb_adapt
    }
}

impl Default for ViewConditions {
    fn default() -> Self {
        Self {
            Yb: 20.0,
            c: 0.69,
            Nc: 1.0,
            F: 1.0,
            La: 100.0,
            //		D_opt: Some(1.0),
            D_opt: None,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CIECAM {
    pub vc: ViewConditions,
    pub jch: Vec<[f64; 3]>,
    pub xyz_w: [f64; 3],
    pub obs: ObserverKey,
}

impl CIECAM {
    pub fn from(xyz: XYZ, vc: ViewConditions) -> CIECAM {
        // See Luo CIECAM02 and its recent developments Appendix A, part 1: The forward mode
        let xyz_w = xyz.xyz_w.expect("Need white point");
        let mut rgb_w = m_cat02(xyz_w);
        let vcd = vc.D();
        let fl = vc.Fl();

        let Yw = xyz_w[1];
        let dr = vcd * Yw / rgb_w[0] + 1.0 - vcd;
        let dg = vcd * Yw / rgb_w[1] + 1.0 - vcd;
        let db = vcd * Yw / rgb_w[2] + 1.0 - vcd;


        let n = vc.Yb / Yw;
        let z = n.sqrt() + 1.48;
        let Nbb = 0.725 * n.powf(-0.2);
        let Ncb = Nbb;

        // rgb_wc
        rgb_w[0] *= dr; rgb_w[1] *= dg; rgb_w[2] *= db;

        // rgb_pw
        rgb_w = m_cat02_inv(rgb_w);
        rgb_w = m_hpe(rgb_w);

        // rgb_paw
        rgb_w = vc.lum_adapt(rgb_w);

        let Aw = achromatic_rsp(rgb_w, Nbb);

        let mut jch: Vec<[f64; 3]> = Vec::with_capacity(xyz.len());

        for _xyz in xyz.xyz_vec {
            let mut rgb = m_cat02(_xyz);

            // rgb_C
            rgb[0] *= dr; rgb[1] *= dg; rgb[2] *= db;

            // rgb_p
            rgb = m_cat02_inv(rgb);
            rgb = m_hpe(rgb);

            // rgb_pa
            rgb = vc.lum_adapt(rgb);

            let ca = rgb[0] - 12.0 / 11.0 * rgb[1] + rgb[2] / 11.0;
            let cb = (rgb[0] + rgb[1] - 2. * rgb[2]) / 9.0;

            // calculate h in radians
            let mut h = cb.atan2(ca);
            if h<0.0 { h+= 2.0* PI;}


            // calculate J = jj
            let jj = 100.0 * (achromatic_rsp(rgb, Nbb) / Aw).powf(vc.c * z);

            // calculate C = cc
            let et = 0.25f64 * ((h + 2.0).cos() + 3.8);
            let t = (50000.0 / 13.0 * Ncb * vc.Nc * et * (ca * ca + cb * cb).sqrt()) / (rgb[0] + rgb[1] + 21.0 / 20.0 * rgb[2]);
            let cc = t.powf(0.9) * (jj / 100.).sqrt() * (1.64 - (0.29f64).powf(n)).powf(0.73);

            jch.push([jj, cc, h * 180.0/PI]);
        }

        CIECAM { vc, jch, xyz_w, obs: xyz.obs }
    }

    // Get J'a'b' coordinates
    pub fn jab_p(&self) -> Vec<[f64; 3]> {

        let mut jab_p_vec : Vec<[f64;3]> = Vec::with_capacity(self.jch.len());

        for [jj,cc,h] in &self.jch {
            let M = cc * self.vc.Fl().powf(0.25);
            let MPrime = 1.0 / 0.0228 * (1.0 + 0.0228 * M).ln();
            jab_p_vec.push(
                [
                    (1.0 + 100.0 * 0.007) * jj / (1.0 + 0.007 * jj),
                    MPrime * (h * PI / 180.0).cos(),
                    MPrime * (h * PI / 180.0).sin(),
                ]
            );
        }
        jab_p_vec
    }
}

pub fn m_cat02(xyz: [f64; 3]) -> [f64; 3] {
    [
        0.7328 * xyz[0] + 0.4296 * xyz[1] - 0.1624 * xyz[2],
        -0.7036 * xyz[0] + 1.6975 * xyz[1] + 0.0061 * xyz[2],
        0.0030 * xyz[0] + 0.0136 * xyz[1] + 0.9834 * xyz[2],
    ]
}

pub fn m_cat02_inv(rgb: [f64; 3]) -> [f64; 3] {
    [
        1.096124 * rgb[0] - 0.278869 * rgb[1] + 0.182745 * rgb[2],
        0.454369 * rgb[0] + 0.473533 * rgb[1] + 0.072098 * rgb[2],
        -0.009628 * rgb[0] - 0.005698 * rgb[1] + 1.015326 * rgb[2],
    ]
}

pub fn m_hpe(rgb: [f64; 3]) -> [f64; 3] {
    [
        0.38971 * rgb[0] + 0.68898 * rgb[1] - 0.07868 * rgb[2],
        -0.22981 * rgb[0] + 1.18340 * rgb[1] + 0.04641 * rgb[2],
        0.0 * rgb[0] +  0.0 * rgb[1] + 1.0 * rgb[2],
    ]
}

pub fn m_hpe_inv(rgb: [f64; 3]) -> [f64; 3] {
    [
        1.910197 * rgb[0]  - 1.112124 * rgb[1] + 0.201908 * rgb[2],
        0.370950 * rgb[0] + 0.629054 * rgb[1] - 0.000008 * rgb[2],
        0.0 * rgb[0] +  0.0 * rgb[1] + 1.0 * rgb[2],
    ]
}

pub fn achromatic_rsp(rgb: [f64; 3], Nbb: f64) -> f64 {
    (2.0 * rgb[0] + rgb[1] + rgb[2] / 20.0 - 0.305) * Nbb
}

#[test]
fn cie_cam_test() {
    use crate::cie1931::CIE1931;
    use crate::observers::ObserverKey;
    use crate::filter::grey_flt;
    use crate::sources::Spectra;
    use crate::physics::assert_vec_eq;
    use assert_approx_eq::assert_approx_eq;
    let xyz = XYZ::from(
        &Spectra::cie_d(5001.8, 1.0 / 1.8101228747229374),
        &grey_flt(380.0, 5.0, 81, vec![0.1841865]),
        &CIE1931,
    );

    // values from CIECAM02 spreadsheet
    let vcd = ViewConditions::default();
    assert_approx_eq!(vcd.D(), 0.940656466);
    assert_approx_eq!(vcd.k(), 0.00199601);
    assert_approx_eq!(vcd.Fl(), 0.79370053);
    let cam = CIECAM::from(xyz, ViewConditions::default());

    // CIECAM02 spreadsheet
    assert_approx_eq!(cam.jch[0][0], 39.74001389);
    assert_approx_eq!(cam.jch[0][1], 0.5660349);
    assert_approx_eq!(cam.jch[0][2], 112.403451);

//	println!("{:?}", cam);


    // http://www.cis.rit.edu/fairchild/files/CIECAM02.XLS
    let tst_xyz = XYZ {
        xyz_vec: vec![
            [28.86602045, 18.41865185, 2.677891918], // Lab 50, 50, 50
            [19.69905189, 18.41865185, 6.068714129], // 50, 10, 30
            [17.75961545, 18.41865185, 2.677891918], // 50, 0, 50
            [28.86602045, 18.41865185, 15.19916323], // 50, 50, 0
        //	[0., 0., 0.], // 0,0,0
            [15.95184479, 18.41865185, 45.32717872], // 50, -10, -50
            [54.65318174, 56.68129075, 103.2566216], // 80, 0, -50
        ],
        obs: ObserverKey::CIE1931,
        xyz_w: Some([96.421908, 100.0, 82.520498]),
    };

    let cam = CIECAM::from(tst_xyz, ViewConditions::default());

//	println!("{:?}", cam.jch);

    let want = vec![
        [41.749269, 68.836414, 38.720187],
        [40.138001, 27.410128, 69.268409],
        [39.662698, 43.578235, 93.629232],
        [41.935959, 55.425303, 1.766343],
    //	[0., 0., 180.],
        [38.787623, 60.277030, 240.725076],
        [73.036617, 54.776410, 254.404297],
    ];
    for i in 0..want.len() {
        assert_vec_eq(&cam.jch[i] ,&want[i], 1.0E-5);
    }
}

 */
