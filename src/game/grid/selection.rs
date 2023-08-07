use crate::game::grid::*;
use crate::game::*;
use crate::resources::*;

/// Attaches selection features that enable the player to click on interact with entities in the grid.
pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        // Types
        app.register_type::<Selectable>();
        app.register_type::<GridSelection>();

        // Systems
        app.add_systems(Update, (core_action).after(GridCursor::track));
    }
}

/// The primary interaction function.
/// TODO: currently this is all in one function. If we want to better control the output, should we use Events?
/// This could add flexibility to dispatch actions, would probably want to add a new Schedule stage after Update to
/// enable all of these actions to be processed in parallel and ensure they are handled the same frame.
pub fn core_action(
    mut commands: Commands,
    mut selection: ResMut<GridSelection>,
    mut grid: ResMut<grid::Grid>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    grid_cursor: Res<grid::GridCursor>,
    resources: Res<Resources>,
    mut query: Query<&mut Transform>,
) {
    // Alt
    if mouse.just_pressed(MouseButton::Right)
        || (keyboard.pressed(KeyCode::SuperLeft) && mouse.just_pressed(MouseButton::Left))
    {
        selection.clear_selection();
        selection.clear_cursor(&mut commands);
        return;
    }

    // Primary
    if mouse.just_pressed(MouseButton::Left) {
        match (
            grid.get_entity(&grid_cursor.position),
            selection.selection,
            selection.selection_position,
        ) {
            (Some(e), _, _) => {
                selection.set_selection(e, grid_cursor.position);
                selection.spawn_cursor(grid_cursor.translation, &mut commands, &resources);
            }
            (None, Some(s), Some(p)) => {
                if let Ok(mut transform) = query.get_mut(s) {
                    transform.translation =
                        grid.world_position_on_top_from_grid_position(&grid_cursor.position);
                    grid.move_entity(&p, &grid_cursor.position, s);
                    selection.set_selection(s, grid_cursor.position);
                    selection.spawn_cursor(grid_cursor.translation, &mut commands, &resources);
                }
            }
            (_, _, _) => {}
        }
    }
}

/// Defines an entity as selectable when marked as the entity for a given tile.
#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Selectable;

/// Defines the currently selected entity and its grid position.
#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GridSelection {
    pub selection: Option<Entity>,
    pub selection_position: Option<IVec2>,

    /// The currently instantiated cursor.
    cursor: Option<Entity>,
}
impl GridSelection {
    /// Removes the current cursor and clears selection.
    fn clear_cursor(&mut self, commands: &mut Commands) {
        if let Some(a) = self.cursor {
            if let Some(entity) = commands.get_entity(a) {
                entity.despawn_recursive()
            }
            self.cursor = None;
        }
    }

    /// Spawns the GridSelections cursor at a given location. Will cleanup any previous cursors.
    fn spawn_cursor(&mut self, at: Vec3, commands: &mut Commands, resources: &Res<Resources>) {
        self.clear_cursor(commands);
        self.cursor = Some(grid::generator::spawn_selection_cursor(
            at, commands, &resources,
        ));
    }

    /// Clears the current selection
    fn clear_selection(&mut self) {
        if let Some(_) = self.selection {
            self.selection = None;
            self.selection_position = None;
        }
    }

    /// Sets the current selection
    fn set_selection(&mut self, e: Entity, p: IVec2) {
        self.selection = Some(e);
        self.selection_position = Some(p);
    }
}
