/*!

IES TM30 Sample Illuminants

A collection of spectral emissivity data for sample lamps, standard reference illuminant spectra,
and spectral distributions from mathematical models, as used by the IES TM30 standardization committee for testing
their color fidelity and color preference metrics.
The intention of this dataset is to be used for general color science research, and the spectral distributions of 
the commcercially available lamps have to be considered as just examples, and are by no means representative for 
the typical or average performance of these lamps.

The data can be used as illuminants through the `TM30Illuminant` type, which takes a `const` identifier as type
argument. The collection is described below, using the categories as used by the IES TM30 committee.
The data is given on the wavelength range from 380 to 780nm, with steps of 1nm.







The collection consists of:

# Fluorescent Lamps

## Broadband 
`EmissionType::FluorescentBroadBand`

### CIE Standard Illuminants
CIE_F1, CIE_F2, CIE_F3, CIE_F4, CIE_F5, CIE_F6, CIE_F7, CIE_F8, CIE_F9,  

### Measured Commercial
F32T8_930, F32T8_950, F34T12_CW_RS_EW, F34T12_LW_RS_EW, F34T12_WW_RS_EW, F40T12_C50_1, F40T12_C50_2, F40T12_C75_1, F40T12_C75_2, F40T12_CWX_1,
F40T12_CWX_2, F40T12_DX_1, F40T12_DX_2, F40T12_DXTP_1, F40T12_DXTP_2, F40T12_N_1, F40T12_N_2,

## Narrowband
`EmissionType::FluorescentNarrowBand`

### CIE Standard Illuminants
CIE_F10, CIE_F11, CIE_F12,

### Measured Commercial
F32T8_730, F32T8_735, F32T8_741,
F32T8_750, F32T8_830_1, F32T8_830_2, F32T8_830_3, F32T8_835_1, F32T8_835_2, F32T8_835_3, F32T8_841_1, F32T8_841_2, F32T8_841_3,
F32T8_850_1, F32T8_850_2, F32T8_850_3, F32T8_865_1, F32T8_865_2, F40T12_30U, F40T12_35U, F40T12_41U, F40T12_50U,

### Modifield Fluorescent Lamps

MODIFIED_TRIPHOSPHOR_DUV_0_000, MODIFIED_TRIPHOSPHOR_DUV_0_006, MODIFIED_TRIPHOSPHOR_DUV_0_01, MODIFIED_TRIPHOSPHOR_DUV_NEG_0_005,

# High Intensity Discharge Lamps 
`EmissionType::HighIntensityDischarge`

All the spectral distributions in this category are measured spectral distribution from commercially available lamps.

C100S54_1_HPS_STANDARD, C100S54_2_HPS_STANDARD, C100S54C_1_HPS_DELUXE, C100S54C_2_HPS_DELUXE, SDW_T_100W_LV_1_SUPER_HPS,

## SDW
SDW_T_100W_LV_2_SUPER_HPS, 
	
## CDM
CDM_830_1_METAL_HALIDE, CDM_830_2_METAL_HALIDE, CDM_830_3_METAL_HALIDE, CDM_940_1_METAL_HALIDE,
CDM_940_2_METAL_HALIDE, 

## Metal Halide
MH100W_METAL_HALIDE, MHC100_U_MP_3K_METAL_HALIDE, MHC100_U_MP_4K_METAL_HALIDE, MHC100UMP3K_METAL_HALIDE,
MHC100UMP4K_METAL_HALIDE, 
	
## Mercury

H38HT_100_1_MERCURY, H38HT_100_2_MERCURY, H38JA_100_DX_1_MERCURY, H38JA_100_DX_2_MERCURY,


# Incandescent Lamps 
`EmissionType::IncandescentOrFilament`

All the spectral distributions in this category are measured examples of commercially available lamps:`

- 3 Halogen lamps (`HALOGEN_1`, `HALOGEN_2`, and `HALOGEN_3`)
- 3 Halogen MR16 spot lamps (`HALOGEN_MR16_1`, `HALOGEN_MR16_2`, `HALOGEN_MR16_3`)
- A standard 60W A 19 incandescent lamp (`INCANDESCENT_60WA19`)
- A 75W A19 Halogena lamp, an A19 shaped bulb with a mains voltage Halogen lamp as source inside (`INCANDESCENT_75WA19_HALOGENA`)
- A 75W A19 Incandescent lamp, with its light filtered by Neodymium coating on the insize of the bulb (`INCANDESCENT_75WA19_NEODYMIUM`)
- Another 75W A19 Incandescent lamp example, in this case one of the so-called "Rough House" variety,
	which has a stronger filament design  (`INCANDESCENT_75WA19_ROUGH_HOUSE`)
- And a 75W A19 "Softer White" sample (`INCANDESCENT_75WA19_SOFTER_WHITE`) 
- Krypton Incandescent (`KRYPTON_INCANDESCENT`)
- Neodymium Incandescent (`NEODYMIUM_INCANDESCENT`)
- and last, a "Filtered Halogen" example (`FILTERED_HALOGEN`)

# LED Hybrid Lamps 
`EmissionType::LedHybrid`

## Blue Pump
`LED_HYBRID_BLUE_PUMP_1`, `LED_HYBRID_BLUE_PUMP_2`, `LED_HYBRID_BLUE_PUMP_3`, `LED_HYBRID_BLUE_PUMP_4`, `LED_HYBRID_BLUE_PUMP_5`, `LED_HYBRID_BLUE_PUMP_6`, `LED_HYBRID_BLUE_PUMP_7`, `LED_HYBRID_BLUE_PUMP_8`, `LED_HYBRID_BLUE_PUMP_9`, `LED_HYBRID_BLUE_PUMP_10`, `LED_HYBRID_BLUE_PUMP_11`, `LED_HYBRID_BLUE_PUMP_12`, `LED_HYBRID_BLUE_PUMP_13`,

## Violet Pump
`LED_HYBRID_VIOLET_PUMP_1`, `LED_HYBRID_VIOLET_PUMP_2`,


# Mixed distributions for RGB, and RGBA LED lamps (`EmissionType::LedMixed`)

## RGB
`RGB_445_500_640_E`, `RGB_445_500_640_M`, `RGB_445_515_640`, `RGB_445_520_640`, `RGB_450_525_625_3K`, `RGB_450_525_625_4K`, `RGB_450_530_645`, `RGB_455_530_615`, `RGB_455_534_616`, `RGB_455_547_623`, `RGB_457_540_605`, `RGB_460_525_625`, `RGB_460_540_620`, `RGB_464_538_613`, `RGB_464_546_620`, `RGB_464_562_626`, `RGB_465_525_630`, `RGB_465_530_620`, `RGB_465_535_590`, `RGB_465_546_614`, `RGB_466_538_603`, `RGB_467_548_616`, `RGB_470_525_630`, `RGB_470_530_635`, `RGB_472_550_603`, `RGB_473_545_616`, `RGB_474_545_616_DUV_0_006`, `RGB_474_545_616_DUV_0_01`, `RGB_474_545_616_DUV_0_000`, `RGB_474_545_616_DUV_NEG_0_006`, `RGB_475_515_630`, `RGB_475_545_615`,

## RGBA
`RGBA_445_495_555_615_E`, `RGBA_445_495_555_615_M`, `RGBA_445_505_590_640`, `RGBA_445_505_595_640`, `RGBA_445_520_595_640`, `RGBA_447_512_573_627`, `RGBA_450_525_580_625`, `RGBA_450_525_600_650`, `RGBA_455_530_590_635_3K`, `RGBA_455_530_590_635_3K_A`, `RGBA_455_530_590_635_3K_B`, `RGBA_455_530_590_635_3K_C`, `RGBA_455_530_590_635_3K_D`, `RGBA_455_530_590_635_3K_E`, `RGBA_455_530_590_635_3K_F`, `RGBA_455_530_590_635_3K_G`, `RGBA_455_530_590_635_3K_H`, `RGBA_455_530_590_635_3K_I`, `RGBA_455_530_590_635_4K`, `RGBA_455_530_590_635_4K_A`, `RGBA_455_530_590_635_4K_B`, `RGBA_455_530_590_635_4K_C`, `RGBA_455_530_590_635_4K_D`, `RGBA_455_530_590_635_4K_E`, `RGBA_455_530_590_635_4K_F`, `RGBA_455_530_590_635_4K_G`, `RGBA_455_530_590_635_4K_H`, `RGBA_455_530_590_635_4K_I`, `RGBA_460_530_575_625`, `RGBA_461_526_576_624`, `RGBA_470_520_595_635_27K`, `RGBA_470_520_595_635_65K`,

# LDRD 
`LDRD1`, `LDRD2`, `LDRD3`, `LDRD4`, `LDRD5`, `LDRD6`, `LDRD7`, `LDRD8`, `LDRD9`, `LDRD10`, `LDRD11`, `LDRD12`, `LDRD13`, `LDRD14`, `LDRD15`, `LDRD16`, `LDRD17`, `LDRD18`, `LDRD19`, `LDRD20`, `LDRD21`, `LDRD22`, `LDRD23`, `LDRD24`, `LDRD25`, `LDRD26`,


# LED Phosphor Lamps 
`EmissionType::LedPhosphor`

## Blue Pump
`LED_PHOSPHOR_BLUE_PUMP_01`, `LED_PHOSPHOR_BLUE_PUMP_02`, `LED_PHOSPHOR_BLUE_PUMP_03`, `LED_PHOSPHOR_BLUE_PUMP_04`, `LED_PHOSPHOR_BLUE_PUMP_05`, `LED_PHOSPHOR_BLUE_PUMP_06`, `LED_PHOSPHOR_BLUE_PUMP_07`, `LED_PHOSPHOR_BLUE_PUMP_08`, `LED_PHOSPHOR_BLUE_PUMP_09`, `LED_PHOSPHOR_BLUE_PUMP_10`, `LED_PHOSPHOR_BLUE_PUMP_11`, `LED_PHOSPHOR_BLUE_PUMP_12`, `LED_PHOSPHOR_BLUE_PUMP_13`, `LED_PHOSPHOR_BLUE_PUMP_14`, `LED_PHOSPHOR_BLUE_PUMP_15`, `LED_PHOSPHOR_BLUE_PUMP_16`, `LED_PHOSPHOR_BLUE_PUMP_17`, `LED_PHOSPHOR_BLUE_PUMP_18`, `LED_PHOSPHOR_BLUE_PUMP_19`, `LED_PHOSPHOR_BLUE_PUMP_20`, `LED_PHOSPHOR_BLUE_PUMP_21`, `LED_PHOSPHOR_BLUE_PUMP_22`, `LED_PHOSPHOR_BLUE_PUMP_23`, `LED_PHOSPHOR_BLUE_PUMP_24`, `LED_PHOSPHOR_BLUE_PUMP_25`, `LED_PHOSPHOR_BLUE_PUMP_26`, `LED_PHOSPHOR_BLUE_PUMP_27`, `LED_PHOSPHOR_BLUE_PUMP_28`, `LED_PHOSPHOR_BLUE_PUMP_29`, `LED_PHOSPHOR_BLUE_PUMP_30`, `LED_PHOSPHOR_BLUE_PUMP_31`, `LED_PHOSPHOR_BLUE_PUMP_32`, `LED_PHOSPHOR_BLUE_PUMP_33`, `LED_PHOSPHOR_BLUE_PUMP_34`, `LED_PHOSPHOR_BLUE_PUMP_35`, `LED_PHOSPHOR_BLUE_PUMP_36`, `LED_PHOSPHOR_BLUE_PUMP_37`, `LED_PHOSPHOR_BLUE_PUMP_38`, `LED_PHOSPHOR_BLUE_PUMP_39`, `LED_PHOSPHOR_BLUE_PUMP_40`, `LED_PHOSPHOR_BLUE_PUMP_41`, `LED_PHOSPHOR_BLUE_PUMP_42`, `LED_PHOSPHOR_BLUE_PUMP_43`, `LED_PHOSPHOR_BLUE_PUMP_44`, `LED_PHOSPHOR_BLUE_PUMP_45`, `LED_PHOSPHOR_BLUE_PUMP_46`, `LED_PHOSPHOR_BLUE_PUMP_47`, `LED_PHOSPHOR_BLUE_PUMP_48`, `LED_PHOSPHOR_BLUE_PUMP_49`, `LED_PHOSPHOR_BLUE_PUMP_50`, `LED_PHOSPHOR_BLUE_PUMP_51`, `LED_PHOSPHOR_BLUE_PUMP_52`, `LED_PHOSPHOR_BLUE_PUMP_53`, `LED_PHOSPHOR_BLUE_PUMP_54`, `LED_PHOSPHOR_BLUE_PUMP_55`, `LED_PHOSPHOR_BLUE_PUMP_56`, `LED_PHOSPHOR_BLUE_PUMP_57`, `LED_PHOSPHOR_BLUE_PUMP_58`, `LED_PHOSPHOR_BLUE_PUMP_59`, `LED_PHOSPHOR_BLUE_PUMP_60`, `LED_PHOSPHOR_BLUE_PUMP_61`, `LED_PHOSPHOR_BLUE_PUMP_62`, `LED_PHOSPHOR_BLUE_PUMP_63`, `LED_PHOSPHOR_BLUE_PUMP_64`, `LED_PHOSPHOR_BLUE_PUMP_65`, `LED_PHOSPHOR_BLUE_PUMP_66`, `LED_PHOSPHOR_BLUE_PUMP_67`, `LED_PHOSPHOR_BLUE_PUMP_68`, `LED_PHOSPHOR_BLUE_PUMP_69`, `LED_PHOSPHOR_BLUE_PUMP_70`, `LED_PHOSPHOR_BLUE_PUMP_71`, `LED_PHOSPHOR_BLUE_PUMP_72`, `LED_PHOSPHOR_BLUE_PUMP_73`, `LED_PHOSPHOR_BLUE_PUMP_74`, `LED_PHOSPHOR_BLUE_PUMP_75`, `LED_PHOSPHOR_BLUE_PUMP_76`, `LED_PHOSPHOR_BLUE_PUMP_77`, `LED_PHOSPHOR_BLUE_PUMP_78`, `LED_PHOSPHOR_BLUE_PUMP_79`, `LED_PHOSPHOR_BLUE_PUMP_80`, `LED_PHOSPHOR_BLUE_PUMP_81`, `LED_PHOSPHOR_BLUE_PUMP_82`, `LED_PHOSPHOR_BLUE_PUMP_83`, `LED_PHOSPHOR_BLUE_PUMP_84`, `LED_PHOSPHOR_BLUE_PUMP_85`, `LED_PHOSPHOR_BLUE_PUMP_86`, `LED_PHOSPHOR_BLUE_PUMP_87`, `LED_PHOSPHOR_BLUE_PUMP_88`, `LED_PHOSPHOR_BLUE_PUMP_89`, `LED_PHOSPHOR_BLUE_PUMP_90`, `LED_PHOSPHOR_BLUE_PUMP_91`, `LED_PHOSPHOR_BLUE_PUMP_92`, `LED_PHOSPHOR_BLUE_PUMP_93`, `LED_PHOSPHOR_BLUE_PUMP_94`, `LED_PHOSPHOR_BLUE_PUMP_95`, `LED_PHOSPHOR_BLUE_PUMP_96`, `LED_PHOSPHOR_BLUE_PUMP_97`, `LED_PHOSPHOR_BLUE_PUMP_98`, `LED_PHOSPHOR_BLUE_PUMP_99`, `LED_PHOSPHOR_BLUE_PUMP_100`, `LED_PHOSPHOR_BLUE_PUMP_101`, `LED_PHOSPHOR_BLUE_PUMP_102`, `LED_PHOSPHOR_BLUE_PUMP_103`, `LED_PHOSPHOR_BLUE_PUMP_104`, `LED_PHOSPHOR_BLUE_PUMP_105`, `LED_PHOSPHOR_BLUE_PUMP_106`, `LED_PHOSPHOR_BLUE_PUMP_107`, `LED_PHOSPHOR_BLUE_PUMP_108`, `LED_PHOSPHOR_BLUE_PUMP_109`, `LED_PHOSPHOR_BLUE_PUMP_110`, `LED_PHOSPHOR_BLUE_PUMP_111`, `LED_PHOSPHOR_BLUE_PUMP_112`, `LED_PHOSPHOR_BLUE_PUMP_113`, `LED_PHOSPHOR_BLUE_PUMP_114`, `LED_PHOSPHOR_BLUE_PUMP_115`, `LED_PHOSPHOR_BLUE_PUMP_116`, `LED_PHOSPHOR_BLUE_PUMP_117`, `LED_PHOSPHOR_BLUE_PUMP_118`, `LED_PHOSPHOR_BLUE_PUMP_119`, `LED_PHOSPHOR_BLUE_PUMP_120`, `LED_PHOSPHOR_BLUE_PUMP_121`, `LED_PHOSPHOR_BLUE_PUMP_122`, `LED_PHOSPHOR_BLUE_PUMP_123`, `LED_PHOSPHOR_BLUE_PUMP_124`, `LED_PHOSPHOR_BLUE_PUMP_125`,

## Violet Pump
`LED_PHOSPHOR_VIOLET_PUMP_1`, `LED_PHOSPHOR_VIOLET_PUMP_2`, `LED_PHOSPHOR_VIOLET_PUMP_3`, `LED_PHOSPHOR_VIOLET_PUMP_4`,


# Mathematical Spectral Distributions 
`EmissionType::Mathematical`

## CIE D-Series

`CIE_D_SERIES_5000_K`, `CIE_D_SERIES_5500_K`, `CIE_D_SERIES_6000_K`, `CIE_D_SERIES_6500_K`, `CIE_D_SERIES_7000_K`, `CIE_D_SERIES_7500_K`, `CIE_D_SERIES_8000_K`,

## Mixed reference
`MIXED_REFERENCE_5000_K`,

## Planckian Sources
`PLANCKIAN_RADIATION_2000_K`, `PLANCKIAN_RADIATION_2500_K`, `PLANCKIAN_RADIATION_2700_K`, `PLANCKIAN_RADIATION_3000_K`, `PLANCKIAN_RADIATION_3500_K`, `PLANCKIAN_RADIATION_4000_K`, `PLANCKIAN_RADIATION_4500_K`, `PLANCKIAN_RADIATION_4999_K`,


# Other Spectral Distributions
`EmissionType::Other`


## Equal Energy spectrum
`EQUAL_ENERGY`,

## Ideal Prime color
`IDEAL_PRIME_COLOR`,

# Plasma lamps
`PLASMA`,

# Triband Gaussian model
`TRI_BAND_GAUSSIAN_1`, `TRI_BAND_GAUSSIAN_2`,






*/


