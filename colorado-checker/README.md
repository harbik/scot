  

<div align="center">
    <img src="https://www.harbik.com/img/daniel-norris.jpg" alt="Colorful Colorado" width="500"/>
</div>


# Colorado-Checker

Colorado-Checker is part of the Rust Colorado Color Science Library,
 and contains spectral data collections for color-rendition charts, 
 which are used for image color correction of digital images.

The first, and most famous, 
 color-rendition chart was introduced by C.S. McCamy at all. from the Macbeth company,
 now part of X-Rite, as described in his paper [A Color-Rendition Chart][McCamy].
This chart is still used today, and is sold under the "ColorChecker&reg; Classic" product name, 
 with "ColorChecker" being a registered trademark of [X-Rite][XRite].
It has an array of 4x6 square matte paint swatches,
 applied to smooth paper and glued to a rigid support.
Each swatch has  a square size of 50mm by 50mm,
 and represents for example light and dark human skin, foliage, and the blue sky.

Besides X-Rite's ColorChecker,
 other color-rendition charts are available on the market now,
 but please be aware that for best results the swatches should come with a color specification,
 in form of CIELAB coordinates for example,
 and that the swatches are a good spectral simulation of the objects they represent,
 to avoid metameric errors.

This crate contains the following data collections:

- [Ohta][ohta] dataset, from N. Ohta, "The Basis of Color Reproduction Engineering (Japanese)", published by Corona-sha Co of Japan in 1997,
- [BabelColor&reg;][babel] Average spectral reflectivity for X-rite's ColorChecker&reg;,
as measured by BabelColor's Danny Pascale.

 

# Colorado: Rust Color Science Library

Colorado is an open-source Color Science data and algorithms library written in the Rust programming language,
targeting applications such as color management and quality control for displays, graphic arts, and architectural lighting.
It implements methods and standards as defined by international standard organizations,
such as the **CIE**, the *International Commission on Illumination*,
and also includes many other color algorithms and datasets.

Colorado is free and open-source, and is released under dual MIT and Apache 2.0 licenses.
It is being developed by Gerard Harbers from *Harbers Bik LLC*.

Mathematical representations of spectral distributions are the base many of color algorithms in this library,
such spectral power distributions of lamps and displays, 
and spectral reflectivity and spectral transmissivity distributions of surfaces and transparent materials:
they are typically measured using spectrometers, or defined by international standards.
This library has a large collection of these spectral distribution, and makes it easy to process them with various color models,
or to create your own models.


# Disclaimer
The data, methods, and algorithms in this library, 
referencing Standard Organizations such as the International Commission on Illumination (CIE), or any other Standards Organizations, 
have not been endorsed, qualified, or approved by these Standard Organizations. 
Please consult their documentation and standards for authoritative methods, recommendations, and data. 
If you find any deviations or errors between the official standards and the implementations in this library, please report them as [issues](https://github.com/harbik/colorado/issues) on its GitHub page.
Please be also aware that light and color measurements are difficult and depend on a lot of factors, 
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

Colorado Photo (top) made by <a href="https://unsplash.com/@danielnorris">Daniel Norris</a> on <a href="https://unsplash.com/s/photos/colorado">Unsplash</a>

[McCamy]: https://home.cis.rit.edu/~cnspci/references/mccamy1976.pdf "C.S. McCamy, H. Marcus, J.G. Davidson, “A Color-Rendition Chart,” J. Appl. Phot. Eng., Vol. 2, No. 3, Summer 1976, pp. 95-99, Society of Photographic Scientists and Engineers"
[XRite]: https://www.xrite.com/categories/calibration-profiling/colorchecker-classic