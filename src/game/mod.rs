use bevy::prelude::*;

use crate::AppState;
mod actors;
pub mod grid;

const ACTIVE_STATE: AppState = AppState::Game;

// Mounts the town systems
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grid::GridPlugin);
        app.add_systems(
            OnEnter(ACTIVE_STATE),
            (
                actors::spawn_boxes,
                actors::spawn_enemies,
                actors::spawn_player,
            ),
        );
        app.add_systems(Update, return_to_town.run_if(in_state(ACTIVE_STATE)));
        app.add_systems(OnExit(ACTIVE_STATE), actors::GameHint::cleanup);
    }
}

fn return_to_town(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state.set(AppState::Town)
    }
}