mod data;
pub use data::*;

use std::collections::HashMap;
use colorado::illuminants::Illuminant;
use colorado::observers::StandardObserver;
use colorado::models::{CieXYZ, XYZValues};
use colorado::{DataSpectrumFromSlice, Domain, NM, SpectralDistribution, WavelengthStep};
use nalgebra::{SVectorSlice, Matrix3xX, SMatrixSlice};
use self::data::{TM30_ILLUMINANTS_DATA, TM30_CIE1931, N, M};


#[derive(Clone,  PartialEq,  Eq)]
pub enum EmissionType {
	FluorescentBroadband = 0,
	FluorescentNarrowband = 1,
	HighIntensityDischarge = 3,
	IncandescentOrFilament = 4,
	LedHybrid = 5,
	LedMixed = 6,
	LedPhosphor = 7,
	Mathematical = 8,
	Other = 9,
}

pub enum ModelType {
	Model = 0,
	Commercial = 1,
	Experimental = 2,
	Theoretical = 3,
}

pub fn tm30_cie1931_xy() -> HashMap<&'static str, [f64;2]> {
	TM30_CIE1931.iter().map(|(key,_,_,x,y)|(*key,[*x,*y])).collect()
}


impl From<EmissionType> for Vec<&str> {
	fn from(et: EmissionType) -> Self {
		let e = et as u32;
		let mut v: Vec<&str> = Vec::with_capacity(M);
		for (k,j,..) in TM30_CIE1931.iter() {
			if e==*j {
				v.push(k);
			}
		}
		v
	}

}

