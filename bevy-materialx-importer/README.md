# Load MaterialX files in Bevy

This crate adds support to MaterialX (`.mtlx`) files as in Bevy.

## Current Status

Some basic functions work,
but most features are not implemented yet.

## Example

```rust,norun
use bevy::prelude::*;
use bevy_materialx_importer::MaterialXPlugin;

App::new()
    .add_plugins((DefaultPlugins, MaterialXPlugin));
```
