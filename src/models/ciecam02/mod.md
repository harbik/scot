# CIECAM02 Color Model

This is a complex model, 
 and –from a color science point of view– 
 so far the most advanced  model to describe the way we perceive color in relationship with its environment,
 or *viewing conditions*.
It was officially published by the [CIE][CIECAM02] in 2002, 
 as "A Color Appearance Model For Color Management Systems", 
 replacing the old CIECAM97 model, 
 which is described by the commission as an interim color appearance model.
Although the title indicates that the model is intended for color management systems, 
 it is also used in the characterization of lamps, 
 in the *IES Method for Evaluating Light Source Color Rendition*, [ANSI/IES TM30-20][TM30-20].

The model consists of a *chromatic adaptation transform*, CIECAT02, 
 and methods to calculate mathematic correlates for the color appearance dimensions:
 - brightness (Q), 
 - lightness (L), 
 - colorfulness (M), 
 - chroma (C), 
 - saturation (s), 
 - and hue (h and H).

In addition, similar to the CIELAB model, CIECAM also defines correlates for:
 - redness-greenness (a),
 - blueness-yellowness (b).

Input parameters are:
 - relative tristimulus values for the *stimulus*, the target, for example a pixel, or a printed color patch (XYZ),
 - tristimulus values for the adapting white point (XYZ<sub>W</sub>
   normalized to Y<sub>W</sub>=100.0),
 - absolute luminance of the adapting field (L<sub>A</sub>, in units of cd·m<sup>-2</sup>, measured, or set to 20% of the absolute luminance of the reference white L<sub>W</sub>),
 - average relative background luminance (Y<sub>b</sub>, typically approximated to a value of 20.0, but better to use real input data),
 - surround ratio (S<sub>R</sub>), with three derived parameters (F,c, and N<sub>c</sub>),
 - and a degree of adaptation (D), which can be set as input, or can be estimated from the surround ratio.

In the model the target area is assumed to have an angular extend of about 2º.

The background is an area 10º around the target area,
 with color stimuli which are considered to be related from a color appearance point of view. 

The surround is everything beyond the background.

A degree of adaption of 0.0 implies no adaptation,
 while a value of 1.0 adaptation is fully accounted for.
CIECAM02 uses the CIECAT02 chromatic adaptation method.



# Color Definitions

A good overview of color appearance can be found in 
 ["Color Appearance Models" by Mark D. Fairchild][FAIRCHILD2013] and 
 ["CIECAM02 and Its Recent Developments" by Luo et all][CIECAM02LUO].

Color is a familiar concept, used frequently in daily life; but what is color fundamentally?

There is color as is measured with a colorimeter, or as an RGB value in an image, and defined through physically measurable quantities: *mathematical correlates*.
Such a characterization of color is also referred to as an *unrelated* color, as perceived in isolation from other colors,
or as *instrumental* color, a color as exists in instruments, cameras, and displays.

And there is color as we see, in our mind, which not always corresponds to the colors as measured, or set as pixel values; 
the colors we experience are defined in qualitative ways, using *perceptual attributes*.
Deviations between perceptual attributes and instrumental color correlates are due to the presence of other, neighboring colors, 
and perception affecting colors are called *related colors* too.


An image of an outdoor scene may contain shadowy areas, and sunny areas:
color within each or these areas are considered to be related.
Color appearance models try to predict how we see a set of related colors, in form of perceptual attributes such as lightness and chroma.

When looking at a single pixel in this image, ignoring other colors, standard –and much simpler– colorimetric color models can be used.


# Perceptual Attributes of Related Colors


**Hue** is defined as:
> attribute of a visual perception according to which an area appears to be similar to one of the colors red, yellow, green, and blue, 
> or to a combination of adjacent pairs of these colors considered in a closed ring

If asked to describe a color,
 using a single property,
 hue is the first aspect of color we think of.
And many associate color with the colors of a rainbow,
 or the colors in the sun-light, 
 dispersed through a prism,
 and with those we see mainly see variations in hue.

**Brightness**
> attribute of a visual perception according to which an area appears to emit, transmit or reflect, more or less light

This perceptual attribute is related to the illuminance –the photometric quantity, measured in lux–
 of an area,
 and has a half open-ended scale,
 starting at zero.
This attribute represents the experience that a colored area in a shadow appears less bright than the same area directly lit by the sun.

**Lightness**
> brightness of an area judged relative to the brightness of a similarly illuminated area that appears to be white or highly transmitting

This is a relative attribute,
 representing the color experience that when asked to compare the color of an object in terms of light and dark,
 you would say both have the same lightness (or darkness, if you will),
 independent if located in a shadowed region or in the bright sun.

**Colorfulness**
> attribute of a visual perception according to which the perceived color of an area appears to be more or less chromatic

This, similar to brightness,
 is an absolute perceptual attribute with an open-high-ended scale, 
 starting at zero
 –having no color or gray–,
 and with the colorfulness increasing with luminance at the other end.
An area appears to be very colorful if its luminance is high compared to its related neighbors.
A colored area illuminated under bright sunlight is more colorful than illuminated under moonlight.

**Chroma**
> colorfulness of an area judged as a proportion of the brightness of a similarly illuminated area that appears grey, white or highly transmitting

This is a relative perceptual attribute with a open-high-ended scale.
You can think of chroma as being "relative colorfulness" similar to lightness being "relative brightness".
Chroma is -mostly- independent of the magnitude of its illuminant
–or average luminance of its related neighbors–
but will change if the illuminant's chromaticity changes.

**Saturation**
> colorfulness of an area judged in proportion to its brightness

Like chroma, saturation can be considered "relative colorfulness",
 but as opposed to chroma, where the illuminant is used as reference,
 its own brightness is used as reference.
In CIECAM02 it is also a high-open-ended scale.
Chroma and saturation are very similar,
 and one could question if defining both attributes is really needed:
 both are derived from colorfulness.
However, both are used frequently:
 saturation is more fundamental for the description of color appearance,
 while chroma is used with color difference metrics.


# CIECAM02 Correlates

CIECAM02 is a quantitative color model, 
 which defines mathematical quantities (correlates) that correlate with the perceptual attributes of color.

This library implements the formulas and calculations steps as described by [Luo][CIECAM02LUO], 
 appendix Part 1 for the forward transform
 (XYZ to CIECAM),
 and appendix Part 2 for the reverse transform
 (CIECAM to XYZ);
 however, the library uses CIELAB model parameters as input for the forward transform and output for the reverse transform.



[TM30-20]: https://store.ies.org/product/tm-30-20-ies-method-for-evaluating-light-source-color-rendition/ "IES Method for Evaluating Light Source Color Rendition"
[CIECAM02]: https://cie.co.at/publications/colour-appearance-model-colour-management-systems-ciecam02 "A Colour Appearance Model For Colour Management Systems: CIECAM02, CIE 159:2004, ISBN: 978-3-901906-29-9"
[FAIRCHILD2013]: https://www.wiley.com/en-us/Color+Appearance+Models%2C+3rd+Edition-p-9781119967033 "Color Appearance Models, 3rd Edition, Mark D. Fairchild, ISBN: 978-1-119-96703-3"
[CIECAM02LUO]: https://link.springer.com/chapter/10.1007/978-1-4419-6190-7_2 "C. Fernandez-Maloigne (ed.), Advanced Color Image Processing and Analysis,  DOI 10.1007/978-1-4419-6190-7 2, Springer Science+Business Media New York 2013"
[EILV]: https://cie.co.at/e-ilv "ILV: International Lighting Vocabulary,2nd Edition, CIE S 017/E:2020"