impl<C: StandardObserver> From<EmissionType> for CieXYZ<C> {
	fn from(et: EmissionType) -> Self {
		let e = et as u32;
		let mut v: Vec<f64> = Vec::with_capacity(3*M);
		for (i,(_,j,..)) in TM30_CIE1931.iter().enumerate(){
			if e==*j {
				let sd = DataSpectrumFromSlice::new(Domain::new(380, 780, NM), &TM30_ILLUMINANTS_DATA[i*N..(i+1)*N]);
				let XYZValues{x, y, z} = sd.xyz::<C>().into_iter().next().unwrap();
				v.push(x);
				v.push(y);
				v.push(z);
			}
		}
		Self::new(Matrix3xX::from_vec(v))
	}
}


/**
Use TM30 Sample Spectrum as illuminant.

Illuminants are used in this library to represent the, typically 'white', illumination
used to illuminate objects such as swatches, or backlight LCD pixels.
Each illuminant in the colorado library has its own type, and is constructed by its
`Default::default` method only.

The use the spectral distributions in the TM30 library as illuminant in the color models
use the `TM30Illuminant::<const K:usize>` type, where `K` can be specified as one of the 
TM30 library constants.

For example, to get the CIE F1 illuminant from this library, use the `CIE_F1` constant:
```
	use colorado_tm30::samples::{TM30Illuminant, CIE_F1};
	let ill = TM30Illuminant::<CIE_F1>;

	use colorado::models::{CieYxy, YxyValues};
	let xy: CieYxy = ill.into();
	let YxyValues{l:_, x, y} = xy.into_iter().next().unwrap();

	use approx::assert_abs_diff_eq;
	assert_abs_diff_eq!(x,0.313100, epsilon=1E-6);
	assert_abs_diff_eq!(y,0.337279, epsilon=1E-6);

```
*/
#[derive(Default)]
pub struct TM30Illuminant<const K:usize>;

