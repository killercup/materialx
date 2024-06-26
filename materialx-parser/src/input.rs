use crate::{ColorSpace, DataType, DataTypeAndValue};
use bevy_color::{Color as BevyColor, LinearRgba};
use serde::{de::Error as _, Deserialize};
use std::{collections::HashMap, str::FromStr as _};

#[derive(Debug)]
pub struct Input {
    // #[serde(rename = "@name")]
    pub name: String,
    // #[serde(rename = "@type")]
    pub raw_type: String,
    // #[serde(rename = "@type")]
    pub r#type: DataType,
    // #[serde(rename = "@value")]
    pub raw_value: Option<String>,
    pub value: Option<DataTypeAndValue>,
    // #[serde(rename = "@colorspace")]
    pub color_space: Option<ColorSpace>,
    // #[serde(rename = "@nodename")]
    pub nodename: Option<String>,
    // #[serde(rename = "@nodegraph")]
    pub nodegraph: Option<String>,
    // #[serde(rename = "@output")]
    pub output: Option<String>,
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D>(deserializer: D) -> Result<Input, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = HashMap::<String, String>::deserialize(deserializer)?;
        let name = input
            .get("@name")
            .ok_or_else(|| D::Error::missing_field("@name"))?
            .to_string();

        let raw_type = input
            .get("@type")
            .ok_or_else(|| D::Error::missing_field("@type"))?
            .to_string();
        let raw_value = input.get("@value").map(|s| s.to_string());
        let value = if let Some(raw_value) = raw_value.as_ref() {
            Some(
                DataTypeAndValue::from_tag_and_value(&raw_type, raw_value)
                    .map_err(D::Error::custom)?,
            )
        } else {
            None
        };

        let color_space = input
            .get("@colorspace")
            .map(|s| ColorSpace::from_str(s))
            .transpose()
            .map_err(D::Error::custom)?;

        Ok(Input {
            name,
            r#type: raw_type.parse().map_err(D::Error::custom)?,
            raw_type,
            raw_value,
            value,
            color_space,
            nodename: input.get("@nodename").map(|s| s.to_string()),
            nodegraph: input.get("@nodegraph").map(|s| s.to_string()),
            output: input.get("@output").map(|s| s.to_string()),
        })
    }
}

impl TryFrom<&Input> for BevyColor {
    type Error = ConversionError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let Some(value) = input.value.as_ref() else {
            return Err(ConversionError::InputWithoutValue {
                name: input.name.clone(),
            });
        };
        match value {
            DataTypeAndValue::Color3(c) => Ok(BevyColor::linear_rgb(
                c.0[0] as f32,
                c.0[1] as f32,
                c.0[2] as f32,
            )),
            DataTypeAndValue::Color4(c) => Ok(BevyColor::linear_rgba(
                c.0[0] as f32,
                c.0[1] as f32,
                c.0[2] as f32,
                c.0[3] as f32,
            )),
            _ => Err(ConversionError::UnexpectedFormat {
                name: input.name.clone(),
                format: input.r#type.clone(),
            }),
        }
    }
}

impl TryFrom<&Input> for LinearRgba {
    type Error = ConversionError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let color: BevyColor = input.try_into()?;
        Ok(color.into())
    }
}

impl TryFrom<&Input> for f32 {
    type Error = ConversionError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let Some(value) = input.value.as_ref() else {
            return Err(ConversionError::InputWithoutValue {
                name: input.name.clone(),
            });
        };
        match value {
            DataTypeAndValue::Float(f) => Ok(*f as f32),
            _ => Err(ConversionError::UnexpectedFormat {
                name: input.name.clone(),
                format: input.r#type.clone(),
            }),
        }
    }
}

impl TryFrom<&Input> for f64 {
    type Error = ConversionError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let Some(value) = input.value.as_ref() else {
            return Err(ConversionError::InputWithoutValue {
                name: input.name.clone(),
            });
        };
        match value {
            DataTypeAndValue::Float(f) => Ok(*f),
            _ => Err(ConversionError::UnexpectedFormat {
                name: input.name.clone(),
                format: input.r#type.clone(),
            }),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum ConversionError {
    #[error("Unexpected input without value: {name}")]
    InputWithoutValue { name: String },
    #[error("Unexpected data format in {name}: {format:?}")]
    UnexpectedFormat { name: String, format: DataType },
}
