use bevy_pbr::StandardMaterial;
use materialx_parser::{ConversionError, Input, Inputs, MaterialX, TopLevel};

use crate::Error;

pub fn material_to_pbr(def: &MaterialX, material: Option<&str>) -> Result<StandardMaterial, Error> {
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
        materialx_parser::TopLevel::SurfaceMaterial { input, .. }
            if input
                .get("surfaceshader")
                .and_then(|x| x.nodename.as_ref())
                .is_some() =>
        {
            input
                .get("surfaceshader")
                .and_then(|x| x.nodename.as_ref())
                .unwrap()
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
fn get_param<'a, T>(inputs: &'a Inputs, name: &str) -> Result<Option<T>, Error>
where
    T: TryFrom<&'a Input, Error = ConversionError>,
{
    let Some(input) = inputs.get(name) else {
        return Ok(None);
    };
    Ok(Some(input.try_into()?))
}
