use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation},
        RenderPlugin,
    },
    window::WindowResolution,
};
use rogue::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rogue".to_string(),
                        visible: false,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        // art assets are small, so need to scale up the screen to make them visible
                        resolution: WindowResolution::new(800.0, 600.0)
                            .with_scale_factor_override(3.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(
                        bevy::render::settings::WgpuSettings {
                            // this is done because DX12 causes a bunch of repeating errors that clog up the logs
                            // so we use Vulkan (Desktop usually) and GL as it targets WASM correctly
                            backends: Some(Backends::VULKAN | Backends::GL),
                            ..default()
                        },
                    ),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .run();
}
