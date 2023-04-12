//! A basic APRG game
use bevy::{app::AppExit, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod actors;
mod camera;
mod debug;
mod dragndrop;
mod grid;
mod resources;

pub const RATIO: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 600.;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(resources::palette::BACKGROUND))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mapping".to_string(),
                        // resolution: WindowResolution::new(HEIGHT * RATIO, HEIGHT),
                        // present_mode: PresentMode::AutoVsync,
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                // For pixel rendering
                .set(ImagePlugin::default_nearest())
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(resources::ResourcePlugin)
        .add_startup_system(actors::spawn_player)
        .add_startup_system(actors::spawn_boxes)
        .add_startup_system(actors::spawn_enemies)
        .add_system(quick_close)
        .add_plugin(dragndrop::DragNDrop)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(grid::GridPlugin);

    #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
    {
        app.add_plugin(debug::DebugPlugin::default());
    }

    app.run();
}

fn quick_close(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    let alt = keyboard_input.any_pressed([KeyCode::LAlt, KeyCode::RAlt]);
    if (alt && keyboard_input.pressed(KeyCode::F4)) || keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit)
    }
}
