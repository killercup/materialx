#![allow(clippy::single_match)]

use bevy_asset::{AssetPath, LoadContext};
use bevy_pbr::StandardMaterial;
use materialx_parser::{
    ast::Element,
    data_types::{DataTypeAndValue, ValueParseError},
    node,
    nodes::{AccessError, InputData},
    wrap_node, GetAllByType, GetByTypeAndName, Input, MaterialX,
};
use smol_str::SmolStr;
use tracing::{debug, instrument, trace, warn};
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

    MaterialBuilder::new()
        .field(
            Setter::new("base_color")
                .file(|m, file| m.base_color_texture = Some(file))
                .value(|m, val| {
                    m.base_color = val.try_into()?;
                    Ok(())
                }),
        )
        .field(Setter::new("normal").file(|m, file| m.normal_map_texture = Some(file)))
        .field(Setter::new("emissive").value(|m, val| {
            m.emissive = val.try_into()?;
            Ok(())
        }))
        .field(
            Setter::new("specular_roughness")
                // wrong format
                // .file(|m, file| m.metallic_roughness_texture = Some(file))
                .value(|m, val| {
                    m.perceptual_roughness = val.try_into()?;
                    Ok(())
                }),
        )
        .field(Setter::new("metalness").value(|m, val| {
            m.metallic = val.try_into()?;
            Ok(())
        }))
        .field(Setter::new("specular").value(|m, val| {
            m.reflectance = val.try_into()?;
            Ok(())
        }))
        .field(Setter::new("specular_IOR").value(|m, val| {
            m.ior = val.try_into()?;
            Ok(())
        }))
        .field(Setter::new("base").value(|m, val| {
            let val: f32 = val.try_into()?;
            m.diffuse_transmission = 1.0 - val;
            Ok(())
        }))
        .field(Setter::new("coat").value(|m, val| {
            m.clearcoat = val.try_into()?;
            Ok(())
        }))
        .field(Setter::new("coat_normal").file(|m, file| m.clearcoat_normal_texture = Some(file)))
        .field(
            Setter::new("coat_roughness")
                .file(|m, file| m.clearcoat_roughness_texture = Some(file))
                .value(|m, val| {
                    m.clearcoat_perceptual_roughness = val.try_into()?;
                    Ok(())
                }),
        )
        .build(&surface, &material, def, path, loader)
        .map_err(|e| Error::MaterialMapping {
            name: material.name.clone(),
            source: Box::new(e),
        })
}

struct MaterialBuilder {
    setters: Vec<Setter>,
}

impl MaterialBuilder {
    fn new() -> Self {
        Self { setters: vec![] }
    }

    fn field(mut self, setter: Setter) -> Self {
        self.setters.push(setter);
        self
    }

    #[instrument(skip_all, fields(%material.name))]
    fn build(
        self,
        surface: &standard_surface,
        material: &surfacematerial,
        def: &MaterialX,
        path: &AssetPath,
        loader: &mut LoadContext<'_>,
    ) -> Result<StandardMaterial, MaterialError> {
        let mut res = StandardMaterial::default();
        for setter in self.setters {
            let field = setter.field.clone();
            match resolve_input(def, surface, field.clone()) {
                Ok(DataTypeAndValue::Filename(filename)) => {
                    if let Some(f) = setter.file_handler {
                        let path = path.resolve_embed(&filename)?;
                        let handle = loader.load(&path);
                        f(&mut res, handle);
                    } else {
                        debug!(filename, %field, "unexpected file");
                    }
                }
                Ok(val) => {
                    if let Some(f) = setter.value_handler {
                        if let Err(error) = f(&mut res, val) {
                            warn!(%error, "failed to assign {field}");
                        }
                    } else {
                        debug!(?val, %field, "unexpected data");
                    }
                }
                Err(ResolveError::NotFound { name, parent }) => {
                    trace!(%name, %parent, "field {field} not found");
                }
                Err(error) => warn!(%error, "failed to get {field}"),
            };
        }
        Ok(res)
    }
}

struct Setter {
    field: SmolStr,
    value_handler: Option<
        Box<dyn FnOnce(&mut StandardMaterial, DataTypeAndValue) -> Result<(), MaterialError>>,
    >,
    file_handler:
        Option<Box<dyn FnOnce(&mut StandardMaterial, bevy_asset::Handle<bevy_image::Image>)>>,
}

impl Setter {
    fn new(field: impl Into<SmolStr>) -> Setter {
        Setter {
            field: field.into(),
            value_handler: None,
            file_handler: None,
        }
    }

    fn file(
        mut self,
        f: impl FnOnce(&mut StandardMaterial, bevy_asset::Handle<bevy_image::Image>) + 'static,
    ) -> Self {
        self.file_handler = Some(Box::new(f));
        self
    }

    fn value(
        mut self,
        f: impl FnOnce(&mut StandardMaterial, DataTypeAndValue) -> Result<(), MaterialError> + 'static,
    ) -> Self {
        self.value_handler = Some(Box::new(f));
        self
    }
}

#[instrument(level="trace", skip_all, fields(%el.name, field))]
fn resolve_input(
    def: impl GetByTypeAndName,
    el: &Element,
    field: impl Into<SmolStr>,
) -> Result<DataTypeAndValue, ResolveError> {
    let field = field.into();
    match el.get::<Input>(field.clone()) {
        Ok(input) => match input.data {
            InputData::OutputReference { nodegraph, output } => {
                let graph = def.get::<Element>(nodegraph)?;
                resolve_input(&graph, &graph, output).map_err(|e| ResolveError::Inner {
                    element: el.name.clone(),
                    field,
                    source: Box::new(e),
                })
            }
            InputData::NodeReference { node_name } => {
                let inner = def.get::<Element>(node_name)?;
                let value = match inner.tag.as_str() {
                    "tiledimage" | "image" => "file",
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
        Err(AccessError::NotFound { name, parent }) => Err(ResolveError::NotFound { name, parent }),
        Err(e) => Err(ResolveError::Inner {
            element: el.name.clone(),
            field,
            source: Box::new(ResolveError::from(e)),
        }),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("No element found with name `{name}` (in `{parent}`)")]
    NotFound { name: SmolStr, parent: SmolStr },
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
node!(tiledimage((file: T, uvtiling: T) => T));
node!(normalmap((r#in: T, scale: T) => T));
node!(displacement((displacement: T, scale: T) => T));

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
