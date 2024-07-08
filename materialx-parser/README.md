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
let mat = MaterialX::from_str("..")?;

// all returns iterator over nodes matching specified type
let first_material = mat.all<SurfaceMaterial>().first().unwrap();
// get fetches node by name and casts it to specified type
let surface = mat.get<StandardSurface>(first_material.get<Input>("surfaceshader").nodename);
// or: index operator returning most generic node type
// might be clever on Vec<Input> to return Input instead of Node
// decide: panic or return dummy node
let surface = mat.get<StandardSurface>(first_material["surfaceshader"].nodename);
```
