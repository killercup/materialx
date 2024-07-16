# Load MaterialX files in Bevy

This crate adds support to MaterialX (`.mtlx`) files as in Bevy.

## Current Status

Some basic functions work,
but most features are not implemented yet.

## Example

```rust,no_run
use bevy::prelude::*;
use bevy_materialx_importer::{MaterialXPlugin, MaterialX, MaterialXLoader};

App::new()
    .add_plugins((DefaultPlugins, MaterialXPlugin))
    .add_systems(Startup, load_jade);

#[derive(Debug, Resource)]
struct Jade(Handle<MaterialX>);

fn load_jade(mut commands: Commands, assets: Res<AssetServer>) {
    let mat = assets.load("materialx-examples/StandardSurface/standard_surface_jade.mtlx");
    commands.insert_resource(Jade(mat));
}

fn spawn_ball(
    materialx: Res<Assets<MaterialX>>,
    assets: Res<AssetServer>,
    mut events: EventReader<AssetEvent<MaterialX>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };
        let path = assets.get_path(*id);
        let Some(asset) = materialx.get(*id) else {
            warn!(?id, ?path, "MaterialX asset not found");
            continue;
        };

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.3)),
                material: materials.add(asset.material.clone()),
                ..Default::default()
            },
        ));
    }
}
```
