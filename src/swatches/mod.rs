/*!
Spectral distributions of collections of color samples, or color swatches. 

Swatches are typically used to specify, or check, the color of products, and objects, such as the color of walls and fabrics.
To see their color, they need to be illuminated, typically ambient light, such as daylight, or artificial light, provided by 
LED or fluorescent lights. 
To get a good impression of the color of a swatch, and the object to be checked, `good` white ambient light is needed. 

The collections in this library represent spectral reflectivity distributions, normalized to a peak reflectivity of 1.0.
They all implement the `SpectralData`, and `Swatch` traits.

Currently, this library has the following collections:

- X-Rites's ColorChecker chart samples, measured by BabelColor's Danny Pascale, averaged over 30 sample charts.


*/

use crate::spectra::SpectralData;
pub trait Swatches: SpectralData + Default {}
/// trait marker for swatch reflection spectra, 
/// such as the Munsell color swatches.

pub mod checker;