  
# scot: Rust Color Science Library

<div align="center">

![Colorful scot](daniel-norris.jpg)

</div>


scot is an open-source Color Science data and algorithms library written in the Rust programming language,
targeting applications such as color management and quality control for displays, graphic arts, and architectural lighting.
It implements methods and standards as defined by international standard organizations,
such as the **CIE**, the *International Commission on Illumination*,
and also includes many other color algorithms and datasets.

scot is free and open-source, and is released under dual MIT and Apache 2.0 licenses.
It is being developed by Gerard Harbers from *Harbers Bik LLC*.

Mathematical representations of spectral distributions are the base many of color algorithms in this library,
such spectral power distributions of lamps and displays, 
and spectral reflectivity and spectral transmissivity distributions of surfaces and transparent materials:
they are typically measured using spectrometers, or defined by international standards.
This library has a large collection of these spectral distribution, and makes it easy to process them with various color models,
or to create your own models.

## Features:
- Spectral power distributions library for a lamps and illuminants, such as fluorescent and LED lamps.
- Materials spectral reflectivity library.
- Spectral distributions from mathematical models such as Planck blackbody radiators, Gaussian spectra, and mathematical LED models.
- Spectral interpolation and spectral calculation using the Rust `nalgebra` package.
- Calculate tristimulus values from spectral data using a number of standard observers such as CIE1931 2º, CIE1964 10º, CIE 2015 2º and 10º.
- Calculate chromaticity coordinates and appearance correlates based on a number of color models.

## Disclaimer
The data, methods, and algorithms in this library, 
referencing Standard Organizations such as the International Commission on Illumination (CIE), or any other Standards Organizations, 
have not been endorsed, qualified, or approved by these Standard Organizations. 
Please consult their documentation and standards for authoritative methods, recommendations, and data. 
If you find any deviations or errors between the official standards and the implementations in this library, please report them as [issues](https://github.com/harbik/scot/issues) on its GitHub page.
Please be also aware that light and color measurements are difficult and depend on a lot of factors, 
with many of them outside the scope of this library and its applications. 
If you have a professional lighting, display lighting, or color issue, please consult a specialist.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

scot Photo (top) made by <a href="https://unsplash.com/@danielnorris">Daniel Norris</a> on <a href="https://unsplash.com/s/photos/scot">Unsplash</a>