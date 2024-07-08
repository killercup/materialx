use crate::ast::Element;

mod accessor;
mod input;
pub mod standard_nodes;

pub use accessor::*;
pub use input::Input;

pub trait Node: Sized {
    fn from_element(elem: &Element) -> Result<Self, AccessError>;
}

impl Node for Element {
    fn from_element(element: &Element) -> Result<Self, AccessError> {
        Ok(element.clone())
    }
}
