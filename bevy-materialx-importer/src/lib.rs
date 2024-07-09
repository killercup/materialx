#![doc = include_str!("../README.md")]

use bevy_app::{App, Plugin};
use bevy_asset::AssetApp as _;
use bevy_reflect::Reflect;
use materialx_parser::nodes::AccessError;
use smol_str::SmolStr;

mod standard_material;
pub use standard_material::material_to_pbr;
mod loader;
pub use loader::{MaterialX, MaterialXLoader};
use standard_material::MaterialError;

#[derive(Debug, Default, Clone, Reflect)]
pub struct MaterialXPlugin;

impl Plugin for MaterialXPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(MaterialXLoader);
        app.init_asset::<MaterialX>();
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("MaterialX error: {0}")]
    MaterialX(#[from] materialx_parser::Error),
    #[error("No material defined")]
    NoMaterialDefined,
    #[error("Material `{name}` not found")]
    MaterialNotFound {
        name: SmolStr,
        source: Box<AccessError>,
    },
    #[error("Failed to get element: {0}")]
    AccessError(#[from] AccessError),
    #[error("Currently not supported: {reason} (node {node})")]
    Unsupported { reason: String, node: SmolStr },
    #[error("Failed to read asset `{path}`: {source}")]
    FailedToReadAsset {
        path: String,
        source: std::io::Error,
    },
    #[error("Failed to build material {name}: {source}")]
    MaterialMapping {
        name: SmolStr,
        source: Box<MaterialError>,
    },
}
