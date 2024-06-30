use std::str::FromStr as _;

use bevy::{color::palettes::css::SANDY_BROWN, prelude::*};
use bevy_materialx_importer::material_to_pbr;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera_setup, spawn_ground, spawn_balls))
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5., 10., 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::from(SANDY_BROWN)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Sphere::new(0.3));

    let examples = example_files();
    dbg!(examples.len());

    let x_start = -2.5;
    let mut x = x_start;
    let mut y = 0.;

    for (index, (name, material)) in examples.into_iter().enumerate() {
        if index % 5 == 0 {
            x = x_start;
            y += 1.;
        } else {
            x += 1.;
        }
        commands.spawn((
            PbrBundle {
                mesh: sphere.clone(),
                material: materials.add(material),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            Name::from(name),
        ));
    }
}

fn example_files() -> Vec<(String, StandardMaterial)> {
    let mut res = Vec::new();

    let examples = glob::glob("resources/**/*.mtlx").unwrap();
    for example in examples {
        let path = example.unwrap();
        if path.is_dir() {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let xml = std::fs::read_to_string(&path).unwrap();
        let def = materialx_parser::MaterialX::from_str(&xml).unwrap();
        match material_to_pbr(&def, None) {
            Ok(t) => {
                res.push((name, t));
            }
            Err(e) => warn!("failed to parse {name}: {e:?}"),
        }
    }

    res
}
