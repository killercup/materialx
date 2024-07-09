use std::ops::Range;

use bevy::{
    color::palettes::css::SANDY_BROWN,
    core_pipeline::{fxaa::Fxaa, Skybox},
    input::mouse::MouseWheel,
    prelude::*,
};
use bevy_asset::ron::de;
use bevy_materialx_importer::{material_to_pbr, MaterialX, MaterialXLoader};
use bevy_pbr::ScreenSpaceReflectionsBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_asset_loader(MaterialXLoader)
        .init_asset::<bevy_materialx_importer::MaterialX>()
        .insert_resource(Arrange {
            spacing: Vec3::ONE,
            current_index: 0,
        })
        .add_systems(
            Startup,
            (
                camera_setup,
                // spawn_ground,
                insert_sphere_mesh,
                load_example_files,
            ),
        )
        .add_systems(Update, spawn_balls)
        .add_systems(Update, move_camera)
        .run();
}

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
            transform: Transform::from_translation(Vec3::new(-1.25, 2.25, 20.5))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
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
        .insert(ScreenSpaceReflectionsBundle::default())
        .insert(Fxaa::default());
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

// From Bevy's SSR example
fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_input: EventReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    // The speed of camera movement.
    const CAMERA_KEYBOARD_ZOOM_SPEED: f32 = 0.1;
    const CAMERA_KEYBOARD_ORBIT_SPEED: f32 = 0.02;
    const CAMERA_MOUSE_WHEEL_ZOOM_SPEED: f32 = 0.25;

    // We clamp camera distances to this range.
    const CAMERA_ZOOM_RANGE: Range<f32> = 1.0..20.0;

    let (mut distance_delta, mut theta_delta) = (0.0, 0.0);

    // Handle keyboard events.
    if keyboard_input.pressed(KeyCode::KeyW) {
        distance_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        distance_delta += CAMERA_KEYBOARD_ZOOM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        theta_delta += CAMERA_KEYBOARD_ORBIT_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        theta_delta -= CAMERA_KEYBOARD_ORBIT_SPEED;
    }

    // Handle mouse events.
    for mouse_wheel_event in mouse_wheel_input.read() {
        distance_delta -= mouse_wheel_event.y * CAMERA_MOUSE_WHEEL_ZOOM_SPEED;
    }

    // Update transforms.
    for mut camera_transform in cameras.iter_mut() {
        let local_z = camera_transform.local_z().as_vec3().normalize_or_zero();
        if distance_delta != 0.0 {
            camera_transform.translation = (camera_transform.translation.length() + distance_delta)
                .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end)
                * local_z;
        }
        if theta_delta != 0.0 {
            camera_transform
                .translate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, theta_delta));
            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

#[derive(Debug, Resource)]
struct Ball(Handle<Mesh>);

fn insert_sphere_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let x = meshes.add(Sphere::new(0.3));
    commands.insert_resource(Ball(x));
}

/// Arrange spheres in a spiral pattern around the origin
#[derive(Debug, Resource)]
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

fn spawn_balls(
    ball: Res<Ball>,
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
        let material = match material_to_pbr(&asset.0, None, &path.unwrap_or_default(), &assets) {
            Ok(m) => m,
            Err(error) => {
                warn!(%error, path=?assets.get_path(*id), "Failed to convert MaterialX to StandardMaterial");
                continue;
            }
        };
        let position = arrange.next();
        commands.spawn((
            PbrBundle {
                mesh: ball.0.clone(),
                material: materials.add(material),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Name::from("MaterialX"),
        ));
    }
}

// #[derive(Debug, Resource)]
// struct ExampleFiles(Handle<LoadedFolder>);

// fn load_materials(mut commands: Commands, assets: Res<AssetServer>) {
//     let examples = assets.load_folder("../../resources/materialx-examples/StandardSurface");
//     commands.insert_resource(ExampleFiles(examples));
// }

#[derive(Debug, Resource)]
struct ExampleFiles(Vec<(String, Handle<MaterialX>)>);

fn load_example_files(mut commands: Commands, assets: Res<AssetServer>) {
    let mut res = Vec::new();

    let examples = glob::glob("assets/**/*.mtlx").unwrap();
    for example in examples {
        let path = example.unwrap();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        res.push((
            name.clone(),
            assets.load(path.strip_prefix("assets").unwrap().to_path_buf()),
        ));
    }

    info!("found {} materials", res.len());

    commands.insert_resource(ExampleFiles(res));
}
