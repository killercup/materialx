use indexmap::IndexMap;
use roxmltree::Document;
use smol_str::SmolStr;
use std::str::FromStr;

mod from;
mod meta;
pub use meta::{ColorSpace, Version};

#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct MaterialX {
    pub version: Version,
    pub colorspace: Option<ColorSpace>,
    #[cfg_attr(feature = "bevy", reflect(ignore))] // FIXME: bevy_reflect doesn't support IndexMap
    pub elements: IndexMap<SmolStr, Element>,
}

impl MaterialX {
    pub(crate) const NAME: SmolStr = SmolStr::new_inline("<root>");
}

impl FromStr for MaterialX {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Document::parse(s)?.try_into()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct Element {
    pub tag: SmolStr,
    pub name: SmolStr,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    // FIXME: bevy_reflect doesn't support IndexMap -- also it'd be recursive
    pub attributes: IndexMap<SmolStr, SmolStr>,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    // FIXME: bevy_reflect doesn't support IndexMap -- also it'd be recursive
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
