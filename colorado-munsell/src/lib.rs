/*!

Munsell Color System

From [Paul Centore]'s article:

> Albert Munsell originally defined the Munsell system conceptually. A colour is specified by its hue, value, and chroma. Hue is notated by a number between 0 and 10,
> which prefixes one of ten hue names: red (R), yellow-red (YR), yellow (Y), greenyellow (GY), green (G), blue-green (BG), blue (B), purple-blue (PB), purple (P),
> and red-purple (RP). There are a total of 100 hues with integer prefixes. Value, indicating how light a colour is, is a number between 0 (signifying black) and 10 (white).
> Chroma extends from 0 (grey) to a positive number, which increases to a varying
> perceptual limit as a colour’s difference from a grey, of the same Munsell hue and
> value, increases. The 100 hues with integer prefixes are evenly spaced perceptually,
> as are values and chromas. In addition to Munsell’s abstract definition, the 1929
> Munsell Book of Color contained physical exemplifications of Munsell specifications.
> This book became a physical standard for the system.

[Paul Centore]:  https://www.munsellcolourscienceforpainters.com/ColourSciencePapers/OpenSourceInverseRenotationArticle.pdf "An Open-Source Inversion Algorithm for the Munsell Renotation"

 */


 pub mod gloss;
 pub use gloss::*;

 pub mod matt;
 pub use matt::*;

 pub mod renotation;
 pub use renotation::*;