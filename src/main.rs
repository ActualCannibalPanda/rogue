use bevy::{
    color::palettes::tailwind,
    core::FrameCount,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation},
        RenderPlugin,
    },
    window::WindowResolution,
};

#[derive(Debug, Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rogue".to_string(),
                        visible: false,
                        resolution: WindowResolution::new(800.0, 600.0)
                            .with_scale_factor_override(3.0),
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
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, (spawn_player))
        .add_systems(Update, (make_visible))
        .run();
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let handle = asset_server.load("sprites/minirogue.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 13, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(Camera2d);
    for y in 0..10 {
        if y == 0 || y == 9 {
            let y = y as f32;
            for x in 0..10 {
                let x = x as f32;
                commands.spawn((
                    Sprite::from_atlas_image(
                        handle.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 1,
                        },
                    ),
                    Player,
                    Transform::from_xyz(x * 8.0, 1.0 + y * 8.0, 0.0),
                    Visibility::default(),
                ));
            }
        } else {
            let y = y as f32;
            commands.spawn((
                Sprite::from_atlas_image(
                    handle.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 1,
                    },
                ),
                Player,
                Transform::from_xyz(0.0 + 0.0 * 8.0, 1.0 + y * 8.0, 0.0),
                Visibility::default(),
            ));
            commands.spawn((
                Sprite::from_atlas_image(
                    handle.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 1,
                    },
                ),
                Player,
                Transform::from_xyz(0.0 + 9.0 * 8.0, 1.0 + y * 8.0, 0.0),
                Visibility::default(),
            ));
        }
    }
}
