  
<div align="center" style="margin:15mm;">
    <img src="https://www.harbik.com/img/daniel-norris.jpg" alt="Colorful Colorado" width="500"/>
</div>

# Colorado: Rust Color Science Library

Colorado is an open-source Color Science library written in the Rust programming language,
 implementing recommended methods and standards such as defined by the *CIE* 
 (International Commission on Illumination),
 the *ICC* (International Color Consortium), and 
 the *IES* (Illumination Engineering Society),
 and is intended for color engineers and semi-professionals,
 interested in using advanced color models in lighting design, 
 digital imaging, graphic arts, lighting, and product design.

For example,
 besides the standard color models (CIELAB, CIE XYZ), it implements the advanced color appearance CIECAM02 model,
 (and its associated color difference metrics), 
 and for color rendering evaluation of light sources,
 it implements calculation of all the IES TM30-20 (R<sub>f</sub>, R<sub>g</sub>) color rendition metrics,
 besides the older CIE CRI (R<sub>a</sub>) metrics.

It also support the use of the new *cone fundamentals* based standard observers for colorimetric calculations.

Colorado is free and open-source, and is released under dual MIT and Apache 2.0 licenses.
It is being developed by Gerard Harbers from *Harbers Bik LLC*.

Why another color library? And why Rust?
There are already many other excellent color libraries written in Python, such as <http:://colour-science.org>.
And, if you like to use MatLab, there is [ColorLab](https://www.uv.es/vista/vistavalencia/software/colorlab.html).

Colorado, written in Rust, has the following benefits:

- Compared to Python it is fast, with a performance comparable to C++ applications:
 it is a system programming language designed for memory safety and speed.
- Rust code can be directly compiled to WebAssembly, 
   which means that all the algorithms will run in a Web browser,
   at near-native performance.
  Using this library,
   and other existing Rust libraries,
   complex color calculations can be performed
   –without installing and frequently updating programs–
   on *any system* which runs a modern web browser.
- And –if you're not interested in online applications–
   Rust has excellent cross-platform development and support tools,
   allowing you to write –single code base– native applications for Linux, MacOS, Windows, and many other platforms.
- Or, if you like a local scripting type development environment, such as in Python, there is [Deno](https://deno.land),
   a JavaScript/TypeScript platform, in which you can import Colorado generated web assembly modules.
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

Colorado is a **spectral color** library,
 which means that –whenever possible– 
 color stimuli are represented by spectral power distributions.
Which –and that is another big benefit– 
 enables correcting and representing color for non-standard,
 individual, observers;
 it is time to retire the -good old, but failing- CIE 1931 observer!
For this purpose the library has a large collection of spectral data collections,
 and spectral composition algorithms,
 and a set of different colorimetric observers, and colorimetric observer composition tools,
 mostly build on top of the CIE cone fundamentals.

To work with the large amount of spectrometric and colorimetric data,
 the Rust [`nalgebra`](https://nalgebra.org) linear algebra library is used extensively;
 almost any component in the library is a wrapper for a general matrix type.
If you only want to use the library at high level,
 you won't need to get familiar with this,
 but if you are interested to start working with the data itself,
 it allows you to crunch-the-numbers with a few lines of code, in the Matlab-style.


## Examples

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
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>?)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Colorado Photo (top) made by <a href="https://unsplash.com/@danielnorris">Daniel Norris</a> on <a href="https://unsplash.com/s/photos/colorado">Unsplash</a>