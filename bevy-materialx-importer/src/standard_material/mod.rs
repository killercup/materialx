#![allow(clippy::single_match)]

use bevy_asset::{AssetPath, LoadContext};
use bevy_pbr::{StandardMaterial, UvChannel};
use materialx_parser::{
    ast::Element,
    data_types::{DataTypeAndValue, ValueParseError},
    node,
    nodes::{AccessError, InputData},
    wrap_node, GetAllByType, GetByTypeAndName as _, Input, MaterialX,
};
use smol_str::SmolStr;
use tracing::{debug, instrument, warn};
use StandardMaterialTransformError as Error;

mod processor;

pub fn material_to_pbr(
    def: &MaterialX,
    material: Option<SmolStr>,
    path: &AssetPath,
    loader: &mut LoadContext<'_>,
) -> Result<StandardMaterial, Error> {
    let material = if let Some(name) = material {
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

    build_material(&surface, &material, def, path, loader).map_err(|e| Error::MaterialMapping {
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
    loader: &mut LoadContext<'_>,
) -> Result<StandardMaterial, MaterialError> {
    let mut res = StandardMaterial::default();
    match resolve_input(def, surface, "base_color") {
        Ok(DataTypeAndValue::Filename(filename)) => {
            let path = path.resolve_embed(&filename)?;
            res.base_color_texture = Some(loader.load(&path));
        }
        Ok(val) => {
            res.base_color = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get base_color"),
    };
    match resolve_input(def, surface, "normal") {
        Ok(DataTypeAndValue::Filename(filename)) => {
            let path = path.resolve_embed(&filename)?;
            res.normal_map_texture = Some(loader.load(&path));
        }
        Ok(data) => debug!(?data, field = "normal", "unexpected data"),
        Err(error) => warn!(%error, "failed to get normal"),
    };
    match resolve_input(def, surface, "emissive") {
        Ok(val) => {
            res.emissive = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get emissive"),
    };
    match resolve_input(def, surface, "specular_roughness") {
        Ok(DataTypeAndValue::Filename(filename)) => {
            let path = path.resolve_embed(&filename)?;
            res.metallic_roughness_texture = Some(loader.load(&path));
        }
        Ok(val) => {
            res.perceptual_roughness = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get emissive"),
    };
    match resolve_input(def, surface, "metalness") {
        Ok(val) => {
            res.metallic = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get emissive"),
    };
    match resolve_input(def, surface, "specular") {
        Ok(val) => {
            res.reflectance = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get emissive"),
    };
    match resolve_input(def, surface, "specular_IOR") {
        Ok(val) => {
            res.ior = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get emissive"),
    };
    match resolve_input(def, surface, "base") {
        Ok(val) => {
            let val: f32 = val.try_into()?;
            res.diffuse_transmission = 1.0 - val;
        }
        Err(error) => warn!(%error, "failed to get base"),
    };
    match resolve_input(def, surface, "coat") {
        Ok(val) => {
            res.clearcoat = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get coat"),
    };
    match resolve_input(def, surface, "coat_normal") {
        Ok(DataTypeAndValue::Filename(filename)) => {
            let path = path.resolve_embed(&filename)?;
            res.clearcoat_normal_texture = Some(loader.load(&path));
        }
        Ok(data) => debug!(?data, field = "coat_normal", "unexpected data"),
        Err(error) => warn!(%error, "failed to get coat_normal"),
    };
    match resolve_input(def, surface, "coat_roughness") {
        Ok(val) => {
            res.clearcoat_perceptual_roughness = val.try_into()?;
        }
        Err(error) => warn!(%error, "failed to get coat_roughness"),
    };

    // {
    //     match surface.get::<Input>("base_color".into()) {
    //         Ok(input) => match input.data {
    //             InputData::Value(val) => {
    //                 let val = DataTypeAndValue::from_tag_and_value(&input.r#type, &val)?;
    //                 res.base_color = val.try_into()?;
    //             }
    //             InputData::NodeReference { node_name } => {
    //                 debug!("Found node ref to {node_name}");
    //                 if let Ok(tiled) = def.get::<tiledimage>(node_name) {
    //                     let filename = tiled.get::<Element>("file".into())?.attr("value")?;
    //                     let path = path.resolve_embed(&filename)?;
    //                     res.base_color_texture = Some(loader.load(&path));
    //                     debug!("Loaded base color texture {path}");
    //                 }
    //             }
    //             _ => {}
    //         },
    //         Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //         Err(e) => return Err(MaterialError::from(e)),
    //     }
    // }

    // #[cfg(feature = "bevy/pbr_multi_layer_material_textures")]
    // {
    //     match surface.get::<Input>("coat_roughness".into()) {
    //         Ok(input) => match input.data {
    //             InputData::NodeReference { node_name } => {
    //                 debug!("Found node ref to {node_name}");
    //                 if let Ok(tiled) = def.get::<tiledimage>(node_name) {
    //                     let filename = tiled.get::<Element>("file".into())?.attr("value")?;
    //                     let path = path.resolve_embed(&filename)?;
    //                     res.clearcoat_roughness_texture = Some(loader.load(&path));
    //                     debug!("Loaded coat_roughness texture {path}");
    //                 }
    //             }
    //             _ => {}
    //         },
    //         Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //         Err(e) => return Err(MaterialError::from(e)),
    //     }
    // }

    // {
    //     match surface.get::<Input>("normal".into()) {
    //         Ok(input) => match input.data {
    //             InputData::NodeReference { node_name } => {
    //                 debug!("Found node ref to {node_name}");
    //                 if let Ok(normal) = def.get::<normalmap>(node_name) {
    //                     let input = normal.r#in.nodename.ok_or(AccessError::InputMissingValue {
    //                         name: "nodename".into(),
    //                     })?;

    //                     if let Ok(tiled) = def.get::<tiledimage>(input) {
    //                         let filename = tiled.get::<Element>("file".into())?.attr("value")?;
    //                         let path = path.resolve_embed(&filename)?;
    //                         res.normal_map_texture = Some(loader.load(&path));
    //                         res.normal_map_channel = UvChannel::Uv0;
    //                         debug!("Loaded normal texture {path}");
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         },
    //         Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //         Err(e) => return Err(MaterialError::from(e)),
    //     }
    // }

    // {
    //     // .surface.specular_roughness --[name]-> .tiledimage[name=name].input[name="file"][value]
    //     match surface.get::<Input>("specular_roughness".into()) {
    //         Ok(input) => match input.data {
    //             InputData::NodeReference { node_name } => {
    //                 if let Ok(tiled) = def.get::<tiledimage>(node_name) {
    //                     let filename = tiled.get::<Element>("file".into())?.attr("value")?;
    //                     let path = path.resolve_embed(&filename)?;
    //                     res.metallic_roughness_texture = Some(loader.load(&path));
    //                     debug!("Loaded metallic_roughness_texture texture {path}");
    //                 }
    //             }
    //             _ => {}
    //         },
    //         Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //         Err(e) => return Err(MaterialError::from(e)),
    //     }
    // }

    // {
    // FIXME: Convert displacement to parallax mapping

    // match material.get::<Input>("displacementshader".into()) {
    //     Ok(input) => match input.data {
    //         InputData::NodeReference { node_name } => {
    //             debug!("Found node ref to {node_name}");
    //             if let Ok(normal) = def.get::<displacement>(node_name) {
    //                 let input = normal
    //                     .get::<Element>("displacement".into())?
    //                     .attr("nodename")?;

    //                 if let Ok(tiled) = def.get::<tiledimage>(input) {
    //                     let filename = tiled.get::<Element>("file".into())?.attr("value")?;
    //                     let path = path.resolve_embed(&filename)?;
    //                     res.depth_map = Some(loader.load(&path));
    //                     debug!("Loaded displacement {path}");
    //                 }
    //             }
    //         }
    //         _ => {}
    //     },
    //     Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //     Err(e) => return Err(MaterialError::from(e)),
    // }
    // }

    // match def.resolve_input::<f32>(surface, None, "base".into()) {
    //     Ok(x) => res.diffuse_transmission = 1.0 - x,
    //     Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //     Err(e) => return Err(MaterialError::from(e)),
    // }

    // macro_rules! set {
    //     ($field:ident, $input:expr) => {
    //         match def.resolve_input(&surface, None, $input.into()) {
    //             Ok(x) => res.$field = x,
    //             Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
    //             Err(e) => return Err(MaterialError::from(e)),
    //         }
    //     };
    // }

    // set!(emissive, "emissive");
    // set!(perceptual_roughness, "specular_roughness");
    // set!(metallic, "metalness");
    // set!(reflectance, "specular");
    // set!(ior, "specular_IOR");

    // set!(clearcoat, "coat"); // maybe needs converting
    // set!(clearcoat_perceptual_roughness, "coat_roughness");

    debug!(material=?res, "new material");

    Ok(res)
}

fn resolve_input(
    def: &MaterialX,
    el: &Element,
    field: impl Into<SmolStr>,
) -> Result<DataTypeAndValue, ResolveError> {
    let field = field.into();
    match el.get::<Input>(field.clone()) {
        Ok(input) => match input.data {
            InputData::NodeReference { node_name } => {
                debug!("Found node ref to {node_name}");
                let inner = def.get::<Element>(node_name)?;
                let value = match inner.tag.as_str() {
                    "tiledimage" => "file",
                    "normalmap" => "in",
                    "displacement" => "displacement",
                    _ => {
                        return Err(ResolveError::UnimplmenentedElement {
                            name: inner.name.clone(),
                            r#type: inner.tag,
                        })
                    }
                };
                resolve_input(def, &inner, value).map_err(|e| ResolveError::Inner {
                    element: el.name.clone(),
                    field,
                    source: Box::new(e),
                })
            }
            InputData::Value(x) => {
                DataTypeAndValue::from_tag_and_value(&input.r#type, &x).map_err(|e| {
                    ResolveError::from(AccessError::ValueParseError {
                        name: el.name.clone(),
                        r#type: "DataTypeAndValue",
                        source: Box::new(e),
                    })
                })
            }
            e => Err(ResolveError::UnexpectedInput(e)),
        },
        // Err(AccessError::NotFound { .. }) | Err(AccessError::Unimplemented(..)) => {}
        Err(e) => Err(ResolveError::Inner {
            element: el.name.clone(),
            field,
            source: Box::new(ResolveError::from(e)),
        }),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("Failed to resolve {element}.{field}: {source}")]
    Inner {
        element: SmolStr,
        field: SmolStr,
        source: Box<ResolveError>,
    },
    #[error("{0}")]
    Access(#[from] AccessError),
    #[error("Unexpected input {0:?}")]
    UnexpectedInput(InputData),
    #[error("Unimplmenented element {type} ({name})")]
    UnimplmenentedElement { name: SmolStr, r#type: SmolStr },
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MaterialError {
    #[error("Failed to parse input: {0}")]
    ParseInput(#[from] ValueParseError),
    #[error("Failed to resolve input: {0}")]
    GetInput(#[from] ResolveError),
    #[error("Failed to parse asset path: {0}")]
    ParseAssetPath(#[from] bevy_asset::ParseAssetPathError),
}

wrap_node!(surfacematerial);
wrap_node!(standard_surface);
// wrap_node!(tiledimage);
// wrap_node!(normalmap);
// wrap_node!(displacement);

node!(tiledimage((r#in: T, scale: T) => T));
node!(normalmap((r#in: T, scale: T) => T));
node!(displacement((r#in: T, scale: T) => T));

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StandardMaterialTransformError {
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
    #[error("Failed to build material {name}: {source}")]
    MaterialMapping {
        name: SmolStr,
        source: Box<MaterialError>,
    },
}
