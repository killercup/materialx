//! Node definitions

// TODO: Refactor node definitions to use a macro and well-typed inputs
//
// E.g. given this description
// > add: add a value to the incoming float/color/vector/matrix, or, add one
// > integer value to another.
// > - in1 (float or colorN or vectorN or matrixNN, or integer): the value or
// >   nodename for the primary input; for matrix types, the default is the
// >   zero matrix.
// > - in2 (same type as in1 or float, or integer): the value or nodename to
// >   add; for matrix types, the default is the zero matrix.
//
// we should translate it to something like
// ```rust,ignore
// node!(Add,
//     "add a value to the incoming float/color/vector/matrix, or, add one integer value to another",
//     in1: (Float | Color | Vector | Matrix | Integer),
//     in2: (Float | Color | Vector | Matrix | Integer)
// );
// ```
//
// and maybe also group it nicely in modules to then have one index enum/macro
// to collect all the nodes.

use serde::Deserialize;

use crate::{Input, Metadata};

// TODO: Add all the other nodes
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Node {
    Constant(Constant),
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Modulo(Modulo),
    Invert(Invert),
    AbsVal(AbsVal),
    Sign(Sign),
    Floor(Floor),
    Ceil(Ceil),
    Round(Round),
    Power(Power),
    SafePower(SafePower),
    Sin(Sin),
    Cos(Cos),
    Tan(Tan),
    ASin(ASin),
    ACos(ACos),
    ATan2(ATan2),
    Sqrt(Sqrt),
    Ln(Ln),
    Exp(Exp),
    Clap(Clap),
    TriangleWave(TriangleWave),
    Min(Min),
    Max(Max),
    And(And),
    Or(Or),
    Not(Not),
    Normalize(Normalize),
    Magnitude(Magnitude),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Constant {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Add {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Subtract {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Multiply {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Divide {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Modulo {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Invert {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AbsVal {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Sign {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Floor {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Ceil {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Round {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Power {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct SafePower {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Sin {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Cos {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Tan {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ASin {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ACos {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ATan2 {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Sqrt {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Ln {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Exp {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Clap {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct TriangleWave {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Min {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Max {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct And {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Or {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: (Input, Input),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Not {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Normalize {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Magnitude {
    #[serde(flatten)]
    pub meta: Metadata,
    pub input: Input,
}
