# Introduction

This is a user guide for an open-source Color Science library, called **SCOT**, written in the Rust programming language;
 it implements recommended methods and standards such as defined by the *CIE* (International Commission on Illumination),
 the *ICC* (International Color Consortium), 
 and the *IES* (Illumination Engineering Society);
 and is intended for color engineers and semi-professionals,
 interested in using advanced color models in lighting design, digital imaging, graphic arts, lighting, and product design.

It also contains a more general background of Color Science, important to understand SCoT's concepts and types.

A description of the `scot` command line tool is provided too, as well of a (simplified) web-assembly library *scot-wasm* which can
be used from scripting languages such as [Deno](https://deno.land), and in a web browser.
these have limited functionality though, providing only limited high level functions.

Why another color library? And why Rust?
There are already many other excellent color libraries written in Python, such as <http:://colour-science.org>.
And, if you like to use MatLab, there is [ColorLab](https://www.uv.es/vista/vistavalencia/software/colorlab.html).

**SCOT**, written in Rust, has the following benefits:

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
   a JavaScript/TypeScript platform, in which you can import **SCoT** generated web assembly modules.
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

**SCOT** is a **spectral color** library,
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


