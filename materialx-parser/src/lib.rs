#![doc = include_str!("../README.md")]

pub mod ast;
pub mod data_types;
pub mod nodes;

pub use ast::{Element, MaterialX};
pub use nodes::{AccessError, GetAllByType, GetByTypeAndName, Input, InputData, Node};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Empty input")]
    Empty,
    #[error("Failed to parse XML")]
    Xml(#[from] roxmltree::Error),
    #[error("Failed to build structure from AST")]
    Ast(#[from] ast::AstError),
    #[error("Include elements are not supported yet")]
    IncludesNotSupported,
    #[error("Failed to access element")]
    Get(#[from] AccessError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr as _;

    #[test]
    fn principle() {
        let xml = r#"
            <materialx version="1.39">
                <child name="child-1">
                    <grandchild name="grandchild-1"/>
                </child>
                <child name="child-2">
                    <grandchild names="grandchild-2"/>
                </child>
            </materialx>
        "#;
        let _ = dbg!(MaterialX::from_str(xml));
    }

    #[test]
    fn debug_one() {
        let file = std::fs::read_to_string(
            "../resources/materialx-examples/StandardSurface/standard_surface_jade.mtlx",
        )
        .unwrap();
        let _ = dbg!(MaterialX::from_str(&file));
    }

    #[test]
    fn readme() {
        // use materialx_parser::{wrap_node, MaterialX};

        let mat = MaterialX::from_str(include_str!(
            "../../resources/materialx-examples/StandardSurface/standard_surface_jade.mtlx"
        ))
        .unwrap();

        wrap_node!(surfacematerial);
        wrap_node!(standard_surface);

        let first_material = mat.all::<surfacematerial>().next().unwrap();
        let nodes::InputData::NodeReference { node_name } = first_material
            .get::<Input>("surfaceshader".into())
            .unwrap()
            .data
        else {
            return;
        };
        let _surface = mat.get::<standard_surface>(node_name.clone()).unwrap();
    }

    // tries to parse all mtlx files in the examples folder
    #[test]
    fn all() {
        let examples = glob::glob("../resources/**/*.mtlx").unwrap();
        let mut failed = 0;
        for example in examples {
            let example = example.unwrap();
            let path = example.as_path();
            if path.is_dir() {
                continue;
            }
            if path.extension().unwrap() == "mtlx" {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let xml = std::fs::read_to_string(path).unwrap();

                match MaterialX::from_str(&xml) {
                    Ok(_) => println!("{name}: Success"),
                    Err(e) => {
                        if matches!(e, Error::IncludesNotSupported) {
                            println!("{name}: Includes not supported");
                            continue;
                        }
                        eprintln!("{name}: Failed {e:?}");
                        failed += 1;
                    }
                }
            }
        }

        if failed > 0 {
            panic!("{failed} example files failed to parse");
        }
    }
}
