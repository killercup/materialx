#![allow(clippy::type_complexity)]

use bevy::{
    core_pipeline::{fxaa::Fxaa, Skybox},
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
};
use bevy_easings::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};
use bevy_materialx_importer::{MaterialX, MaterialXPlugin};
use bevy_mod_picking::prelude::*;
use std::time::Duration;

fn main() {
    let filter = MaterialFilter(std::env::args().nth(1));

    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialXPlugin,
            EasingsPlugin,
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
            DefaultInspectorConfigPlugin,
            WorldInspectorPlugin::new(),
        ))
        .register_type::<Ball>()
        .register_type::<MaterialFilter>()
        .register_type::<Arrange>()
        .register_type::<ExampleFiles>()
        .insert_resource(Arrange {
            spacing: Vec3::ONE,
            current_index: 0,
        })
        .insert_resource(filter)
        .add_event::<HoverBall>()
        .add_event::<BlurBall>()
        .add_event::<SelectedBall>()
        .add_event::<DeselectedBalls>()
        .add_systems(
            Startup,
            (
                camera_setup,
                insert_sphere_mesh,
                load_example_files,
                spawn_info_text,
            ),
        )
        .add_systems(
            Update,
            (
                spawn_balls,
                move_camera,
                update_info_text.run_if(on_event::<HoverBall>()),
                reset_info_text.run_if(on_event::<BlurBall>()),
                select_ball.run_if(on_event::<SelectedBall>()),
                escape,
                deselect_balls
                    .run_if(on_event::<DeselectedBalls>())
                    .after(escape),
            ),
        )
        .run();
}

const CAMERA_START: Transform = Transform::from_translation(Vec3::new(-1.25, 2.25, 20.5));
const TRANSITION_DURATION: u64 = 500;

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct MaterialFilter(Option<String>);

#[derive(Component)]
struct Camera;

fn camera_setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: CAMERA_START.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Camera)
        .insert(EnvironmentMapLight {
            diffuse_map: assets.load("examples/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: assets.load("examples/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 5000.0,
        })
        .insert(Skybox {
            image: assets.load("examples/pisa_specular_rgb9e5_zstd.ktx2"),
            brightness: 5000.0,
        })
        // .insert(Skybox {
        //     image: assets.load("examples/DaySkyHDRI029A_2K-HDR.exr"),
        //     brightness: 5000.0,
        // })
        // .insert(ScreenSpaceReflectionsBundle::default())
        .insert(Fxaa::default())
        .insert(Name::from("Main Camera"));
}

fn move_camera(
    mut camera: Query<(Entity, &mut Transform), With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (_camera, mut transform) = camera.single_mut();
    if input.pressed(KeyCode::KeyW) {
        transform.translation = transform.translation + transform.local_y() * 0.1;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation = transform.translation - transform.local_y() * 0.1;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation = transform.translation - transform.local_x() * 0.1;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation = transform.translation + transform.local_x() * 0.1;
    }
    if input.pressed(KeyCode::KeyQ) {
        transform.rotation *= Quat::from_rotation_y(0.05);
    }
    if input.pressed(KeyCode::KeyE) {
        transform.rotation *= Quat::from_rotation_y(-0.05);
    }
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct Meshes {
    ball: Handle<Mesh>,
}

fn insert_sphere_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(Meshes {
        ball: meshes.add(SphereMeshBuilder::new(
            0.3,
            SphereKind::Ico { subdivisions: 42 },
        )),
    });
}

/// Arrange spheres in a spiral pattern around the origin
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct Arrange {
    spacing: Vec3,
    current_index: i32,
}

