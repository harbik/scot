use nalgebra::{Matrix3xX, Matrix3};
use std::iter::repeat;
use std::ops::Range;
use rcs::differences::CieDE2000;
use rcs::illuminants::{D65};
use rcs::models::{CieCamJCh,  CieLab, LabValues, VcAvg, CieCamEnv, CieXYZ};
use rcs::observers::{CieObs1931, StandardObserver};

use svg::Document;

/// A CIECAM Hue gradients in CIELAB coordinates
/// 
pub fn lab_hue_gradient<V, I, C>(hues: Range<f64>, step: f64, lightness: f64, chroma: f64) -> CieLab<I,C>
where 
    V: Default + Into<CieCamEnv<I, C>>,
    I: Default + Into<CieXYZ<C>>,
    C: StandardObserver
{
    let jch_values: Vec<f64> = repeat(step).scan(hues.start,
    |h,step|{ 
        let t = *h;
        *h += step;
        if t>hues.end {
            None
        } else {
            Some([lightness, chroma, t])
        }
    }).flatten().collect();

    CieCamJCh::<V, I, C>::from(jch_values).into_cielab::<V,I>()
}

pub fn svg(lab: CieLab) {

    let srgb = Matrix3::<f64>::new(
        3.2404542, -1.5371385, -0.4985314,
        -0.9692660,  1.8760108,  0.0415560,
        0.0556434, -0.2040259,  1.0572252,
    );

    let xyz = CieXYZ::from(lab);
    let rgb = ((xyz.data.transpose()/100.0)*(srgb.transpose())).map(srgb_gamma)*255.0;

    let radius = 50;
    let gap= 5;

    let mut document = Document::new()
        .set("viewBox", (0, 0,  (xyz.len() + 1) * (2 * radius + gap), 2 * radius + gap))
        .set("style", "background-color:black;")
       ;
    let mut cx = 2* radius + gap;
    for c in rgb.row_iter() {
        let [r,g,b] = [c[(0,0)], c[(0,1)], c[(0,2)]];
        let rect = svg::node::element::Circle::new()
            .set("r", radius)
            .set("cy", radius + gap)
            .set("cx", cx)
            .set("fill", format!("rgb({:.0},{:.0},{:.0})", r, g, b))
            ;
        document = document.add(rect);
        cx += 2 * radius + gap;
        println!("{:.0} {:.0} {:.0}",r, g, b);
    }
    svg::save("image.svg", &document).unwrap();
    

}

pub fn srgb_gamma(v: f64) -> f64 {
    if v <= 0.0031308 {
        v*12.92
    } else {
        (1.055*v).powf(1.0/2.4) - 0.055
    }
}

pub fn main() {
    let lab: CieLab<D65, CieObs1931> = lab_hue_gradient::<VcAvg,_,_>(0.0..360.0, 5.0, 60.0, 48.0);

    let mut diffs = Vec::new();
    for (
        LabValues { l, a, b },
        LabValues { l: l1, a: a1, b: b1 }
    ) in lab.iter().zip(lab.iter().skip(1))
    {
        let dif = CieDE2000::<D65, CieObs1931>::new(
            CieLab::new(Matrix3xX::from_vec(vec![l, a, b])),
            CieLab::new(Matrix3xX::from_vec(vec![l1, a1, b1])),
        );
        println!("{:.2} {:.2} {:.2} {:.2}", l, a, b, dif.0[(0,0)]);
        diffs.push(dif.0[(0,0)]);
    }

    svg(lab);


}
