// TODO: Load readme with example

pub mod ast;
pub mod data_types;
pub mod nodes;

pub use ast::MaterialX;
pub use nodes::{GetAllByType, GetByTypeAndName, Input, Node};

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
        dbg!(MaterialX::from_str(xml));
    }

    #[test]
    fn debug_one() {
        let file = std::fs::read_to_string(
            "../resources/materials/ambientCg/Bricks075A_1K-JPG.zip/Bricks075A_1K-JPG.mtlx",
        )
        .unwrap();
        dbg!(MaterialX::from_str(&file));
    }

    #[test]
    fn readme() {
        use crate::MaterialX;

        let file = std::fs::read_to_string(
            "../resources/materials/ambientCg/Bricks075A_1K-JPG.zip/Bricks075A_1K-JPG.mtlx",
        )
        .unwrap();
        let mat = MaterialX::from_str(&file).unwrap();

        mat.element("standard_surface");
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
                        if (matches!(e, Error::IncludesNotSupported)) {
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
