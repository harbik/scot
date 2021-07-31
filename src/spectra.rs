
extern crate nalgebra as na;

use na::DMatrix;

use crate::observers::StandardObserver;
use crate::cie::xyz::XYZ;
use crate::util::domain::{Domain};
use crate::util::units::{Meter, Scale, Unit};



pub trait Pixel {}
/// trait marker for display pixel spectra


/**
	Get spectral data from spectral models, standards, and spectral libraries, mapped to a spectral domain.

	Spectral data is represented by a `nalgebra` matrix, with the spectral distribution values as column vectors,
	allowing further processing using this libary's matrix calculation methods. T
*/

pub trait SpectralData {

	type ScaleType: Scale;
//	type UnitValue;

	/**
		Values for a set of spectral distributions.

		Returns values for any spectral representation of light, such as from a light source, or an illuminated surface,
		in form of an nalgebra's `DMatrix<f64>`, with  one or more spectral distribution data as columns. The values are
		mapped to a specified domain, typically by interpolation, or by evaluation of functions for functional
		representations.
	*/
	fn values<L>(&self, domain: Domain<L>) -> DMatrix<f64>
		where
			L: Scale,
			<<Self as SpectralData>::ScaleType as Scale>::UnitType: From<<L>::UnitType> 
			// need to be able to map the target domain onto the native domain of the spectral data,
			// or, in other words, need to be able to convert from the target domain's unit into the
			// object's domain's unit.
			; 

	/// spectral's native or default spectral range
	fn domain(&self) -> Domain<Self::ScaleType>; 

	/// Optional keys for each of the spectral distribution in the collection.
	fn keys(&self) -> Option<Vec<String>> { None }
		//  here implemented as a default method, to be overridden if applicable

	/// Optional description of spectral collection.
	fn description(&self) -> Option<String> { None }

}

/**
	Calculate XYZ tristimilus value from spectral distributions.

	This is a generic implementation for calculation of XYZ values. 
	It interpolates the color matching functions values onto the 
	spectral distribution's domain.

	# Examples
	Calculate Tristimulus values for a Blackbody radiator
	```
	use colorado::illuminants::Blackbody;
	use colorado::observers::Cie1931;
	use colorado::cie::XYZ;

	let bb = XYZ::<Cie1931>::from(Blackbody::new(3000));
	println!("{}",bb);
	```
 */
impl<C, S> From<S> for XYZ<C>
where 
	C: StandardObserver,
	&'static C: Default,
	S: SpectralData,
	Meter: From<<<S as SpectralData>::ScaleType as Scale>::UnitType>

 {
	fn from(sd: S) -> Self {
		let xyz = <&C>::default().cmf(sd.domain()) * sd.values(sd.domain()) * C::K * sd.domain().scale.unit(1).value();
		XYZ::<C>::new(xyz)
	}
}