impl Arrange {
    /// Get next coordinate in the spiral pattern
    ///
    /// Adapted from <hhttps://stackoverflow.com/a/19287714>
    fn next(&mut self) -> Vec3 {
        let n = self.current_index as f32;
        self.current_index += 1;

        let r = (((n + 1.0).sqrt() - 1.) / 2.).floor() + 1.;
        let p = (8. * r * (r - 1.)) / 2.;
        let en = r * 2.;
        let a = (1. + n - p) % (r * 8.);
        match (a / (r * 2.)).floor() as i32 {
            0 => Vec3::new(a - r, -r, 0.) * self.spacing,
            1 => Vec3::new(r, (a % en) - r, 0.) * self.spacing,
            2 => Vec3::new(r - (a % en), r, 0.) * self.spacing,
            3 => Vec3::new(-r, r - (a % en), 0.) * self.spacing,
            _ => unreachable!(),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Ball {
    name: String,
    label: String,
}

fn spawn_balls(
    meshes: Res<Meshes>,
    materialx: Res<Assets<MaterialX>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<AssetEvent<MaterialX>>,
    mut arrange: ResMut<Arrange>,
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

        let position = arrange.next();
        let name = asset
            .file_name
            .clone()
            .unwrap_or_else(|| "MaterialX".to_string());
        commands.spawn((
            Name::from(name.as_str()),
            Ball {
                name,
                label: asset
                    .material_name
                    .as_ref()
                    .map(|x| x.to_string())
                    .unwrap_or_default(),
            },
            PbrBundle {
                mesh: meshes.ball.clone(),
                material: materials.add(asset.material.clone()),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            PickableBundle::default(),
            On::<Pointer<Over>>::send_event::<HoverBall>(),
            On::<Pointer<Out>>::send_event::<BlurBall>(),
            On::<Pointer<Click>>::send_event::<SelectedBall>(),
        ));
    }
}

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

#[derive(Event)]
struct DeselectedBalls;

#[derive(Event)]
struct HoverBall(Entity);

impl From<ListenerInput<Pointer<Over>>> for HoverBall {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        HoverBall(event.target)
    }
}

#[derive(Event)]
struct BlurBall;

impl From<ListenerInput<Pointer<Out>>> for BlurBall {
    fn from(_event: ListenerInput<Pointer<Out>>) -> Self {
        BlurBall
    }
}

#[derive(Event)]
struct SelectedBall(Entity);

impl From<ListenerInput<Pointer<Click>>> for SelectedBall {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        SelectedBall(event.target)
    }
}

fn select_ball(
    mut commands: Commands,
    mut events: EventReader<SelectedBall>,
    mut camera: Query<(Entity, &Transform), (With<Camera>, Without<Ball>)>,
    balls: Query<(&Name, &Transform), With<Ball>>,
) {
    for event in events.read() {
        let Ok((name, ball_transform)) = balls.get(event.0) else {
            warn!(entity=%event.0, "Selected ball not found");
            continue;
        };
        info!(ball = %name, "Selected ball");
        for (camera, transform) in camera.iter_mut() {
            let target = transform
                .with_translation(ball_transform.translation + Vec3::new(0.0, 0.0, 1.0))
                .looking_at(ball_transform.translation, Vec3::Y);
            commands.entity(camera).insert(transform.ease_to(
                target,
                EaseFunction::ExponentialIn,
                EasingType::Once {
                    duration: Duration::from_millis(TRANSITION_DURATION),
                },
            ));
        }
    }
}

fn escape(mut events: EventWriter<DeselectedBalls>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        events.send(DeselectedBalls);
    }
}

fn deselect_balls(
    mut commands: Commands,
    mut camera: Query<(Entity, &Transform), (With<Camera>, Without<Ball>)>,
) {
    let (camera, transform) = camera.single_mut();

    commands.entity(camera).insert(transform.ease_to(
        transform.with_translation(CAMERA_START.translation),
        EaseFunction::ExponentialIn,
        EasingType::Once {
            duration: Duration::from_millis(TRANSITION_DURATION),
        },
    ));
}

#[derive(Component)]
struct MaterialName;

fn spawn_info_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "\n",
            TextStyle {
                font_size: 36.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        MaterialName,
    ));
}

fn update_info_text(
    balls: Query<&Ball, With<Ball>>,
    mut events: EventReader<HoverBall>,
    mut text: Query<&mut Text, With<MaterialName>>,
) {
    for event in events.read() {
        if let Ok(ball) = balls.get(event.0) {
            text.single_mut().sections[0].value = format!("{}\n{}", ball.name, ball.label);
        }
    }
}

fn reset_info_text(mut text: Query<&mut Text, With<MaterialName>>) {
    text.single_mut().sections[0].value = "\n".to_string();
}
