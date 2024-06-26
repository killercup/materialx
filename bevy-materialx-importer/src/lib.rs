use bevy_pbr::StandardMaterial;
use materialx_parser::{ConversionError, Input, MaterialX, TopLevel};
use std::str::FromStr as _;

pub fn material_to_pbr(input: &str, material: Option<&str>) -> Result<StandardMaterial, Error> {
    let def = MaterialX::from_str(input)?;
    if def
        .materials
        .iter()
        .any(|m| matches!(m, TopLevel::NodeGraph { .. }))
    {
        return Err(Error::UnsupportedMaterialHasNodeGraph);
    }

    let mut material_name = String::new();

    let surface_material = match material {
        Some(m) => {
            material_name = m.to_string();
            def.materials
                .iter()
                .find(|m| match m {
                    materialx_parser::TopLevel::SurfaceMaterial { name, .. } => {
                        name == &material_name
                    }
                    _ => false,
                })
                .ok_or_else(|| Error::MaterialNotFound(material_name.to_string()))?
        }
        None => def
            .materials
            .iter()
            .find(|m| match m {
                materialx_parser::TopLevel::SurfaceMaterial { name, .. } => {
                    material_name.clone_from(name);
                    true
                }
                _ => false,
            })
            .ok_or_else(|| Error::NoMaterialDefined)?,
    };

    let standard_surface_name = match surface_material {
        materialx_parser::TopLevel::SurfaceMaterial { input, .. } if input.nodename.is_some() => {
            input.nodename.as_ref().unwrap()
        }
        _ => return Err(Error::SurfaceDefinitionNotFound(material_name.to_string())),
    };
    let standard_surface = def
        .materials
        .iter()
        .find(|n| match n {
            materialx_parser::TopLevel::StandardSurface { name, .. } => {
                name == standard_surface_name
            }
            _ => false,
        })
        .ok_or_else(|| Error::SurfaceDefinitionNotFound(material_name.to_string()))?;
    let standard_surface_values = match standard_surface {
        materialx_parser::TopLevel::StandardSurface { input, .. } => input,
        _ => return Err(Error::SurfaceDefinitionNotFound(material_name.to_string())),
    };

    let mut res = StandardMaterial::default();

    macro_rules! param {
        ($parameter:ident, $name:expr) => {
            if let Some($parameter) = get_param(standard_surface_values, $name)? {
                res.$parameter = $parameter;
            }
        };
    }
    param!(base_color, "base_color");
    param!(emissive, "emission_color");
    param!(perceptual_roughness, "specular_roughness");
    param!(metallic, "metalness");
    param!(reflectance, "specular");
    param!(diffuse_transmission, "base");
    param!(ior, "specular_IOR");
    param!(clearcoat, "coat");
    param!(clearcoat_perceptual_roughness, "coat_roughness");

    Ok(res)
}

/// Get parameter from list of [`Input`]s and convert it to the desired type
///
/// This works because [`Input`] has [TryFrom] implementations for usual bevy types
fn get_param<'a, T>(inputs: &'a [Input], name: &str) -> Result<Option<T>, Error>
where
    T: TryFrom<&'a Input, Error = ConversionError>,
{
    let Some(input) = inputs.iter().find(|i| i.name == name) else {
        return Ok(None);
    };
    Ok(Some(input.try_into()?))
}

#[derive(Debug, Clone, thiserror::Error)]
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
    InputConversionError(#[from] ConversionError),
}
