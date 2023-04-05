//! A basic APRG game
use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;

pub const RATIO: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 600.;

mod actors;
mod camera;
mod debug;
mod dragndrop;
mod grid;
mod palette;
mod resources;
// mod ui;

// use ui::gui::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(palette::BACKGROUND))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mapping".to_string(),
                        resolution: WindowResolution::new(HEIGHT * RATIO, HEIGHT),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_startup_system(resources::ascii::load_ascii.in_base_set(StartupSet::PreStartup))
        .add_startup_system(actors::player::spawn_player)
        .add_startup_system(actors::boxes::spawn_boxes)
        .add_system(quick_close)
        .add_plugin(dragndrop::DragNDrop)
        // .add_startup_system(setup)
        // .add_system(button_system)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(grid::GridPlugin)
        // .add_plugin(GuiPlugin)
        .add_plugin(debug::DebugPlugin::default())
        .run();
}

fn quick_close(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    let alt = keyboard_input.any_pressed([KeyCode::LAlt, KeyCode::RAlt]);
    if (alt && keyboard_input.pressed(KeyCode::F4)) || keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit)
    }
}