impl<const K:usize> SpectralDistribution for TM30Illuminant<K> {
    type MatrixType = SVectorSlice<'static, f64, N>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (Domain::new(380, 780, NM), Self::MatrixType::from_slice(&TM30_ILLUMINANTS_DATA[(K-1)*N..K*N]))
    }

    fn shape(&self) -> (usize, usize) {
		(N,1)
    }
}

impl<const K:usize> Illuminant for TM30Illuminant<K>{}

impl<C: StandardObserver, const K: usize> From<TM30Illuminant<K>> for CieXYZ<C> {
	fn from(ill: TM30Illuminant<K>) -> Self {
		ill.xyz()	
	}
}

#[test]
fn test_tm30_ill(){
	use colorado::models::{CieYxy, YxyValues};
	use approx::assert_abs_diff_eq;

	let ill = TM30Illuminant::<CIE_F1>;
	let xy: CieYxy = ill.into();
	let YxyValues{l:_, x, y} = xy.into_iter().next().unwrap();

	assert_abs_diff_eq!(x,0.313100, epsilon=1E-6);
	assert_abs_diff_eq!(y,0.337279, epsilon=1E-6);
}

#[derive(Default)]
pub struct TM30Illuminants;

impl SpectralDistribution for TM30Illuminants {
    type MatrixType = SMatrixSlice<'static, f64, N, M>;
    type StepType = WavelengthStep;

