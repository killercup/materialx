// TODO: Add preprocessor to convert mtlx to standard material in some format (e.g. ron)

use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy_reflect::TypePath;
use std::str::FromStr;

use crate::Error;

#[derive(Debug, Default)]
pub struct MaterialXLoader;

#[derive(Debug, Asset, TypePath)]
pub struct MaterialX(pub materialx_parser::MaterialX);

impl AssetLoader for MaterialXLoader {
    type Asset = MaterialX;
    type Settings = ();
    type Error = crate::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<MaterialX, Self::Error> {
        let mut res = String::new();
        reader
            .read_to_string(&mut res)
            .await
            .map_err(|e| Error::FailedToReadAsset {
                path: load_context.path().to_string_lossy().to_string(),
                source: e,
            })?;
        let value = materialx_parser::MaterialX::from_str(&res)?;
        Ok(MaterialX(value))
    }

    fn extensions(&self) -> &[&str] {
        &["mtlx"]
    }
}
