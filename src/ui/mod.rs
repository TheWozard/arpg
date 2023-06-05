use bevy::prelude::*;

/// UIPlugin loads the systems for managing the UI components.
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(grid_position_forced.after(crate::dragndrop::dragndrop_movement));
    }
}

pub fn ui_track() {
    for (mut transform, mut tracking) in &mut entities {
        let pos = grid.new_grid_position_from_translation(transform.translation);
        transform.translation = grid
            .new_translation_from_grid(&pos)
            .extend(transform.translation.z);
        if pos != tracking.position {
            // We should remove it from the old tile and add it to the new
            tracking.position = pos;
        }
    }
}
