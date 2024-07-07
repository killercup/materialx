#![allow(clippy::large_enum_variant)]

use serde::{de::Error as _, Deserialize};
use std::str::FromStr;

pub use input::{ConversionError, Input};
use nodes::Node;
pub use typed_input::{DataType, DataTypeAndValue, ValueParseError};

mod input;
mod nodes;
mod typed_input;

/// MaterialX document
///
/// # Specification
///
/// see [MTLX File Format Definition](https://github.com/AcademySoftwareFoundation/MaterialX/blob/v1.39.0/documents/Specification/MaterialX.Specification.md#mtlx-file-format-definition)
#[derive(Debug, Deserialize)]
pub struct MaterialX {
    /// A string containing the version number of the MaterialX specification
    /// that this document conforms to, specified as a major and minor number
    /// separated by a dot.
    #[serde(rename = "@version")]
    pub version: Version,
    /// The name of the "working color space" for this element and all of its
    /// descendants. This is the default color space for all image inputs and
    /// color values, and the color space in which all color computations will
    /// be performed. The default is "none", for no color management.
    #[serde(rename = "@colorspace")]
    pub color_space: Option<ColorSpace>,
    /// Defines the namespace for all elements defined within this `<materialx>`
    /// scope.
    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "$value")]
    pub materials: Vec<TopLevel>,
}

impl MaterialX {
    pub fn get(&self, find_name: &str) -> Option<&TopLevel> {
        self.materials.iter().find(|m| match m {
            TopLevel::SurfaceMaterial { name, .. }
            | TopLevel::NodeGraph { name, .. }
            | TopLevel::StandardSurface { name, .. }
            | TopLevel::Image { name, .. }
            | TopLevel::TiledImage { name, .. } => name == find_name,
            _ => false,
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to parse XML {0}")]
    Xml(#[from] serde_path_to_error::Error<quick_xml::DeError>),
}

impl FromStr for MaterialX {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut deserializer = quick_xml::de::Deserializer::from_str(s);
        let res: Result<MaterialX, _> = serde_path_to_error::deserialize(&mut deserializer);
        Ok(res?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub r#type: DataType,
}

#[derive(Debug, Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl<'a> serde::Deserialize<'a> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Version, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        let mut iter = s.split('.');
        Ok(Version {
            major: iter
                .next()
                .ok_or_else(|| Error::missing_field("major version"))?
                .parse()
                .map_err(|e| Error::custom(format!("Invalid major version: {}", e)))?,
            minor: iter
                .next()
                .ok_or(Error::missing_field("minor version"))?
                .parse()
                .map_err(|e| Error::custom(format!("Invalid minor version: {}", e)))?,
        })
    }
}

#[derive(Debug)]
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

impl<'de> Deserialize<'de> for ColorSpace {
    fn deserialize<D>(deserializer: D) -> Result<ColorSpace, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ColorSpace::from_str(&s).map_err(D::Error::custom)
    }
}

/// Top level nodes
#[derive(Debug, Deserialize)]
pub enum TopLevel {
    #[serde(rename = "surfacematerial")]
    SurfaceMaterial {
        #[serde(rename = "@name")]
        name: String,
        // type: "material"
        input: Inputs,
    },
    #[serde(rename = "nodegraph")]
    NodeGraph {
        #[serde(rename = "@name")]
        name: String,
        // type: "material"
        #[serde(default)]
        input: Inputs,
        output: Vec<Output>,
        // TODO: hashmap with custom deserializer to collect nodes by `@name`
        #[serde(rename = "$value", default)]
        nodes: Vec<nodes::ParsedNode>,
    },
    #[serde(rename = "standard_surface")]
    StandardSurface {
        #[serde(rename = "@name")]
        name: String,
        input: Inputs,
    },
    Image {
        #[serde(rename = "@name")]
        name: String,
        #[serde(rename = "@type")]
        r#type: DataType,
        input: Inputs,
    },
    TiledImage {
        #[serde(rename = "@name")]
        name: String,
        #[serde(rename = "@type")]
        r#type: DataType,
        input: Inputs,
    },
    #[serde(other)]
    Other,
}

impl TopLevel {
    pub fn get(&self, name: &str) -> Option<&Input> {
        match self {
            TopLevel::SurfaceMaterial { input, .. }
            | TopLevel::NodeGraph { input, .. }
            | TopLevel::StandardSurface { input, .. }
            | TopLevel::Image { input, .. }
            | TopLevel::TiledImage { input, .. } => input.get(name),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(transparent)]
pub struct Inputs(pub Vec<Input>);

impl Inputs {
    pub fn get(&self, name: &str) -> Option<&Input> {
        self.0.iter().find(|i| i.name == name)
    }
}

#[derive(Debug, Deserialize)]
pub struct Output {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub r#type: DataType,
    #[serde(rename = "@nodename")]
    pub nodename: Option<String>,
    #[serde(rename = "@uniform")]
    pub uniform: Option<bool>,
    #[serde(rename = "@output")]
    pub output: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_one() {
        let file = std::fs::read_to_string(
            "../resources/materialx-examples/StandardSurface/standard_surface_brass_tiled.mtlx",
        )
        .unwrap();
        dbg!(MaterialX::from_str(&file));
    }

    // tries to parse all mtlx files in the examples folder
    #[test]
    fn all() {
        let examples = glob::glob("../resources/**/*.mtlx").unwrap();
        let mut failed = 0;
        for example in examples {
            let example = example.unwrap();
            let path = example.as_path();
            if path.is_dir() {
                continue;
            }
            if path.extension().unwrap() == "mtlx" {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let xml = std::fs::read_to_string(path).unwrap();

                match MaterialX::from_str(&xml) {
                    Ok(_) => println!("{name}: Success"),
                    Err(e) => {
                        eprintln!("{name}: Failed {e}");
                        failed += 1;
                    }
                }
            }
        }

        if failed > 0 {
            panic!("{failed} example files failed to parse");
        }
    }
}
