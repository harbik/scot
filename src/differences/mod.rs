pub mod de1976;

//use crate::{illuminants::Illuminant, observers::StandardObserver, spectra::SpectralData, swatches::Swatches, util::units::{Meter, Scale}};


/*
pub trait DeltaE<C, I> 	
where
	C: StandardObserver,
	I: Illuminant,
{

	fn new<S1, S2>(s1: S1 , s2: S2) -> Vec<f64>
	where 
		S1: Swatches,
		S2: Swatches,
		<<S1 as SpectralData>::ScaleType as Scale>::UnitType: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>, 
		<<S2 as SpectralData>::ScaleType as Scale>::UnitType: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>, 
		Meter: From<<<I as SpectralData>::ScaleType as Scale>::UnitType>
	
	;

}
 */