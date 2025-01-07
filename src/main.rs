use bevy::{
    core::FrameCount,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation},
        RenderPlugin,
    },
    window::WindowResolution,
};
use bevy_ecs_tilemap::prelude::*;

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
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, spawn_map)
        .add_systems(Update, make_visible)
        .run();
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}

const MAP: [[u32; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let handle = asset_server.load("sprites/minirogue.png");
    let map_size = TilemapSize { x: 8, y: 8 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_index = MAP[x as usize][y as usize];
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(tile_index),
                    tilemap_id: TilemapId(tilemap_entity),
                    visible: if tile_index == 0 {
                        TileVisible(false)
                    } else {
                        TileVisible(true)
                    },
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
