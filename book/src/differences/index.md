
# Color Difference Formulas

In color science a perceived color difference between two samples is expressed by a single number,
with dimensionless unit &Delta;E, and related to a just-noticeable difference JND by:

\\[  1 \Delta E \approx 2.3\ \text{JND}  \\]

A just noticeable difference is the amount a physiological stimulus - a display pixel, a color swatch – has to change to
be noticed by an observer; a difference half of the observers see, the other half not.
And a color difference of \\( 1 \Delta E \\) can be interpreted as a significant color difference, detectable by 99.8% of observers with normal color vision.

The unit &Delta;E  was introduced by the CIE in 1976 with the CIE &Delta;E<sup>\*</sup><sub>76</sub> definition of color difference, 
defined as the distance between two points in the CIE \\( L^\*a^\*b\^*\\) (CIELAB) color space;
this color space was designed to be perceptually uniform, so that a distance between two points in a space represents
the same magnitude of color difference perception independent of their location.

Color differences are typically defined as function of color space coordinates, which
in turn are based on a standard observer's spectral sensitivities, represented by its color matching functions.

Over time, other color difference formulas have been defined, better representing color matching experiments,
either based on the same color spaces, or new color spaces, such as the CIECAM color space.

scot currently implements the following color difference metrics:

- CIE \\( \Delta E_{1976}\\)
- CIE \\( \Delta E_{1994}\\)
- CIE \\( \Delta E_{2000}\\)
- \\( \Delta E_{\text{cam02}}\\)

# Examples

The color differences in this library: `CieDE1976`, `CieDE1994`,`CieDE2000`, and `DECam02`,
all have two type parameters, `I` and `C`, e.g. `CieDE1976<I,C>`, with `I` the illuminant, and `C` the Observer being used.
When comparing colors on a display, the illuminant is the display white source, as a spectral distribution.

Color differences are calculated from a reference and a test set of color samples, using a two argument `new` constructor, or a `from` method with a two type arguments tuple:

Using a `new` constructor:

```rust
    use scot::observers::Cie1931;
    use scot::illuminants::CieD65;
    use scot::swatches::{ColorCheckerSwatch, Ces};

    let de = CieDE1976::<Cie1931, CieD65>::new(ColorCheckerSwatch::<13>, Ces);

    println!("{:.1}", de.0);
```
and a `from` pattern:

```rust
    use scot::observers::Cie1931;
    use scot::illuminants::CieD65;
    use scot::swatches::{ColorCheckerSwatch, Ces};

    let de = CieDE1976::<Cie1931, CieD65>::new(ColorCheckerSwatch::<13>, Ces);

    println!("{:.1}", de.0);
```