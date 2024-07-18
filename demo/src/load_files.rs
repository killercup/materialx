use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result};
use bevy::{prelude::*, utils::HashMap};
use bevy_materialx_importer::{MaterialX, MaterialXPlugin};

pub struct LoadFilesPlugin;

impl Plugin for LoadFilesPlugin {
    fn build(&self, app: &mut App) {
        let filter = MaterialFilter(std::env::args().nth(1));

        app.add_plugins((MaterialXPlugin,))
            .insert_resource(filter)
            .register_type::<ExampleFiles>()
            .add_systems(Startup, (load_example_files,));
    }
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct MaterialFilter(Option<String>);

/// All the example materials by their name
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
#[allow(dead_code)]
struct ExampleFiles(HashMap<String, MaterialAsset>);

#[derive(Debug, Reflect)]
struct MaterialAsset {
    path: PathBuf,
    material: Handle<MaterialX>,
    meta: Option<Metadata>,
}

// Same as in downloader
#[derive(Debug, serde::Deserialize, Reflect)]
pub struct Metadata {
    pub source: String,
    pub name: String,
    pub id: String,
    pub url: String,
    pub preview_image: Option<String>,
}

fn load_example_files(
    mut commands: Commands,
    assets: Res<AssetServer>,
    filter: Res<MaterialFilter>,
) {
    let mut res = HashMap::new();

    let examples = glob::glob("assets/**/*.mtlx").unwrap();
    for example in examples {
        let path = example.unwrap();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        if let Some(filter) = &filter.0 {
            if !name.contains(filter) {
                continue;
            }
        }

        let meta = match load_meta_file(&path).context("load meta file") {
            Ok(meta) => meta,
            Err(e) => {
                log_err(&e);
                None
            }
        };

        res.insert(
            name,
            MaterialAsset {
                material: assets.load(path.strip_prefix("assets").unwrap().to_path_buf()),
                meta,
                path,
            },
        );
    }

    info!("found {} materials", res.len());

    commands.insert_resource(ExampleFiles(res));
}

fn load_meta_file(path: &Path) -> Result<Option<Metadata>> {
    let meta_path = path.with_file_name("meta.json");
    if meta_path.exists() {
        let file = std::fs::read_to_string(&meta_path)
            .with_context(|| format!("can't read {meta_path:?}"))?;
        let meta: Metadata =
            serde_json::from_str(&file).with_context(|| format!("can't parse {meta_path:?}"))?;
        Ok(Some(meta))
    } else {
        Ok(None)
    }
}

fn log_err(error: &anyhow::Error) {
    let mut source = format!("{error}");
    let mut e = error.source();
    while let Some(inner) = e {
        let _ = write!(&mut source, " > {inner}");
        e = inner.source();
    }
    error!("{source}");
}
