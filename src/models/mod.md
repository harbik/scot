# Color Models

*Color Models* are mathematical models to describe the appearance of a color,
 *using models of human vision as a basis, 
 as a set of typically three of four values, or color components, 
 which form a three or four dimensional *Color Space*.
All the models in this module are mathematical, 
 human vision based, 
 color models.

Currently, this library implements the following mathematical models:
- CIE XYZ Tristimulus values colorspace &amp; (x,y) chromaticity,
- [CIE 1960 UCS][crate::models::yuv1960], a.k.a UVW color space, represented by [CieUVW][crate::models::CieUVW] &amp;   [CieYuv1960][crate::models::CieYuv1960],
- CIE 1964 U<sup>\*</sup>V<sup>\*</sup>W<sup>*</sup> color space,
- [1976 CIELUV][crate::models::yuv] L<sup>\*</sup>u<sup>\*</sup>v<sup>\*</sup>, and [CieYuv][crate::models::CieYuv] chromaticity model,
- CIELAB L<sup>\*</sup>a<sup>\*</sup>b<sup>\*</sup> [CieLab][mod@crate::models::cielab] color space
- [CIECAM02][crate::models::ciecam02] Color Appearance Models [CieCam][crate::models::CieCam], [CieCamJCh][crate::models::CieCamJCh], and [CieCamUcs][crate::models::CieCamUcs].


A color model can be considered a *Color Order System* 
 with colors ordered along continuous valued axis, 
 representing an infinite number of appearances of color.

A *Color Naming System* is a color order system which does not use mathematical scales for it axis,
 or has no clear axis at all.

# Color Order Systems
A color order system is a systematic and rational method to arrange color samples,
 with the samples given a descriptive term or number.
Non-mathematical order systems typically use physical samples of colorants 
 (e.g.  pigments, or dyes), 
 applied to a carrier such a piece of paper, or a piece of textile, 
 and arrangements of these samples are collected in a *Color Atlas* or *Color Book*, 
 typically made available by dye manufacturers to their customers.
Samples are made by mixing multiple colorants or dyes, 
 with varying concentrations of each.
Examples of these are the *ICI Color Atlas* (1969), 
 containing 1379 original colors,
 and 27_580 variations, 
 all printed on paper.

# Color Naming Systems
When random color samples are ordered, 
 with the samples not related to each other by mixing fractions and colorant types, 
 or by comparing color samples of unknown composition, 
 a color order system is said to be a "Color Naming System". 
The ordering in these does not have a continuous scale.
An example of a "Color Naming System" is the Pantone system, 
which is popular in the graphic art and textile industry, 
and has dedicated color atlases for use with textiles,
plastics, architecture and interiors; 
in this system a six digit notation and a color name is used to specify a color.

# Color Spaces
For representations of color order systems see the *Swatches* section in this library.

Most common are three dimensional color models, with a color described by three
color components, such as the display RGB color spaces *sRGB*, and *AdobeRGB*, and
the colorimetric *CIE XYZ* and *CIE LAB* color spaces.
A color space has minimal thee dimensions, due to our eyes having three types of
photoreceptors: the cone cells, referred to the L, M, and S cone cells, with L
being the more long wavelength sensitive type, M the medium wavelength sensitive
type, and S the short wavelength sensitive cone cell type.

# Color Diagrams
Two dimensional representations of color spaces are used as *Color Diagrams* or
*Color Charts*: these are cross sections of a color space.
A good example is the CIE 1931 chromaticity diagram, where a cross
section through the CIE XYZ color spaces is made by using the *x+y+z=1* plane,
with chromaticity axis *x=X/(X+Y+Z)*,  *y=Y/(X+Y+Z)*, and *z=X/(X+Y+Z)*.
Another example of a category of color charts are the *Color Circles*, such as a
CIELAB diagram, with chromaticity coordinates a and b, with a fixed value of
L=50, represented as a circle or disk.

# Uniform Color Spaces (UCS)
Uniform Color Spaces, or UCS color spaces, are designed to be perceptual
uniform, with distances between two points in such a space representing the
actual perceived color difference, as observed with our eyes, independent of the
location of the two points in the space: for example, a perceived color
difference blueish and greenish point, with a certain distance apart, is same to
the perceived color difference between a reddish and purplish point.
An early example of a uniform color space is the CIE 1960 CIEUVW color space,
with its associated (u,v) chromaticity color diagram, and its newer version, the
CIE 1976 CIELUV color space, with its (u',v') color diagram.
Newer uniform color spaces are the CIELAB and CIECAM02-UCS color spaces.
Achieving uniformity in a color space is hard, as a color model needs to model
how colors are processed and appear in our mind: advanced models are called
*color appearance models*.

# CIECAM02 Color Appearance Model
The CIECAM02 color appearance model uses up to nine parameters, or correlates, to
describe color, such as Brightness, Lightness, Chroma, and Saturation.
These parameters don't form a multi-dimensional color space: they can be
considered to form different representation of the same three dimensional color
space, with different oriented axis, or base vectors, with different non-linear,
scale factors along the axis.
A color appearance can be uniquely represented in this model by a selection of
three independent correlates, such as Lightness (L), Chroma (C), and Hue Angle
(h),  or Lightness (L), Redness-Greenness (a), and Blueness-Yellowness (b).
A perceptually uniform color space is defined in this model using the Lightness, Colorfulness,
and Hue Angle correlates, and are represented by the symbols J', a' and b',
and is considered to be the most uniform color space developed so far.

# Overview

Every model in this library has it own type, with at least one type parameter,
the observer, as for example `CieXYZ<C>`, with C representing a color matching
functions associated with an observer, but depending on the model more than one
type, such as the illuminant for the types using a reference white, e.g.
`CieLab<I,C>`.
Color Appearance models use an additional view conditions type parameter,
e.g. `CieCamUcs<V,I,C>`, with `V` representing the viewing conditions.

As stated before, these strong typed model representation help to identify and
avoid incorrect color calculations, and allow the compiler to figure out how to
transform color coordinates: it lets the compiler do most of the work for you.

The library uses the `CieXYZ<C>` and `CieLab<I,C>` models as base representations for 
color.
For illuminants `CieXYZ<C>` is used, when transforming from spectral
distributions of sources of light, and for reflective (and transmissive) color
samples, and displays, the `CieLab<I,C>` model is used.
The other models in this library use either one or the other to calculate their
parameters.

## CIE XYZ types: `CieXYZ`, and `CieYxy`

## CIE UVW types: `CieUVW` and `CieYuv1960`

## CIE LUV types `CieLUV` and `CieYuv`

## CIE LAB and Hunter LAB types: `CieLab` and Hunter `HunterLab`

## CIECAM types: `CieCam`, `CieCamJCh`, and `CieCamUcs`.


