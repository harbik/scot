  

<div align="center">
    <img src="https://www.harbik.com/img/daniel-norris.jpg" alt="Colorful rcs" width="500"/>
</div>


# rcs-Charts

rcs-Charts part of the Rust rcs Color Science Library,
 and contains data collections for color *test charts*,
 which are used for characterizing or calibrating input devices such as digital cameras and scanners.
These charts contain many different color samples with known target color coordinates (typically expressed in `CieLab<D50, CieObs1931>` coordinates),
 which have been manufactured according to target's color specification,
 or which have been measured,
 for example by the manufacturer, 
 or by yourself with a spectrometer or colorimeter.
The colorimetric values for the patches on the test target are typically supplied in form of a *reference color* file,
 in either the CGATS file format, 
 or CxF, XRite's color exchange format.

To calibrate a camera or scanner, 
 you take an image of it,
 or scan it with a scanner,
 and the color differences between the patches in the image and the colorimetric reference values are used to create a new set of color transformation values,
 typically stored in an ICC color profile.
 
There are many different type of test charts,
 with different sizes,
 different number of patches,
 and different composition of patches:
 some are real paint samples, composed of mixtures of different pigments,
 and some others are printed.
From a color science perspective,
 samples which are chosen to match spectral distributions real world materials most closely are preferred,
 to avoid metameric errors,
 but,
 at the same time,
 a large number of samples is better than a small number of samples,
 which is much harder with using real paint samples.

# Features
In this crate you will find a collection of spectrometric and colorimetric datasets, algorithms, and tools, 
 for use with color test charts.

- Curated set of colorimetric and spectrometric reference values, from various sources,
- &hellip;

# X-Rite&reg; ColorChecker&reg;

The first, and most famous, 
 color-rendition chart was introduced by C.S. McCamy at all. from the Macbeth company,
 now part of X-Rite, as described in his paper [A Color-Rendition Chart][McCamy].
This chart is still used today, and is sold under the "ColorChecker&reg; Classic" product name, 
 with "ColorChecker" being a registered trademark of [X-Rite][XRite].

## Classic Series
X-Rite&reg; ColorChecker&reg; Classic series consist  of an array of 4x6 square matte paint swatches,
 with the same color as introduced by McCamy,
 and come in all kind of different sizes:

- original, 
- XL,
- Mega
- Mini
- Nano

This crate contains the following data collections for the ColorChecker Family of charts:

- [Ohta][ohta] dataset, from N. Ohta, "The Basis of Color Reproduction Engineering (Japanese)", published by Corona-sha Co of Japan in 1997,
- [BabelColor&reg;][babel] Average spectral reflectivity for X-rite's ColorChecker&reg;,
as measured by BabelColor's Danny Pascale.

# IT8.7 (ISO 12641) Charts

From [Wikipedia](https://en.wikipedia.org/wiki/IT8):


- IT8.7/1 - 1993 (R2003) - Graphic technology - Color transmission target for input scanner calibration  

  This standard defines an input test target that will allow any color input scanner to be calibrated with any film dye
  set used to create the target. It is intended to address the color transparency products that are generally used for
  input to the preparatory process for printing and publishing. This standard defines the layout and colorimetric values
  of a target that can be manufactured on any positive color transparency film and that is intended for use in the
  calibration of a photographic film/scanner combination

- IT8.7/2 - 1993 (R2003) Graphic technology - Color reflection target for input scanner calibration 

  This standard defines an input test target that will allow any color input scanner to be calibrated with any film dye
  set used to create the target. It is intended to address the color photographic paper products that are generally used
  for input to the preparatory process for printing and publishing. It defines the layout and colorimetric values of the
  target that can be manufactured on any color photographic paper and is intended for use in the calibration of a
  photographic paper/scanner combination
 
- IT8.7/3 - 1993 (R2003) Graphic technology - Input data for characterization of 4-color process printing 

  The purpose of this standard is to specify an input data file, a measurement procedure and an output data format to
  characterize any four-color printing process. The output data (characterization) file should be transferred with any
  four-color (cyan, magenta, yellow and black) halftone image files to enable a color transformation to be undertaken when
  required

[McCamy]: https://home.cis.rit.edu/~cnspci/references/mccamy1976.pdf "C.S. McCamy, H. Marcus, J.G. Davidson, “A Color-Rendition Chart,” J. Appl. Phot. Eng., Vol. 2, No. 3, Summer 1976, pp. 95-99, Society of Photographic Scientists and Engineers"
[XRite]: https://www.xrite.com/categories/calibration-profiling/colorchecker-classic
 

# rcs: Rust Color Science Library

rcs is an open-source Color Science data and algorithms library written in the Rust programming language,
targeting applications such as color management and quality control for displays, graphic arts, and architectural lighting.
It implements methods and standards as defined by international standard organizations,
such as the **CIE**, the *International Commission on Illumination*,
and also includes many other color algorithms and datasets.

rcs is free and open-source, and is released under dual MIT and Apache 2.0 licenses.
It is being developed by Gerard Harbers from *Harbers Bik LLC*.

Mathematical representations of spectral distributions are the base many of color algorithms in this library,
such spectral power distributions of lamps and displays, 
and spectral reflectivity and spectral transmissivity distributions of surfaces and transparent materials:
they are typically measured using spectrometers, or defined by international standards.
This library has a large collection of these spectral distribution, and makes it easy to process them with various color models,
or to create your own models.


# Disclaimer
The data, methods, and algorithms in this library, 
 referencing Standard Organizations such as the International Commission on Illumination (CIE), or any other Standards Organization, 
 have not been endorsed, qualified, or approved by these Standard Organizations. 
Please consult their documentation and standards for authoritative methods, recommendations, and data. 
If you find any deviations or errors between the official standards and the implementations in this library, 
 please report them as [issues](https://github.com/harbik/rcs/issues) on its GitHub page.
Please be also aware that light and color measurements are difficult and depend on many factors, 
 with many of them outside the scope of this library and its applications. 
If you have a professional lighting, display lighting, or color issue, please consult a specialist.


# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

rcs Photo (top) made by <a href="https://unsplash.com/@danielnorris">Daniel Norris</a> on <a href="https://unsplash.com/s/photos/rcs">Unsplash</a>
