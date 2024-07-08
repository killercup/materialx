use indexmap::IndexMap;
use roxmltree::Document;
use smol_str::SmolStr;
use std::str::FromStr;

mod from;
mod meta;
pub use meta::{ColorSpace, Version};

#[derive(Debug)]
pub struct MaterialX {
    pub version: Version,
    pub colorspace: Option<ColorSpace>,
    pub elements: IndexMap<SmolStr, Element>,
}

impl FromStr for MaterialX {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Document::parse(s)?.try_into()
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: SmolStr,
    pub name: SmolStr,
    pub attributes: IndexMap<SmolStr, SmolStr>,
    pub children: IndexMap<SmolStr, Element>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AstError {
    #[error("Failed to build Ast")]
    Build {
        parent: SmolStr,
        index: usize,
        source: Box<AstError>,
    },
    #[error("No name attribute found")]
    NoName,
    #[error("Invalid version attribute on materialx element")]
    InvalidVersion(#[from] meta::VersionError),
}
