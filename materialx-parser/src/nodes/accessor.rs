use super::Node;
use crate::{
    ast::{Element, MaterialX},
    data_types::ValueParseError,
};
use smol_str::SmolStr;

pub trait GetByTypeAndName {
    fn get<T>(&self, name: SmolStr) -> Result<T, AccessError>
    where
        T: Node;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AccessError {
    #[error("No element found with name `{name}`")]
    NotFound { name: SmolStr },
    #[error("Element `{name}` has wrong tag mismatch, expected `{expected}`, found `{found}`")]
    TagMismatch {
        name: SmolStr,
        expected: SmolStr,
        found: SmolStr,
    },
    #[error("Failed to convert element `{name}` to `{r#type}`")]
    ConversionError {
        name: SmolStr,
        r#type: &'static str,
        source: Box<AccessError>,
    },
    #[error("Failed to convert element `{name}` to `{r#type}`")]
    ValueParseError {
        name: SmolStr,
        r#type: &'static str,
        source: Box<ValueParseError>,
    },
    #[error("No value, node name, or interface name found for input `{name}`")]
    InputMissingData { name: SmolStr },
    #[error("No value found for input `{name}`")]
    InputMissingValue { name: SmolStr },
    #[error("Could not get `{child}` from `{parent}`")]
    SubElementAccess {
        child: SmolStr,
        parent: SmolStr,
        source: Box<AccessError>,
    },
}

impl GetByTypeAndName for MaterialX {
    fn get<T>(&self, name: SmolStr) -> Result<T, AccessError>
    where
        T: Node,
    {
        let elem = self
            .elements
            .get(&name)
            .ok_or_else(|| AccessError::NotFound { name: name.clone() })?;
        T::from_element(elem).map_err(|e| AccessError::ConversionError {
            name: name.clone(),
            r#type: std::any::type_name::<T>(),
            source: Box::new(e),
        })
    }
}

impl MaterialX {
    pub fn element(&self, name: impl Into<SmolStr>) -> Result<&Element, AccessError> {
        let name = name.into();
        self.elements
            .get(&name)
            .ok_or_else(|| AccessError::NotFound { name })
    }
}

impl GetByTypeAndName for Element {
    fn get<T>(&self, name: SmolStr) -> Result<T, AccessError>
    where
        T: Node,
    {
        let elem = self
            .children
            .get(&name)
            .ok_or_else(|| AccessError::NotFound { name: name.clone() })?;
        T::from_element(elem).map_err(|e| AccessError::ConversionError {
            name: name.clone(),
            r#type: std::any::type_name::<T>(),
            source: Box::new(e),
        })
    }
}

impl Element {
    pub fn attr(&self, name: impl Into<SmolStr>) -> Result<SmolStr, AccessError> {
        let name = name.into();
        self.attributes
            .get(&name)
            .cloned()
            .ok_or_else(|| AccessError::InputMissingData { name })
    }
}

pub trait GetAllByType {
    fn all<T>(&self) -> impl Iterator<Item = T>
    where
        T: Node;
}

impl GetAllByType for MaterialX {
    fn all<T>(&self) -> impl Iterator<Item = T>
    where
        T: Node,
    {
        self.elements
            .values()
            .filter_map(|element| T::from_element(element).ok())
    }
}

impl GetAllByType for Element {
    fn all<T>(&self) -> impl Iterator<Item = T>
    where
        T: Node,
    {
        self.children
            .values()
            .filter_map(|element| T::from_element(element).ok())
    }
}
