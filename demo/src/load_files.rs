use bevy::prelude::*;
use bevy_materialx_importer::{MaterialX, MaterialXPlugin};

pub struct LoadFilesPlugin;

impl Plugin for LoadFilesPlugin {
    fn build(&self, app: &mut App) {
        let filter = MaterialFilter(std::env::args().nth(1));

        app.add_plugins((MaterialXPlugin,))
            .insert_resource(filter)
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
struct ExampleFiles(Vec<(String, Handle<MaterialX>)>);

fn load_example_files(
    mut commands: Commands,
    assets: Res<AssetServer>,
    filter: Res<MaterialFilter>,
) {
    let mut res = Vec::new();

    let examples = glob::glob("assets/**/*.mtlx").unwrap();
    for example in examples {
        let path = example.unwrap();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        if let Some(filter) = &filter.0 {
            if !name.contains(filter) {
                continue;
            }
        }

        res.push((
            name.clone(),
            assets.load(path.strip_prefix("assets").unwrap().to_path_buf()),
        ));
    }

    info!("found {} materials", res.len());

    commands.insert_resource(ExampleFiles(res));
}
