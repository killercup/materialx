// TODO: Add preprocessor to convert mtlx to standard material in some format (e.g. ron)

use crate::{material_to_pbr, standard_material::StandardMaterialTransformError};
use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy_pbr::StandardMaterial;
use bevy_reflect::TypePath;
use smol_str::SmolStr;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct MaterialXLoader;

#[derive(Debug, Asset, TypePath)]
pub struct MaterialX {
    pub file_name: Option<String>,
    pub material_name: Option<SmolStr>,
    pub material: StandardMaterial,
    pub source: materialx_parser::MaterialX,
}

impl AssetLoader for MaterialXLoader {
    type Asset = MaterialX;
    type Settings = ();
    type Error = LoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut res = String::new();
        reader
            .read_to_string(&mut res)
            .await
            .map_err(|e| LoaderError::FailedToReadAsset {
                path: load_context.path().to_string_lossy().to_string(),
                source: e,
            })?;
        let def = materialx_parser::MaterialX::from_str(&res)?;
        let path = load_context.asset_path().to_owned();
        let material_name = load_context.asset_path().label().map(|x| x.into());

        let material = material_to_pbr(&def, material_name.clone(), &path, load_context)?;

        Ok(MaterialX {
            file_name: path
                .path()
                .file_name()
                .map(|x| x.to_string_lossy().to_string()),
            material_name,
            material,
            source: def,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["mtlx"]
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LoaderError {
    #[error("MaterialX parsing error: {0}")]
    MaterialX(#[from] materialx_parser::Error),
    #[error("Failed to read asset `{path}`: {source}")]
    FailedToReadAsset {
        path: String,
        source: std::io::Error,
    },
    #[error("Failed to convert MaterialX to StandardMaterial: {0}")]
    FailedToConvertMaterialX(#[from] StandardMaterialTransformError),
}
