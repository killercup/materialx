use crate::ast::Element;
use std::fmt::Debug;

mod accessor;
mod input;
mod resolve;
pub mod standard_nodes;

pub use accessor::*;
pub use input::{Input, InputData};

pub trait Node: Sized + Debug {
    const ELEMENT_NAME: Option<&'static str> = None;

    fn from_element(elem: &Element) -> Result<Self, AccessError>;
}

impl Node for Element {
    fn from_element(element: &Element) -> Result<Self, AccessError> {
        Ok(element.clone())
    }
}

#[macro_export]
macro_rules! wrap_node {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            element: Element,
        }

        impl Node for $name {
            const ELEMENT_NAME: Option<&'static str> = Some(stringify!($name));

            fn from_element(element: &Element) -> Result<Self, AccessError> {
                if let Some(expected) = Self::ELEMENT_NAME {
                    if element.tag != expected {
                        return Err(AccessError::TagMismatch {
                            name: element.name.clone(),
                            expected: expected.into(),
                            found: element.tag.clone(),
                        });
                    }
                }

                Ok(Self {
                    element: element.clone(),
                })
            }
        }

        impl Deref for $name {
            type Target = Element;

            fn deref(&self) -> &Self::Target {
                &self.element
            }
        }
    };
}

#[macro_export]
macro_rules! node {
    ($name:ident(
        ($( $param:ident : $paramType:ident $(= $paramDefault:expr)? ),+)
        => $returnType:ident
    )) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            $(
                pub $param: Input,
            )*
        }

        impl Node for $name {
            const ELEMENT_NAME: Option<&'static str> = Some(stringify!($name));

            fn from_element(element: &Element) -> Result<Self, AccessError> {
                if let Some(expected) = Self::ELEMENT_NAME {
                    if element.tag != expected {
                        return Err(AccessError::TagMismatch {
                            name: element.name.clone(),
                            expected: expected.into(),
                            found: element.tag.clone(),
                        });
                    }
                }

                Ok(Self {
                    $(
                        $param: element
                            .get::<Input>(stringify!($param).into())
                            .map_err(|e| AccessError::SubElementAccess {
                                child: stringify!($param).into(),
                                parent: element.name.clone(),
                                source: Box::new(e),
                            })?,
                    )*
                })
            }
        }
    };
}
