use super::{accessor::AccessError, Node};
use crate::{ast::Element, data_types::DataTypeAndValue};
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct Input {
    pub name: SmolStr,
    pub r#type: SmolStr,
    pub data: InputData,
    pub output: Option<SmolStr>,
    pub color_space: Option<SmolStr>,
}

impl Node for Input {
    fn from_element(element: &Element) -> Result<Self, AccessError> {
        let data = InputData::from_element(element).map_err(|e| AccessError::ConversionError {
            name: element.name.clone(),
            r#type: "InputData",
            source: Box::new(e),
        })?;
        Ok(Self {
            name: element.name.clone(),
            r#type: element.attr("type")?,
            data,
            output: element.attr("output").ok(),
            color_space: element.attr("colorspace").ok(),
        })
    }
}

impl TryFrom<&Input> for DataTypeAndValue {
    type Error = AccessError;

    fn try_from(value: &Input) -> Result<Self, Self::Error> {
        let InputData::Value(data) = &value.data else {
            return Err(AccessError::InputMissingValue {
                name: value.name.clone(),
            });
        };
        DataTypeAndValue::from_tag_and_value(&value.r#type, data).map_err(|e| {
            AccessError::ValueParseError {
                name: value.name.clone(),
                r#type: "DataTypeAndValue",
                source: Box::new(e),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum InputData {
    Value(SmolStr),
    NodeReference { node_name: SmolStr },
    InputReference { interface_name: SmolStr },
    OutputReference { nodegraph: SmolStr, output: SmolStr },
}

impl Node for InputData {
    fn from_element(e: &Element) -> Result<Self, AccessError> {
        if let Ok(value) = e.attr("value") {
            Ok(InputData::Value(value))
        } else if let Ok(node_name) = e.attr("nodename") {
            Ok(InputData::NodeReference { node_name })
        } else if let Ok(interface_name) = e.attr("interface_name") {
            Ok(InputData::InputReference { interface_name })
        } else if let (Ok(nodegraph), Ok(output)) = (e.attr("nodegraph"), e.attr("output")) {
            Ok(InputData::OutputReference { nodegraph, output })
        } else {
            Err(AccessError::InputMissingData {
                name: e.name.clone(),
            })
        }
    }
}
