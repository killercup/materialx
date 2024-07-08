use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum DataType {
    Integer,
    Boolean,
    Float,
    Color3,
    Color4,
    Vector2,
    Vector3,
    Vector4,
    Matrix3x3,
    Matrix4x4,
    String,
    Filename,
    IntegerArray,
    FloatArray,
    Color3Array,
    Color4Array,
    Vector2Array,
    Vector3Array,
    Vector4Array,
    StringArray,
    Unknown(String),
}

impl FromStr for DataType {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "integer" => Ok(DataType::Integer),
            "boolean" => Ok(DataType::Boolean),
            "float" => Ok(DataType::Float),
            "color3" => Ok(DataType::Color3),
            "color4" => Ok(DataType::Color4),
            "vector2" => Ok(DataType::Vector2),
            "vector3" => Ok(DataType::Vector3),
            "vector4" => Ok(DataType::Vector4),
            "matrix33" => Ok(DataType::Matrix3x3),
            "matrix44" => Ok(DataType::Matrix4x4),
            "string" => Ok(DataType::String),
            "filename" => Ok(DataType::Filename),
            "integerarray" => Ok(DataType::IntegerArray),
            "floatarray" => Ok(DataType::FloatArray),
            "color3array" => Ok(DataType::Color3Array),
            "color4array" => Ok(DataType::Color4Array),
            "vector2array" => Ok(DataType::Vector2Array),
            "vector3array" => Ok(DataType::Vector3Array),
            "vector4array" => Ok(DataType::Vector4Array),
            "stringarray" => Ok(DataType::StringArray),
            s => Ok(DataType::Unknown(s.into())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataTypeAndValue {
    Integer(u64),
    Boolean(bool),
    Float(f64),
    Color3(Color3),
    Color4(Color4),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
    Matrix3x3(Matrix3x3),
    Matrix4x4(Matrix4x4),
    String(String),
    Filename(String),
    IntegerArray(Vec<u64>),
    FloatArray(Vec<f64>),
    Color3Array(Vec<Color3>),
    Color4Array(Vec<Color4>),
    Vector2Array(Vec<Vector2>),
    Vector3Array(Vec<Vector3>),
    Vector4Array(Vec<Vector4>),
    StringArray(Vec<String>),
    Unknown { tag: String, value: String },
}

impl DataTypeAndValue {
    pub fn from_tag_and_value(tag: &str, value: &str) -> Result<Self, ValueParseError> {
        match tag {
            "integer" => value
                .parse()
                .map_err(|e| ValueParseError::InvalidInteger {
                    got: value.into(),
                    source: e,
                })
                .map(DataTypeAndValue::Integer),
            "boolean" => value
                .parse()
                .map_err(|e| ValueParseError::InvalidBoolean {
                    got: value.into(),
                    source: e,
                })
                .map(DataTypeAndValue::Boolean),
            "float" => value
                .parse()
                .map_err(|e| ValueParseError::float(value, e))
                .map(DataTypeAndValue::Float),
            "color3" => value.parse().map(DataTypeAndValue::Color3),
            "color4" => value.parse().map(DataTypeAndValue::Color4),
            "vector2" => value.parse().map(DataTypeAndValue::Vector2),
            "vector3" => value.parse().map(DataTypeAndValue::Vector3),
            "vector4" => value.parse().map(DataTypeAndValue::Vector4),
            "matrix33" => value.parse().map(DataTypeAndValue::Matrix3x3),
            "matrix44" => value.parse().map(DataTypeAndValue::Matrix4x4),
            "string" => Ok(DataTypeAndValue::String(value.to_string())),
            "filename" => Ok(DataTypeAndValue::Filename(value.to_string())),
            "integerarray" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map_err(|e| ValueParseError::InvalidInteger {
                    got: value.into(),
                    source: e,
                })
                .map(DataTypeAndValue::IntegerArray),
            "floatarray" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map_err(|e| ValueParseError::float(value, e))
                .map(DataTypeAndValue::FloatArray),
            "color3array" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map(DataTypeAndValue::Color3Array),
            "color4array" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map(DataTypeAndValue::Color4Array),
            "vector2array" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map(DataTypeAndValue::Vector2Array),
            "vector3array" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map(DataTypeAndValue::Vector3Array),
            "vector4array" => value
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<_, _>>()
                .map(DataTypeAndValue::Vector4Array),
            "stringarray" => Ok(DataTypeAndValue::StringArray(
                value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>(),
            )),
            _ => Ok(DataTypeAndValue::Unknown {
                tag: tag.into(),
                value: value.into(),
            }),
        }
    }

    pub fn tag(&self) -> DataType {
        match self {
            DataTypeAndValue::Integer(..) => DataType::Integer,
            DataTypeAndValue::Boolean(..) => DataType::Boolean,
            DataTypeAndValue::Float(..) => DataType::Float,
            DataTypeAndValue::Color3(..) => DataType::Color3,
            DataTypeAndValue::Color4(..) => DataType::Color4,
            DataTypeAndValue::Vector2(..) => DataType::Vector2,
            DataTypeAndValue::Vector3(..) => DataType::Vector3,
            DataTypeAndValue::Vector4(..) => DataType::Vector4,
            DataTypeAndValue::Matrix3x3(..) => DataType::Matrix3x3,
            DataTypeAndValue::Matrix4x4(..) => DataType::Matrix4x4,
            DataTypeAndValue::String(..) => DataType::String,
            DataTypeAndValue::Filename(..) => DataType::Filename,
            DataTypeAndValue::IntegerArray(..) => DataType::IntegerArray,
            DataTypeAndValue::FloatArray(..) => DataType::FloatArray,
            DataTypeAndValue::Color3Array(..) => DataType::Color3Array,
            DataTypeAndValue::Color4Array(..) => DataType::Color4Array,
            DataTypeAndValue::Vector2Array(..) => DataType::Vector2Array,
            DataTypeAndValue::Vector3Array(..) => DataType::Vector3Array,
            DataTypeAndValue::Vector4Array(..) => DataType::Vector4Array,
            DataTypeAndValue::StringArray(..) => DataType::StringArray,
            DataTypeAndValue::Unknown { tag, .. } => DataType::Unknown(tag.to_string()),
        }
    }
}

pub type Color3 = Vector3;
pub type Color4 = Vector4;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ValueParseError {
    #[error("Invalid length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("Invalid f64 value `{got}`: {source}")]
    InvalidFloat {
        got: String,
        source: std::num::ParseFloatError,
    },
    #[error("Invalid integer value `{got}`: {source}")]
    InvalidInteger {
        got: String,
        source: std::num::ParseIntError,
    },
    #[error("Invalid bool value `{got}`: {source}")]
    InvalidBoolean {
        got: String,
        source: std::str::ParseBoolError,
    },
}

impl ValueParseError {
    pub fn float(got: &str, source: std::num::ParseFloatError) -> Self {
        ValueParseError::InvalidFloat {
            got: got.into(),
            source,
        }
    }

    pub fn assert_length(expected: usize, actual: usize) -> Result<(), Self> {
        if expected != actual {
            Err(ValueParseError::InvalidLength { expected, actual })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector2(pub [f64; 2]);

impl FromStr for Vector2 {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<f64> = s
            .split(',')
            .map(|s| s.trim().parse().map_err(|e| ValueParseError::float(s, e)))
            .collect::<Result<_, _>>()?;
        ValueParseError::assert_length(2, v.len())?;
        Ok(Vector2([v[0], v[1]]))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3(pub [f64; 3]);

impl FromStr for Vector3 {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<f64> = s
            .split(',')
            .map(|s| s.trim().parse().map_err(|e| ValueParseError::float(s, e)))
            .collect::<Result<_, _>>()?;
        ValueParseError::assert_length(3, v.len())?;
        Ok(Vector3([v[0], v[1], v[2]]))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector4(pub [f64; 4]);

impl FromStr for Vector4 {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<f64> = s
            .split(',')
            .map(|s| s.trim().parse().map_err(|e| ValueParseError::float(s, e)))
            .collect::<Result<_, _>>()?;
        ValueParseError::assert_length(4, v.len())?;
        Ok(Vector4([v[0], v[1], v[2], v[3]]))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix3x3(pub [Vector3; 3]);

impl FromStr for Matrix3x3 {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<f64> = s
            .split(',')
            .map(|s| s.trim().parse().map_err(|e| ValueParseError::float(s, e)))
            .collect::<Result<_, _>>()?;
        ValueParseError::assert_length(9, v.len())?;
        Ok(Matrix3x3([
            Vector3([v[0], v[1], v[2]]),
            Vector3([v[3], v[4], v[5]]),
            Vector3([v[6], v[7], v[8]]),
        ]))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x4(pub [Vector4; 4]);

impl FromStr for Matrix4x4 {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<f64> = s
            .split(',')
            .map(|s| s.trim().parse().map_err(|e| ValueParseError::float(s, e)))
            .collect::<Result<_, _>>()?;
        ValueParseError::assert_length(16, v.len())?;
        Ok(Matrix4x4([
            Vector4([v[0], v[1], v[2], v[3]]),
            Vector4([v[4], v[5], v[6], v[7]]),
            Vector4([v[8], v[9], v[10], v[11]]),
            Vector4([v[12], v[13], v[14], v[15]]),
        ]))
    }
}
