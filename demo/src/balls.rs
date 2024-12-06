use std::time::Duration;

use bevy::{
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
};
use bevy_easings::{Ease as _, *};
use bevy_materialx_importer::MaterialX;

use crate::camera::CAMERA_START;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EasingsPlugin::default())
            .register_type::<Ball>()
            .register_type::<Arrange>()
            .insert_resource(Arrange {
                spacing: Vec3::ONE,
                current_index: 0,
            })
            .add_event::<HoverBall>()
            .add_event::<BlurBall>()
            .add_event::<SelectedBall>()
            .add_event::<DeselectedBalls>()
            .add_systems(Startup, (insert_sphere_mesh, spawn_info_text))
            .add_systems(
                Update,
                (
                    spawn_balls,
                    update_info_text.run_if(on_event::<HoverBall>),
                    reset_info_text.run_if(on_event::<BlurBall>),
                    select_ball.run_if(on_event::<SelectedBall>),
                    escape,
                    deselect_balls
                        .run_if(on_event::<DeselectedBalls>)
                        .after(escape),
                ),
            );
    }
}

const TRANSITION_DURATION: u64 = 500;

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct Meshes {
    ball: Handle<Mesh>,
}

fn insert_sphere_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let asset = SphereMeshBuilder::new(0.3, SphereKind::Ico { subdivisions: 42 });
    let mesh = asset.build().with_generated_tangents().unwrap();
    commands.insert_resource(Meshes {
        ball: meshes.add(mesh),
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
        commands
            .spawn((
                Name::from(name.as_str()),
                Ball {
                    name,
                    label: asset
                        .material_name
                        .as_ref()
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                },
                Transform::from_translation(position),
                MeshMaterial3d(materials.add(asset.material.clone())),
                Mesh3d(meshes.ball.clone()),
            ))
            .observe(
                |trigger: Trigger<Pointer<Over>>, mut events: EventWriter<HoverBall>| {
                    events.send(HoverBall(trigger.event().target));
                },
            )
            .observe(
                |_trigger: Trigger<Pointer<Out>>, mut events: EventWriter<BlurBall>| {
                    events.send(BlurBall);
                },
            )
            .observe(
                |mut trigger: Trigger<Pointer<Click>>, mut events: EventWriter<SelectedBall>| {
                    trigger.propagate(false);
                    let click_event: &Pointer<Click> = trigger.event();
                    events.send(SelectedBall(click_event.target));
                },
            )
            .observe(
                |trigger: Trigger<Pointer<Drag>>, mut e: Query<&mut Transform>| {
                    if let Ok(mut transform) = e.get_mut(trigger.event().target) {
                        transform.rotate_y(trigger.delta.x / 200.);
                        transform.rotate_x(trigger.delta.y / 200.);
                    }
                },
            );
    }
}

#[derive(Event)]
struct DeselectedBalls;

#[derive(Event)]
struct HoverBall(Entity);

#[derive(Event)]
struct BlurBall;

#[derive(Event)]
struct SelectedBall(Entity);

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
                bevy_easings::EaseFunction::ExponentialIn,
                bevy_easings::EasingType::Once {
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
        bevy_easings::EaseFunction::ExponentialIn,
        EasingType::Once {
            duration: Duration::from_millis(TRANSITION_DURATION),
        },
    ));
}

#[derive(Component)]
struct MaterialName;

fn spawn_info_text(mut commands: Commands) {
    commands.spawn((
        MaterialName,
        Text::new(""),
        TextFont::from_font_size(36.0),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}

fn update_info_text(
    balls: Query<&Ball, With<Ball>>,
    mut events: EventReader<HoverBall>,
    mut text: Query<&mut Text, With<MaterialName>>,
) {
    for event in events.read() {
        if let Ok(ball) = balls.get(event.0) {
            text.single_mut().0 = format!("{}\n{}", ball.name, ball.label);
        }
    }
}

fn reset_info_text(mut text: Query<&mut Text, With<MaterialName>>) {
    text.single_mut().0 = "\n".to_string();
}
