


//!  **Colorado** is an open-source Color Science library, written in the Rust programming language.
//! It is being developed by Gerard Harbers (Harbers Bik LLC).
//! Its intended use is color research and development of color management tools for displays, graphic arts,
//! and architectural lighting.
//! It implements methods and standards as defined by international standard organizations
//! (such as the **CIE**, the *International Commission on Illumination*),
//! and many other, still experimental color algorithms and data.
//! 
//! Colorado is free and open-source, and is released under the Apache 2.0 license.
//! 
//! The library contains a large collection of spectral distribution data, and provides many spectral calculations tools.
//! It uses `nalgebra`'s linear algebra and matrix library for implementation of the algorithms.
//!
//! Spectral distributions are typically measured using spectrometers, and the library supports various data formats of spectral distribution files.
//! Besides using measured data from files, the library also includes many spectral distributions defined by international standards, and a collection of spectral libraries.
//! Spectral distributions can also be derived from physics laws and mathematical models, also included in the library.
//! 
//! ## Features
//! - Spectral power distributions library for a lamps and illuminants, such as fluorescent and LED lamps.
//! - Materials spectral reflectivity library.
//! - Spectral distributions from mathematical models such as Planck blackbody radiators, Gaussian spectra, and mathematical LED models.
//! - Calculate tristimulus values from spectral data using a number of standard observers such as CIE1931 2º, CIE1964 10º, CIE 2015 2º and 10º.
//! - Calculate chromaticity coordinates and appearance correlates based on a number of color models.
//! 
//! ## Disclaimer
//! The data, methods, and algorithms in this library, 
//! referencing Standard Organizations such as the International Commission on Illumination (CIE), or any other Standards Organizations, 
//! have not been endorsed, qualified, or approved by these Standard Organizations. 
//! Please consult their documentation and standards for authoritative methods, recommendations, and data. 
//! If you find any deviations or errors between the official standards and the implementations in this library, please report them as issues on its GitHub page.
//! Please be also aware that light and color measurements are difficult and depend on a lot of factors, 
//! with many of them outside the scope of this library and its applications. 
//! If you have a professional lighting, display lighting, or color issue, please consult a specialist.
//! 
//! <p align="center">
//! Copyright (c) 2021 Harbers Bik LLC.<br>
//! All rights reserved.
//! </p>


/**
	CIE Tristimulus Values
 */
pub mod cie;

pub mod cct; // correlated color temperature 

pub mod spectra; // spectral matrix and range 

pub mod blackbody; // blackbody radiator illuminants

/**
CIE Stanadard Observers
 */
pub mod observer; 


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
