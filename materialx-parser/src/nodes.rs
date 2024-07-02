//! Node definitions

use crate::{Input, Metadata};
use serde::{
    de::{value::StringDeserializer, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Deserialize)]
pub struct Node {
    // #[serde(rename = "$tag")]
    // pub tag: String,
    #[serde(flatten)]
    pub meta: Metadata,
    #[serde(default)]
    pub input: Vec<Input>,
}

// impl<'de> Deserialize<'de> for Node {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         // deserializer.deserialize_identifier(NodeVisitor);
//         Deserializer::deserialize_struct(self, name, fields, visitor);

//         todo!();
//     }
// }

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

macro_rules! nodes {
    ($(
        $name:ident(
            ($( $param:ident : $paramType:ident $(= $paramDefault:expr)? ),+)
            => $returnType:ident
        )
    ),*$(,)*) => {
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum ParsedNode {
            $(
                $name(nodes::$name),
            )*
            // #[serde(untagged)]
            // Manual(std::collections::HashMap<String, Vec<crate::Input>>),
            #[serde(other)]
            Unknown,
        }

        pub mod nodes {
            $(
                #[derive(Debug, serde::Deserialize)]
                #[serde(rename_all = "lowercase")]
                pub struct $name {
                    #[serde(flatten)]
                    pub meta: crate::Metadata,
                    pub input: crate::Inputs,
                }
            )*
        }
    };
}

nodes! {
    Add((in1: T, in2: T) => T),
    Subtract((in1: T, in2: T) => T),
    Multiply((in1: T, in2: T) => T),
    Divide((in1: T, in2: T) => T),
    Modulo((in1: T, in2: T) => T),
    Invert((r#in: T, amount: T = 1.0) => T),
    AbsVal((r#in: T) => T),
    Sign((r#in: T) => T),
    Floor((r#in: T) => T),
    Ceil((r#in: T) => T),
    Round((r#in: T) => T),
    Power((in1: T, in2: T = 1.0) => T),
    SafePower((in1: T, in2: T = 1.0) => T),
    Sin((r#in: T) => T),
    Cos((r#in: T) => T),
    Tan((r#in: T) => T),
    Asin((r#in: T) => T),
    Acos((r#in: T) => T),
    Atan2((inY: T = 0.0, inX: T = 1.0) => T),
    Sqrt((r#in: T) => T),
    Ln((r#in: T = 1.0) => T),
    Exp((r#in: T) => T),
    Clap((r#in: T, low: T = 0.0, high: T = 1.0) => T),
    TriangleWave((r#in: T) => T),
    Min((in1: T, in2: T) => T),
    Max((in1: T, in2: T) => T),
    And((in1: T, in2: T) => T),
    Or((in1: T, in2: T) => T),
    Not((r#in: T) => T),
    Normalize((r#in: T) => T),
    Magnitude((r#in: T) => T),
    Distance((in1: T, in2: T) => T),
    DotProduct((in1: T, in2: T) => T),
    CrossProduct((in1: T, in2: T) => T),
    TransformPoint((r#in: T, fromSpace: T, toSpace: T) => T),
    TransformVector((r#in: T, fromSpace: T, toSpace: T) => T),
    TransformMatrix((r#in: T, mat: T) => T),
    TransformColor((r#in: T, fromSpace: T, toSpace: T) => T),
    NormalMap((r#in: T, scale: T = 1.0, normal: T, tangent: T, bitangent: T) => T),
    Image((file: T, uvtiling: T) => T),
    TiledImage((file: T, uvtiling: T) => T),
}

mod manual {
    use crate::{Input, Metadata};
    use serde::Deserialize;

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
        Asin(Asin),
        Acos(Acos),
        Atan2(Atan2),
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
    pub struct Asin {
        #[serde(flatten)]
        pub meta: Metadata,
        pub input: Input,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub struct Acos {
        #[serde(flatten)]
        pub meta: Metadata,
        pub input: Input,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub struct Atan2 {
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
}
