use super::{DataTypeAndValue, ValueParseError};
use bevy_color::{Color as BevyColor, LinearRgba};

impl TryFrom<DataTypeAndValue> for BevyColor {
    type Error = ValueParseError;

    fn try_from(value: DataTypeAndValue) -> Result<Self, Self::Error> {
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
            _ => Err(ValueParseError::UnexpectedFormat {
                format: value.tag(),
            }),
        }
    }
}

impl TryFrom<DataTypeAndValue> for LinearRgba {
    type Error = ValueParseError;

    fn try_from(value: DataTypeAndValue) -> Result<Self, Self::Error> {
        let color: BevyColor = value.try_into()?;
        Ok(color.into())
    }
}

impl TryFrom<DataTypeAndValue> for f32 {
    type Error = ValueParseError;

    fn try_from(value: DataTypeAndValue) -> Result<Self, Self::Error> {
        match value {
            DataTypeAndValue::Float(f) => Ok(f as f32),
            _ => Err(ValueParseError::UnexpectedFormat {
                format: value.tag(),
            }),
        }
    }
}

impl TryFrom<DataTypeAndValue> for f64 {
    type Error = ValueParseError;

    fn try_from(value: DataTypeAndValue) -> Result<Self, Self::Error> {
        match value {
            DataTypeAndValue::Float(f) => Ok(f),
            _ => Err(ValueParseError::UnexpectedFormat {
                format: value.tag(),
            }),
        }
    }
}
