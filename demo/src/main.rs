#![allow(clippy::type_complexity)]

use anyhow::{Context as _, Result};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::path::PathBuf;

mod balls;
mod camera;
mod load_files;

fn main() -> Result<()> {
    std::env::set_current_dir(workspace_dir().context("can't find workspace dir")?)
        .context("can't set current dir")?;

    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            // Debug window
            WorldInspectorPlugin::new(),
            // Basic setup and movement
            camera::CameraPlugin,
            // This plugin loads all the `.mtlx` files in the `assets` directory
            load_files::LoadFilesPlugin,
            // For every `.mtlx` file loaded, this plugin will spawn a new ball
            // with that material
            balls::BallPlugin,
        ))
        .run();
    Ok(())
}

fn workspace_dir() -> Result<PathBuf> {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .context("Can't run `cargo locate-project`")?
        .stdout;
    let output = std::str::from_utf8(&output)
        .context("path not utf8")?
        .trim();
    let cargo_path = std::path::Path::new(output);
    Ok(cargo_path
        .parent()
        .context("can't get parent dir of `Cargo.toml`")?
        .to_path_buf())
}
