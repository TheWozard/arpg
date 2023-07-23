use crate::camera::WorldCursor;
use crate::resources::isometric;
use crate::StateBasedPlugin;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_inspector_egui::prelude::*;

use self::selection::GridSelection;

pub mod dragndrop;
pub mod generator;
pub mod selection;

StateBasedPlugin!(GridPlugin);
impl<S: States> Plugin for GridPlugin<S> {
    fn build(&self, app: &mut App) {
        // Debug mounting.
        app.register_type::<GridCursor>();
        app.register_type::<GridCursorFollowHint>();
        app.register_type::<GridTracked>();
        app.register_type::<selection::GridSelection>();

        app.insert_resource(Grid::new(
            IVec2::new(10, 10),
            isometric::SPACING,
            Vec2::new(0., 0.),
        ));
        app.insert_resource(GridCursor::default());
        app.insert_resource(GridSelection::default());
        app.add_systems(
            Update,
            (
                GridCursor::track,
                (GridTracked::track, GridCursorFollowHint::track),
            )
                .chain()
                .run_if(in_state(self.state())),
        );
        app.add_systems(
            Update,
            selection::select_tile.run_if(in_state(self.state())),
        );
    }
}

/// GridCursor resource for storing the current grid position the cursor is in. This position is not guaranteed to exist.
#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GridCursor {
    pub position: IVec2,
    pub translation: Vec3,
}
impl GridCursor {
    /// System responsible for keeping the GridCursor up to date
    fn track(world_cursor: Res<WorldCursor>, mut grid_cursor: ResMut<GridCursor>, grid: Res<Grid>) {
        let position = grid.grid_position_from_world_position(world_cursor.position);
        if grid_cursor.position != position {
            grid_cursor.position = grid.grid_position_from_world_position(world_cursor.position);
            grid_cursor.translation = grid.world_position_from_grid_position(&grid_cursor.position);
        }
    }
}

/// FollowGridCursorHint updates the given entity Transform based on GridCursor.
#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct GridCursorFollowHint;
impl GridCursorFollowHint {
    /// System responsible for updating Transform. Does not touch the Z value of the Transform.
    fn track(
        mut entity: Query<&mut Transform, With<GridCursorFollowHint>>,
        cursor: Res<GridCursor>,
    ) {
        for mut transform in entity.iter_mut() {
            transform.translation = cursor.translation
        }
    }
}

/// GridTracked forces a Transform to align with the grid.
#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct GridTracked {
    pub position: IVec2,
}
impl GridTracked {
    /// System responsible for updating GridTracked components and moving Transform translations
    fn track(
        mut grid: ResMut<Grid>,
        mut entities: Query<(&mut Transform, &mut GridTracked, Entity), Changed<Transform>>,
    ) {
        for (mut transform, mut tracker, entity) in &mut entities {
            let pos = grid.grid_position_from_translation(transform.translation);
            // Snap
            transform.translation = grid.world_position_from_grid_position(&pos);

            // Grid Update
            if pos != tracker.position {
                grid.move_entity(&tracker.position, &pos, entity);
                tracker.position = pos;
            }
        }
    }
}

/// Tile holds data relevant to a single tile on the grid.
/// Clone functions are implemented for easily initializing grids, this value should only
/// ever be borrowed outside of the grid.
#[derive(Clone)]
pub struct Tile {
    items: HashSet<Entity>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            items: HashSet::new(),
        }
    }
}

/// Grid holds the locations of all elements on the grid.
/// TODO: Should this be a component?
#[derive(Resource)]
pub struct Grid {
    tiles: Vec<Tile>,
    size: IVec2,
    scale: Vec2,
    offset: Vec2,
}

impl Grid {
    /// Convenience function for building new grids.
    pub fn new(size: IVec2, scale: Vec2, offset: Vec2) -> Self {
        Grid {
            tiles: vec![Tile::default(); (size.x * size.y) as usize],
            size: size,
            scale: Vec2::new(scale.x / 2., scale.y / 4.),
            offset: offset,
        }
    }

    /// Clears all tiles with the default.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.tiles.fill_with(Tile::default);
    }

    /// Gets the given tile data for the provided grid position. Mutable access is never provided.
    #[allow(dead_code)]
    pub fn get_tile(&self, position: &IVec2) -> Option<&Tile> {
        self.tiles.get(self.get_index_of(position))
    }

    /// Converts a bevy based translation to a grid based position. The z level is dropped. Uses standard .round() for translation.
    pub fn grid_position_from_translation(&self, world: Vec3) -> IVec2 {
        self.grid_position_from_world_position(Vec2::new(world.x, world.y))
    }

    /// Converts a bevy based position to a grid based position. Uses standard .round() for translation.
    pub fn grid_position_from_world_position(&self, world: Vec2) -> IVec2 {
        IVec2::new(
            (((world.x / self.scale.x) + (world.y / self.scale.y)) / 2.).round() as i32,
            (((world.y / self.scale.y) - (world.x / self.scale.x)) / 2.).round() as i32,
        )
    }

    /// Converts a grid based position to a bevy based position. Will always be a multiple of scale.
    pub fn world_position_from_grid_position(&self, position: &IVec2) -> Vec3 {
        Vec3::new(
            (position.x as f32 - position.y as f32) * self.scale.x,
            (position.x as f32 + position.y as f32) * self.scale.y,
            (100 - (position.x + position.y)) as f32,
        )
    }

    /// Adds a lightweight entity handle to a given grid position. Returns true if the tile exists.
    pub fn add_entity(&mut self, position: &IVec2, entity: Entity) -> bool {
        let position = self.get_index_of(position);
        match self.tiles.get_mut(position) {
            Some(s) => {
                s.items.insert(entity);
                true
            }
            None => false,
        }
    }

    /// Removes a lightweight entity handle that match the passed handle from a given position.
    pub fn remove_entity(&mut self, position: &IVec2, entity: &Entity) -> bool {
        let position = self.get_index_of(position);
        match self.tiles.get_mut(position) {
            Some(s) => s.items.remove(entity),
            None => false,
        }
    }

    /// Moves a lightweight entity handle from one location to another. Returns if the entity actually moved.
    pub fn move_entity(&mut self, old: &IVec2, new: &IVec2, entity: Entity) -> bool {
        if old == new {
            return false;
        }
        if self.remove_entity(old, &entity) {
            return self.add_entity(new, entity);
        }
        false
    }

    /// Fakes the grid position into and 1d index, this should be the only location that ever does this.
    fn get_index_of(&self, position: &IVec2) -> usize {
        return ((position.y * self.size.y) + position.x) as usize;
    }
}
