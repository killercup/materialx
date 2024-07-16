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

/// Newtype pattern for wrapping a node
///
/// This macro generates a newtype struct that wraps an `Element` and implements
/// the `Node` trait.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use materialx_parser::{wrap_node, MaterialX, GetAllByType};
/// # let xml = include_str!(
/// #   "../../../resources/materialx-examples/StandardSurface/standard_surface_jade.mtlx"
/// # );
/// let mat = MaterialX::from_str(xml)?;
/// wrap_node!(standard_surface);
/// let surfaces = mat.all::<standard_surface>();
/// # Ok::<(), materialx_parser::Error>(())
/// ```
#[macro_export]
macro_rules! wrap_node {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types, non_snake_case)]
        pub struct $name {
            element: $crate::Element,
        }

        impl $crate::Node for $name {
            const ELEMENT_NAME: Option<&'static str> = Some(stringify!($name));

            fn from_element(
                element: &$crate::Element,
            ) -> ::std::result::Result<Self, $crate::AccessError> {
                if let Some(expected) = Self::ELEMENT_NAME {
                    if element.tag != expected {
                        return Err($crate::AccessError::TagMismatch {
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

        impl ::std::ops::Deref for $name {
            type Target = $crate::Element;

            fn deref(&self) -> &Self::Target {
                &self.element
            }
        }
    };
}

/// Create a new node struct
///
/// This macro generates a struct that allows converting an `Element` into a
/// more specific representation.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use materialx_parser::{node, MaterialX, GetByTypeAndName};
/// # let xml = include_str!(
/// #   "../../../resources/materialx-examples/StandardSurface/standard_surface_jade.mtlx"
/// # );
/// let mat = MaterialX::from_str(xml)?;
/// node!(surfacematerial((surfaceshader: T) => material));
/// let surfaces = mat.get::<surfacematerial>("Jade".into())?;
/// # Ok::<(), materialx_parser::Error>(())
/// ```
#[macro_export]
macro_rules! node {
    ($name:ident(
        ($( $param:ident : $paramType:ident $(= $paramDefault:expr)? ),+)
        => $returnType:ident
    )) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types, non_snake_case)]
        pub struct $name {
            $(
                pub $param: $crate::Input,
            )*
        }

        impl $crate::Node for $name {
            const ELEMENT_NAME: Option<&'static str> = Some(stringify!($name));

            fn from_element(element: &$crate::Element) -> Result<Self, $crate::AccessError> {
                use $crate::GetByTypeAndName;

                if let Some(expected) = Self::ELEMENT_NAME {
                    if element.tag != expected {
                        return Err($crate::AccessError::TagMismatch {
                            name: element.name.clone(),
                            expected: expected.into(),
                            found: element.tag.clone(),
                        });
                    }
                }

                Ok(Self {
                    $(
                        $param: element
                            .get::<$crate::Input>(stringify!($param).into())
                            .map_err(|e| $crate::AccessError::SubElementAccess {
                                child: stringify!($param).into(),
                                parent: element.name.clone(),
                                source: ::std::boxed::Box::new(e),
                            })?,
                    )*
                })
            }
        }
    };
}
