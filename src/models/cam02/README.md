# CIECAM02 Color Model

This is a complex model, with many complex input and output parameters.
Below is a summary of them; for a more comprehensive overview see ["Color Appearance Models" by Mark D. Fairchild][FAIRCHILD2013] and ["CIECAM02 and Its Recent Developments" by Luo et all.][CIECAM02LUO]

The output of the model are six color appearance attributes, or color correlates, described by Luo as follows:

<blockquote>

- Brightness (Q)

  This is a visual perception according to which an area appears to exhibit more or less light. This is an open-ended scale with a zero origin defining the black.
  The brightness of a sample is affected by the luminance of the light source used. A surface color illuminated by a higher luminance would appear brighter than the same surface illuminated by a lower luminance. \[...\]
  Brightness is an absolute quantity, for example, a color appears much brighter when it is viewed under bright outdoor sunlight than under moonlight. Hence, their Q values could be largely different

- Lightness (J)

  This is the brightness of an area judged relative to the brightness of a similarly illuminated reference white.
  It is a relative quantity, for example, thinking a saturated red color printed onto a paper. The paper is defined as reference white having a lightness of 100. By comparing the light reflected from both surfaces in the bright sunlight, the red has a lightness of about 40% of the reference white (J value of 40). When assessing the lightness of the same red color under the moonlight against the same reference white paper, the lightness remains more or less the same with a J of 40.
  It can be expressed by J = Q<sub>S</sub>/Q<sub>W</sub>, where Q<sub>S</sub> and Q<sub>W</sub> are the brightness values for the sample and reference white, respectively.

- Colorfulness (M)

  Colorfulness is that attribute of a visual sensation according to which an area appears to exhibit more or less chromatic content.
  This is an open-ended scale with a zero origin defining the neutral colors. Similar to the brightness attribute, the colorfulness of a sample is also affected by luminance. An object illuminated under bright sunlight would appear more colorful than when viewed under moonlight, such as M value changes from 2000 to 1 with a ratio of 2000.

- Chroma (C)

  This is the colorfulness of an area judged as a proportion of the brightness of a similarly illuminated reference
  white. This is an open-ended scale with a zero origin representing neutral colors. It can be expressed by C = M/Q<sub>W</sub>.
  The same example is given here, a saturated red printed on a white paper. It has a Rs of 50 against the white paper having a brightness of 250 when viewed under sunlight. When viewed under dim light, Rs reduces to 25 and brightness of paper also reduces to half. Hence, the C value remains unchanged.

- Saturation (S)

  This is the colorfulness of an area judged in proportion to its brightness as expressed by s = M/Q, or s = C/J. This scale runs from zero, representing neutral colors, with an open end.
  Taking Figs. 2.3–2.5 as an example, the green grass under sunlight is bright and colorful. In contrast, those under the tree appear dark and less colorful. Because they are the same grass in the field, we know that they have the same color, but their brightness and colorfulness values are largely different. However, their saturation values will be very close because it is the ratio between brightness and colorfulness. Similar example can also be found in the image on the brick wall. Hence, saturation could be a good measure for detecting the number and size of objects in an image.

- Hue (h  and H)

  Hue is the attribute of a visual sensation according to which an area appears to be similar to one, or to proportions of two, of the perceived colors red, yellow, green and blue.
  CIECAM02 predicts hue with two measures: hue angle (h) ranging from 0º to 360º, and hue composition (H) ranging from 0, through 100, 200, 300, to 400 corresponding to the psychological hues of red, yellow, green, blue and back to red. These four hues are the psychological hues, which cannot be described in terms of any combinations of the other color names. All other hues can be described as a mixture of them. For example, an orange color should be described as mixtures of red and yellow, such as 60% of red and 40% of yellow.

</blockquote>

Similar to CIELAB, also here two additional correlates are defined:

- Redness-Greenness (a)

- Yellowness-Blueness (b)


# Viewing Conditions Input Parameters

CIECAM02 takes into account the viewing conditions of the model target.
To define these viewing conditions, the following terms are used:

- A "Stimulus", or "Target", is the element for which a CIECAM02 values is determined. This can take different forms: it can be for example a color swatch, or color sample of a material, which is in the model assumed to have a 2º angular extend, and assumed to be uniform. For a display, it is a bit more confusing to defined the target: is it a single pixel, or a collection of uniform pixels, with an angular size of 2º, or is it the full display, with all the pixels set to the same RGB pixel values?

