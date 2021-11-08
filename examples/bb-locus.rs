use colorado::models::{CieYuv1960, };
use colorado::observers::{CieObs1931, };
use colorado::illuminants::{Planckian, };

fn main() -> Result<(), Box< dyn std::error::Error>> {

    let mireds: Vec<f64> = (1..=2000).into_iter().map(|v|v as f64).collect();
    let ts : Vec<f64> = mireds.iter().map(|&v|1E6/v as f64).collect();
    let bb_locus = CieYuv1960::<CieObs1931>::from(Planckian::new(ts.clone()));
    for (&m,yuv) in mireds.iter().zip(bb_locus.data.column_iter()) {
        let [_,u,v]: &[f64;3] =  yuv.as_ref();
        println!("{},{},{}", m, u, v);
    }
    Ok(())
}