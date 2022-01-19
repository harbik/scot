  
# scot Munsell Library

<div align="center">

![Colorful scot](daniel-norris.jpg)

</div>


scot is an open-source Color Science library written in the Rust programming language.
It is free and open-source, and released under dual MIT and Apache 2.0 licenses.
It is being developed by Gerard Harbers from *Harbers Bik LLC*.

**Albert Munsell** created a systematic catalogue of paint colors, 
in form of a color book with color samples ordered and characterized by hue, value and chroma.
scot-Munsell contains spectral representations of these, derived from a dataset measured by XXX.

**Hue** is defined by a number, varying between 0 and 10, and 10 hue names: 
red (R), yellow-red (YR), yellow (Y), green-yellow (GY), green (G), blue-green (BG), blue (B), purple-blue (PB), purple (P), and red-purple (RP). 
Hue numbers are mostly, but not limited to, integer values.

**Value** represents lightness of a color patch, with 0 being pure black (fully absorbing), and 10 pure white (perfect reflecting). 

**Chroma** defines the purity of a color, with the zero value being grey, and high values very pure and bright colors:
there is no fixed upper bound, and chroma values can be well over 10.

The scot-Munsell library can be used to generate spectral reflectivities, closely matching the Munsell book color samples.
Input can be any valid combination of Hue/Value/Chroma values, or, more common, any valid CieLab value. 


# Examples


```
    
```




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