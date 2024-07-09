use super::{AccessError, InputData};
use crate::{
    ast::Element,
    data_types::{DataTypeAndValue, ValueParseError},
    GetByTypeAndName as _, Input, MaterialX,
};
use smol_str::SmolStr;
use std::any::type_name;

impl MaterialX {
    // FIXME: Access nodes that are siblings of the current node using node_name
    pub fn resolve_input<T>(
        &self,
        element: &Element,
        parent: Option<&Element>,
        name: SmolStr,
    ) -> Result<T, AccessError>
    where
        T: TryFrom<DataTypeAndValue, Error = ValueParseError>,
    {
        let input = element.get::<Input>(name.clone())?;
        match input.data {
            InputData::Value(x) => Ok(DataTypeAndValue::from_tag_and_value(&input.r#type, &x)
                .map_err(|e| AccessError::ValueParseError {
                    name: name.clone(),
                    r#type: "DataTypeAndValue",
                    source: Box::new(e),
                })?
                .try_into()
                .map_err(|e| AccessError::InputConvertError {
                    name: input.name.clone(),
                    parent: name.clone(),
                    r#type: type_name::<T>(),
                    source: e,
                }))?,
            InputData::NodeReference { node_name } => {
                Err(AccessError::Unimplemented("following node references"))
            }
            InputData::InputReference { interface_name } => {
                Err(AccessError::Unimplemented("following input references"))
            }
            InputData::OutputReference { nodegraph, output } => {
                Err(AccessError::Unimplemented("following output references"))
            }
        }
    }
}
