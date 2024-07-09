use super::ValueParseError;
use std::str::FromStr;

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
