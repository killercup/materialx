#![doc = include_str!("../README.md")]

use bevy_app::{App, Plugin};
use bevy_asset::AssetApp as _;
use bevy_reflect::Reflect;

mod standard_material;
pub use standard_material::material_to_pbr;

mod loader;
pub use loader::{MaterialX, MaterialXLoader};

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
    #[error("Material not found")]
    MaterialNotFound(String),
    #[error("Surface definition not found")]
    SurfaceDefinitionNotFound(String),
    #[error(
        "MaterialX contains a node graph definition which not supported by bevy's StandardMaterial"
    )]
    UnsupportedMaterialHasNodeGraph,
    #[error("Conversion failed: {0}")]
    InputConversionError(#[from] materialx_parser::ConversionError),
    #[error("Failed to read asset `{path}`: {source}")]
    FailedToReadAsset {
        path: String,
        source: std::io::Error,
    },
}
