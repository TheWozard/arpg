use bevy::prelude::*;
pub mod button;

// Mounts systems to handle interactive UI elements
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button::interact_with_interactive_buttons);
    }
}
