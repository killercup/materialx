use bevy::{color::palettes::css::SANDY_BROWN, prelude::*};
use bevy_materialx_importer::{material_to_pbr, MaterialX, MaterialXLoader};

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
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if let Some(asset) = materialx.get(*id) {
                let material = match material_to_pbr(&asset.0, None) {
                    Ok(m) => m,
                    Err(error) => {
                        warn!(?error, path=?assets.get_path(*id), "Failed to convert MaterialX to StandardMaterial");
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
            } else {
                warn!(?id, path=?assets.get_path(*id), "MaterialX asset not found");
            }
        }
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

    commands.insert_resource(ExampleFiles(res));
}
