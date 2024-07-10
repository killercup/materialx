#![doc = include_str!("../README.md")]

use bevy_app::{App, Plugin};
use bevy_asset::{processor::LoadTransformAndSave, AssetApp as _};
use bevy_reflect::Reflect;
use materialx_parser::nodes::AccessError;
use smol_str::SmolStr;

pub(crate) mod standard_material;
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
