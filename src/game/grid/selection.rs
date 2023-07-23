use crate::game::grid::*;
use crate::game::*;
use crate::resources::*;
use bevy_inspector_egui::prelude::*;

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GridSelection {
    selected: Option<Entity>,
}

pub fn select_tile(
    mut commands: Commands,
    mut selection: ResMut<GridSelection>,
    buttons: Res<Input<MouseButton>>,
    grid_cursor: Res<grid::GridCursor>,
    resources: Res<Resources>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(a) = selection.selected {
            if let Some(entity) = commands.get_entity(a) {
                entity.despawn_recursive()
            }
        }
        selection.selected = Some(grid::generator::spawn_selection_cursor(
            grid_cursor.translation,
            &mut commands,
            &resources,
        ));
    }
}
