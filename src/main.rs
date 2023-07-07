//! A basic APRG game
use bevy::{app::AppExit, prelude::*, window};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod camera;
mod debug;
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
                // For pixel rendering
                .set(ImagePlugin::default_nearest())
                .build()
                // Uncomment to use bevy_mod_debugdump
                // .disable::<LogPlugin>()
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_state::<AppState>()
        .add_plugin(resources::ResourcePlugin)
        .add_plugin(menu::MenuPlugin)
        // Constant Systems - These will always be running.
        .add_system(quick_close)
        .add_plugin(camera::CameraPlugin);

    // We only compile in debugging ui if we arn't deployed to Web
    #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
    {
        app.add_plugin(debug::DebugPlugin::default());
    }

    // Use with cargo run | dot -Tsvg > out.svg
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    app.run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Town,
    Game,
}

// quick_close listens for quick exit conditions to ensure the game can always be closed.
fn quick_close(
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<AppState>>,
) {
    // Alt + F4 always works
    let alt = keyboard_input.any_pressed([KeyCode::LAlt, KeyCode::RAlt]);
    if alt && keyboard_input.pressed(KeyCode::F4) {
        exit.send(AppExit)
    }
    // Escape only works in AppState::Menu
    match (game_state.0, keyboard_input.pressed(KeyCode::Escape)) {
        (AppState::Menu, true) => exit.send(AppExit),
        (_, _) => (),
    }
}
