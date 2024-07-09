#![allow(clippy::single_match)]

use std::ops::Deref;

use bevy_asset::{AssetPath, AssetServer};
use bevy_pbr::StandardMaterial;
use materialx_parser::{
    ast::Element,
    data_types::{DataTypeAndValue, ValueParseError},
    nodes::{AccessError, InputData},
    wrap_node, GetAllByType, GetByTypeAndName as _, Input, MaterialX, Node,
};
use smol_str::SmolStr;
use tracing::{debug, instrument, warn};

use crate::Error;

pub fn material_to_pbr(
    def: &MaterialX,
    material: Option<&str>,
    path: &AssetPath,
    assets: &AssetServer,
) -> Result<StandardMaterial, Error> {
    let material = if let Some(name) = material {
        let name: SmolStr = name.into();
        def.get(name.clone()).map_err(|e| Error::MaterialNotFound {
            name,
            source: Box::new(e),
        })?
    } else {
        def.all::<surfacematerial>()
            .next()
            .ok_or(Error::NoMaterialDefined)?
    };

    let surface_input = material.get::<Input>("surfaceshader".into())?;
    let surface = def.get::<standard_surface>(match surface_input.data {
        InputData::NodeReference { node_name } => node_name.clone(),
        _ => {
            return Err(Error::Unsupported {
                reason: "Surface shader input must be a node reference".into(),
                node: surface_input.name.clone(),
            })
        }
    })?;

    build_material(&surface, &material, def, path, assets).map_err(|e| Error::MaterialMapping {
        name: material.name.clone(),
        source: Box::new(e),
    })
}

#[instrument(skip_all, fields(%material.name))]
fn build_material(
    surface: &standard_surface,
    material: &surfacematerial,
    def: &MaterialX,
    path: &AssetPath,
    assets: &AssetServer,
) -> Result<StandardMaterial, MaterialError> {
    let mut res = StandardMaterial::default();
    {
        match surface.get::<Input>("base_color".into()) {
            Ok(input) => match input.data {
                InputData::Value(val) => {
                    let val = DataTypeAndValue::from_tag_and_value(&input.r#type, &val)?;
                    res.base_color = val.try_into().unwrap();
                }
                InputData::NodeReference { node_name } => {
                    debug!("Found node ref to {node_name}");
                    if let Ok(tiled) = def.get::<tiledimage>(node_name) {
                        let filename = tiled.get::<Element>("file".into())?.attr("value")?;
                        let path = path.resolve_embed(&filename).unwrap();
                        res.base_color_texture = Some(assets.load(&path));
                        debug!("Loaded base color texture {path}");
                    }
                }
                _ => {}
            },
            Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
            Err(e) => return Err(MaterialError::from(e)),
        }
    }

    #[cfg(feature = "bevy/pbr_multi_layer_material_textures")]
    {
        match surface.get::<Input>("coat_roughness".into()) {
            Ok(input) => match input.data {
                InputData::NodeReference { node_name } => {
                    debug!("Found node ref to {node_name}");
                    if let Ok(tiled) = def.get::<tiledimage>(node_name) {
                        let filename = tiled.get::<Element>("file".into())?.attr("value")?;
                        let path = path.resolve_embed(&filename).unwrap();
                        res.clearcoat_roughness_texture = Some(assets.load(&path));
                        debug!("Loaded coat_roughness texture {path}");
                    }
                }
                _ => {}
            },
            Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
            Err(e) => return Err(MaterialError::from(e)),
        }
    }

    {
        match surface.get::<Input>("normal".into()) {
            Ok(input) => match input.data {
                InputData::NodeReference { node_name } => {
                    debug!("Found node ref to {node_name}");
                    if let Ok(normal) = def.get::<normalmap>(node_name) {
                        let input = normal.get::<Element>("in".into())?.attr("nodename")?;

                        if let Ok(tiled) = def.get::<tiledimage>(input) {
                            let filename = tiled.get::<Element>("file".into())?.attr("value")?;
                            let path = path.resolve_embed(&filename).unwrap();
                            res.normal_map_texture = Some(assets.load(&path));
                            debug!("Loaded normal texture {path}");
                        }
                    }
                }
                _ => {}
            },
            Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
            Err(e) => return Err(MaterialError::from(e)),
        }
    }

    {
        match material.get::<Input>("displacementshader".into()) {
            Ok(input) => match input.data {
                InputData::NodeReference { node_name } => {
                    debug!("Found node ref to {node_name}");
                    if let Ok(normal) = def.get::<displacement>(node_name) {
                        let input = normal
                            .get::<Element>("displacement".into())?
                            .attr("nodename")?;

                        if let Ok(tiled) = def.get::<tiledimage>(input) {
                            let filename = tiled.get::<Element>("file".into())?.attr("value")?;
                            let path = path.resolve_embed(&filename).unwrap();
                            res.depth_map = Some(assets.load(&path));
                            debug!("Loaded displacement {path}");
                        }
                    }
                }
                _ => {}
            },
            Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
            Err(e) => return Err(MaterialError::from(e)),
        }
    }

    match def.resolve_input::<f32>(surface, None, "base".into()) {
        Ok(x) => res.diffuse_transmission = 1.0 - x,
        Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
        Err(e) => return Err(MaterialError::from(e)),
    }

    macro_rules! set {
        ($field:ident, $input:expr) => {
            match def.resolve_input(&surface, None, $input.into()) {
                Ok(x) => res.$field = x,
                Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
                Err(e) => return Err(MaterialError::from(e)),
            }
        };
    }

    set!(emissive, "emissive");
    set!(perceptual_roughness, "specular_roughness");
    set!(metallic, "metalness");
    set!(reflectance, "specular");
    set!(clearcoat, "coat");
    set!(clearcoat_perceptual_roughness, "coat_roughness");

    Ok(res)
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MaterialError {
    #[error("Failed to parse input: {0}")]
    ParseInput(#[from] ValueParseError),
    #[error("Failed to get input: {0}")]
    GetInput(#[from] AccessError),
}

wrap_node!(surfacematerial);
wrap_node!(standard_surface);
wrap_node!(tiledimage);
wrap_node!(normalmap);
wrap_node!(displacement);
