  
# Colorado: Rust Color Science Library

<div align="center">

![Colorful Colorado](daniel-norris.jpg)

</div>


Colorado is an open-source Color Science library written in the Rust programming language,
 implementing recommended methods and standards as defined by for example the **CIE** 
 (International Commission on Illumination)
 the ICC (International Color Consortium),
 the IES (Illumination Engineering Society),
 and many others.
It intended use is for scientists, engineers, and any other (semi)professionals,
 working in the field of color imaging,
 graphic arts, architectural lighting,
 and color engineering in general.

Colorado is free and open-source, and is released under dual MIT and Apache 2.0 licenses.
It is being developed by Gerard Harbers from *Harbers Bik LLC*.

Why another color library? And why Rust?
There is already an excellent color library written in Python, as documented at <http:://colour-science.org>.
And, if you like to use MatLab, there is [ColorLab](https://www.uv.es/vista/vistavalencia/software/colorlab.html).

Colorado, written in Rust, has the following benefits:

- Compared to Python it is fast, with a performance comparable to C++ applications.
- The main reason -for me- is that Rust code can be directly compile to WebAssembly, 
   which means that all the algorithms will run in a Web browser,
   at near-native performance.
  Using this library,
   and other existing Rust libraries,
   complex color calculations can be performed in the browsers,
   without installing and frequently updating programs,
   on *any system* which runs a modern web browser.
- And if you're not interested in online color applications,
   Rust has excellent cross-platform development and support tools,
   allowing you to develop native applications for Linux, MacOS, Windows, and many other platforms.
- Rust has an amazing build tool,
  called cargo, and a large collection of Rust packages in the <http://crates.io> package registry,
  with a very good code documentation system.
  Using cargo, crates.io, and the Rust documentation system,
  it is very easy to re-use code written by others.
- Rust uses static typing,
   and has an excellent and strict compiler,
   which not only helps to avoid writing erroneous or unsafe code,
   but also allows you to use the compiler to write code for you.
  When I started writing Rust,
   I you got frustrated with the compiler initially,
   but quickly started to enjoy the experience that if a program compiles,
   if often runs as expected.


 

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
If you find any deviations or errors between the official standards and the implementations in this library, please report them as [issues](https://github.com/harbik/colorado/issues) on its GitHub page.
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

Colorado Photo (top) made by <a href="https://unsplash.com/@danielnorris">Daniel Norris</a> on <a href="https://unsplash.com/s/photos/colorado">Unsplash</a>