use crate::{material_to_pbr, MaterialX};
use bevy_asset::transformer::{AssetTransformer, TransformedAsset};
use bevy_pbr::StandardMaterial;

pub struct MaterialXToStandardMaterial;

impl AssetTransformer for MaterialXToStandardMaterial {
    type AssetInput = MaterialX;
    type AssetOutput = StandardMaterial;
    type Settings = ();
    type Error = super::StandardMaterialTransformError;

    async fn transform<'a>(
        &'a self,
        _asset: TransformedAsset<Self::AssetInput>,
        _settings: &'a Self::Settings,
    ) -> Result<TransformedAsset<Self::AssetOutput>, Self::Error> {
        // FIXME: How to get the material name from fragment?
        // FIXME: How to load images referred to by the MaterialX file?

        todo!()
    }
}
