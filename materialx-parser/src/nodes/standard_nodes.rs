use super::{AccessError, GetByTypeAndName, Input, Node};
use crate::{ast::Element, node};

node!(add((in1: T, in2: T) => Add));
node!(subtract((in1: T, in2: T) => T));
node!(multiply((in1: T, in2: T) => T));
node!(divide((in1: T, in2: T) => T));
node!(modulo((in1: T, in2: T) => T));
node!(invert((r#in: T, amount: T = 1.0) => T));
node!(absval((r#in: T) => T));
node!(sign((r#in: T) => T));
node!(floor((r#in: T) => T));
node!(ceil((r#in: T) => T));
node!(round((r#in: T) => T));
node!(power((in1: T, in2: T = 1.0) => T));
node!(safepower((in1: T, in2: T = 1.0) => T));
node!(sin((r#in: T) => T));
node!(cos((r#in: T) => T));
node!(tan((r#in: T) => T));
node!(asin((r#in: T) => T));
node!(acos((r#in: T) => T));
node!(atan2((inY: T = 0.0, inX: T = 1.0) => T));
node!(sqrt((r#in: T) => T));
node!(ln((r#in: T = 1.0) => T));
node!(exp((r#in: T) => T));
node!(clap((r#in: T, low: T = 0.0, high: T = 1.0) => T));
node!(trianglewave((r#in: T) => T));
node!(min((in1: T, in2: T) => T));
node!(max((in1: T, in2: T) => T));
node!(and((in1: T, in2: T) => T));
node!(or((in1: T, in2: T) => T));
node!(not((r#in: T) => T));
node!(normalize((r#in: T) => T));
node!(magnitude((r#in: T) => T));
node!(distance((in1: T, in2: T) => T));
node!(dotproduct((in1: T, in2: T) => T));
node!(crossproduct((in1: T, in2: T) => T));
node!(transformpoint((r#in: T, fromSpace: T, toSpace: T) => T));
node!(transformvector((r#in: T, fromSpace: T, toSpace: T) => T));
node!(transformmatrix((r#in: T, mat: T) => T));
node!(transformcolor((r#in: T, fromSpace: T, toSpace: T) => T));
node!(normalmap((r#in: T, scale: T = 1.0, normal: T, tangent: T, bitangent: T) => T));
node!(image((file: T, uvtiling: T) => T));
node!(tiledimage((file: T, uvtiling: T) => T));

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn add() {
        let doc = roxmltree::Document::parse(
            r#"
            <add name="node_add_16" type="float">
                <input name="in1" type="float" value="12.3" />
                <input name="in2" type="float" value="0.1" />
            </add>"#,
        )
        .unwrap();
        let element: Element = dbg!(doc.root_element().try_into().unwrap());
        dbg!(add::from_element(&element).unwrap());
    }

    #[test]
    fn debug_add() {
        let file = std::fs::read_to_string(
            "../resources/materialx-examples/StandardSurface/standard_surface_brick_procedural.mtlx",
        )
        .unwrap();
        let material = crate::MaterialX::from_str(&file).unwrap();
        let nodegraph = dbg!(material.get::<Element>("NG_BrickPattern".into()).unwrap());
        let _add = dbg!(nodegraph.get::<add>("node_add_16".into())).unwrap();
    }
}