- The next is "Reference White".
For the model we need the its tristimulus values X<sub>W</sub>, Y<sub>W</sub>, and Z<sub>W</sub>, and its absolute luminance, L<sub>W</sub>.
What is this Reference White, and its values?
Are the Reference White tristimulus values obtained by measuring a perfectly white reflecting sample, instead of the "colored' target sample?
Or is it the "Adopted White", as defined by ISO12231 as <i>"a stimulus that an observer who is adapted to the viewing environment would judge to be perfectly achromatic and to have a reflectance factor of unity (i.e., have absolute colorimetric coordinates that an observer would consider to be the perfect white diffuser)"</i>.
Very confusingly, sometimes the term "Adapted White" is used as well, which is the white point an observers assigns to a scene,
for example	an image viewed on a laptop computer, with the "adapted white" being the white in the scene of the image on the display, which can outdoor daylight scene, with a correlated color temperature of 6500K.
Looking at a image scene on a monitor, it is not always clear what the white point in the image is supposed to be; a very clear example of this is the famous picture of a dress, which can be interpreted as white-yellow, or blue-black, depending on the white point adapted by the observer. In that particular case, it is impossible to to describe its color appearance by the CIECAM model, or for that matter any color model at all.

- The "Proximal Field" is the area extending 2º around the Stimulus. This area is currently
ignored in the model, and are considered to be part of the Background.

- The "Background" in a model test set-up is the area around the Proximal Field, extending over a field angle of about 10º.
When considering color swatches, this background area is typically well defined: for images not;
sometimes the average of all the pixels in an image is taken as a representation of its background.
In CIECAM, only the relative luminance of the background area is used, with the symbol Y<sub>B</sub>, and is often set to a value of 20.

- "Surround" is the angular field outside the background, extending to an eye's full view.

# CIECAM Model Input Parameters

The following parameters are used in CIECAM02:

- (X,Y,Z): Tristimulus values of the target (e.g. surface, or pixel) to be described.
- (X<sub>W</sub>,Y<sub>W</sub>,Z<sub>W</sub>): Tristimulus values  of the reference white (perfect white surface, or white pixel).
For the model, the tristimulus values are normalized to Y<sub>W</sub>=100.
- L<sub>A</sub>: Absolute luminance of the adapting field , in cd/m<sup>2</sup>, which should be ideally measured with a photometer,
but which can be approximated by setting it to 20% of the luminance of the reference white L<sub>W</sub>, assuming an average reflectivity
of 20% for the objects in the adapting field.
For example, in a brightly lit room, average illuminance is 1000lux, which is approximately 318 cd/m<sup>2</sup>, so L<sub>W</sub>=318 cd/m2<sup>2</sup>,
resulting in an L<sub>A</sub>=64 cd/m<sup>2</sup>.
- Y<sub>b</sub>: Background relative luminance, which is 20 for the average "world grey" assumption in the previous item, but ideally be derived from
the absolute luminance of the background, and the absolute luminance of the reference white.
- S<sub>R</sub>: Surround ratio, the relative luminance of the target's surround, an area of approximately 10º in field around the target, which is assumed to have an angular extend of 2º. It is typically approximated by values described as "average", "dim", and
"dark", and is used through its derived dimensionless parameters impact factor c, chromatic induction factor N<sub>c</sub>, and degree of
adaptation F.
- D: Discounting-the-illuminant" parameter, representing in how far color constancy is in effect.
It can either be approximated by using the other parameters, or set manually; D is set to 1.0 when the illuminant is fully discounted, for example when viewing surface colors in a bright environment,
or is set to 0.0, at the other end of its range, when not in effect at all, such as when viewing pixels on a display in a complete dark environment,
In practice the value of D is never 0.0, as there is always some adaptation taking place, and is in higher than 0.6 in almost all cases.
If D is larger than 0, a chromatic adaptation transform is applied to a target's tristimulus values, described by the CIECAT02, the CIE recommended Chromatic Adaptation Transform as defined in 2002.





[FAIRCHILD2013]: https://www.wiley.com/en-us/Color+Appearance+Models%2C+3rd+Edition-p-9781119967033 "Color Appearance Models, 3rd Edition, Mark D. Fairchild, ISBN: 978-1-119-96703-3"
[CIECAM02LUO]: https://link.springer.com/chapter/10.1007/978-1-4419-6190-7_2 "C. Fernandez-Maloigne (ed.), Advanced Color Image Processing and Analysis,  DOI 10.1007/978-1-4419-6190-7 2, Springer Science+Business Media New York 2013"
