# Parser for MaterialX (`.mtlx`) Files

This Rust library provides a parser for [MaterialX](https://materialx.org) files.
It is part of a project to provide MaterialX support for the [Bevy](https://bevyengine.org/) game engine.

## Current Status

Can parse MaterialX files and convert them to a Rust struct.

### Missing

- Includes

## Usage

Somewhat like this:

```rust
use std::str::FromStr;
use materialx_parser::{
    MaterialX, Error, wrap_node,
    Input, InputData, GetByTypeAndName, GetAllByType
};

fn main() -> Result<(), Error> {
    let mat = MaterialX::from_str(include_str!(
        "../assets/materialx-examples/StandardSurface/standard_surface_jade.mtlx"
    ))?;

    wrap_node!(surfacematerial);
    wrap_node!(standard_surface);

    let first_material = mat.all::<surfacematerial>().next().unwrap();
    let InputData::NodeReference { node_name } = first_material
        .get::<Input>("surfaceshader".into())?
        .data
    else {
        return Ok(());
    };
    let surface = mat.get::<standard_surface>(node_name.clone())?;
    Ok(())
}
```
