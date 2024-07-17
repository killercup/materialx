use std::{num::ParseIntError, str::FromStr};

use smol_str::SmolStr;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = SmolStr::from(s);
        let segments = s.split('.').collect::<Vec<_>>();
        if segments.len() != 2 {
            return Err(VersionError::InvalidLength { given: s.clone() });
        }
        Ok(Version {
            major: segments[0]
                .parse()
                .map_err(|source| VersionError::InvalidNumber {
                    given: s.clone(),
                    source,
                })?,
            minor: segments[1]
                .parse()
                .map_err(|source| VersionError::InvalidNumber {
                    given: s.clone(),
                    source,
                })?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VersionError {
    #[error("No version attribute found on materialx element")]
    NoVersion,
    #[error("Invalid version: {given}")]
    InvalidLength { given: SmolStr },
    #[error("Invalid version: {given}")]
    InvalidNumber {
        given: SmolStr,
        source: ParseIntError,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub enum ColorSpace {
    SrgbTexture,
    LinRec709,
    G22Rec709,
    G18Rec709,
    AcesCG,
    /// alias for "acescg"
    LinAp1,
    G22Ap1,
    G18Ap1,
    LinSrgb,
    AdobeRGB,
    LinAdobeRGB,
    SrgbDisplayP3,
    LinDisplayP3,
    /// Any other color space is treated as unknown
    Unknown(String),
}

impl FromStr for ColorSpace {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<ColorSpace, &'static str> {
        match s {
            "srgb_texture" => Ok(ColorSpace::SrgbTexture),
            "lin_rec709" => Ok(ColorSpace::LinRec709),
            "g22_rec709" => Ok(ColorSpace::G22Rec709),
            "g18_rec709" => Ok(ColorSpace::G18Rec709),
            "acescg" => Ok(ColorSpace::AcesCG),
            "lin_ap1" => Ok(ColorSpace::LinAp1),
            "g22_ap1" => Ok(ColorSpace::G22Ap1),
            "g18_ap1" => Ok(ColorSpace::G18Ap1),
            "lin_srgb" => Ok(ColorSpace::LinSrgb),
            "adobergb" => Ok(ColorSpace::AdobeRGB),
            "lin_adobergb" => Ok(ColorSpace::LinAdobeRGB),
            "srgb_displayp3" => Ok(ColorSpace::SrgbDisplayP3),
            "lin_displayp3" => Ok(ColorSpace::LinDisplayP3),
            s => Ok(ColorSpace::Unknown(s.into())),
        }
    }
}
