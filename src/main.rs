//! A basic APRG game
use bevy::{app::AppExit, prelude::*, window};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod camera;
mod debug;
mod game;
mod macros;
mod menu;
mod resources;
mod town;

pub const RATIO: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 600.;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(resources::palette::BACKGROUND))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Learning".to_string(),
                        resolution: window::WindowResolution::new(HEIGHT * RATIO, HEIGHT),
                        present_mode: window::PresentMode::AutoVsync,
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()) // For pixel rendering
                .build()
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_state::<AppState>()
        .add_plugins((
            resources::ResourcePlugin,
            menu::MenuPlugin,
            town::TownPlugin,
            game::MapPlugin::new(AppState::Game),
            camera::CameraPlugin,
        ))
        .add_systems(Update, quick_close);

    // We only compile in debugging ui if we arn't deployed to Web
    #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
    {
        app.add_plugins(debug::DebugPlugin::default());
    }

    app.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    Menu,
    Town,
    #[default]
    Game,
}

// quick_close listens for quick exit conditions to ensure the game can always be closed.
fn quick_close(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    // Alt + F4 always works
    let alt = keyboard_input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);
    if alt && keyboard_input.pressed(KeyCode::F4) {
        exit.send(AppExit)
    }
}