    fn spd(&self) -> (Domain<Self::StepType>, Self::MatrixType) {
        (Domain::new(380, 780, NM), Self::MatrixType::from_slice(&TM30_ILLUMINANTS_DATA))
    }

    fn shape(&self) -> (usize, usize) {
		(N,M)
    }
}


impl<C: StandardObserver> From<TM30Illuminants> for CieXYZ<C> {
	fn from(ill: TM30Illuminants) -> Self {
		ill.xyz()	
	}
}

#[test]
fn test_tm30_illuminants(){
	use colorado::models::CieYxy;
	let ill = TM30Illuminants;
	let xy: CieYxy = ill.into();
	println!{"{}", xy.data.transpose()};
}




#[test]
fn test_from_emission_type(){
	use colorado::models::{CieYxy, YxyValues};
	use approx::assert_abs_diff_eq;

	for emission_type in [
			EmissionType::FluorescentNarrowband,
			EmissionType::FluorescentBroadband,
			EmissionType::HighIntensityDischarge,
			EmissionType::IncandescentOrFilament,
			EmissionType::LedHybrid,
			EmissionType::LedMixed,
			EmissionType::LedPhosphor,
			EmissionType::Mathematical,
			EmissionType::Other,
		]
		{
		let xyz: CieYxy = emission_type.clone().into();
		let keys: Vec<&str> = emission_type.into();
		let w = tm30_cie1931_xy();
		for (YxyValues {l: _, x,y}, k) in xyz.into_iter().zip(keys.into_iter()) {
			let [xw,yw] = w[k];
			assert_abs_diff_eq!(x,xw,epsilon=5E-7);
			assert_abs_diff_eq!(y,yw,epsilon=5E-7);
			println!("{} {} {} {} {}" , k, x, y, xw, yw);
		}

	}
}