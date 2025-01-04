use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::tailwind,
    core::FrameCount,
    input::mouse::AccumulatedMouseMotion,
    pbr::NotShadowCaster,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation},
        view::RenderLayers,
        RenderPlugin,
    },
};

const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Craft".to_string(),
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(
                        bevy::render::settings::WgpuSettings {
                            backends: Some(Backends::VULKAN),
                            ..default()
                        },
                    ),
                    ..default()
                }),
        )
        .add_systems(
            Startup,
            (spawn_view_models, spawn_world_models, spawn_lights),
        )
        .add_systems(Update, (make_visible, move_player))
        .run();
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}

fn spawn_view_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_matieral = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));

            parent.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_matieral),
                Transform::from_xyz(0.2, -0.1, -0.25),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                NotShadowCaster,
            ));
        });
}

fn spawn_world_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let Ok((mut transform, camera_sensitivity)) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    // no need to multiply by deltatime because accumulated_mouse_motion is measured at a per frame level
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
