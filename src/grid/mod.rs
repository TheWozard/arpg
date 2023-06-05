use crate::resources::ascii;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_inspector_egui::prelude::*;

/// GridPlugin loads the Grid resource and adds systems responsible for maintaining the grid.
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::default())
            .add_system(grid_position_forced.after(crate::dragndrop::dragndrop_movement));
    }
}

/// grid_position_forced moves anything with a Transform that has changed onto its grid based location.
pub fn grid_position_forced(
    mut entities: Query<(&mut Transform, &mut GridTracked), Changed<Transform>>,
    grid: Res<Grid>,
) {
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

/// GridTracked forces a Transform to align with the grid.
#[derive(Component)]
pub struct GridTracked {
    pub position: GridPosition,
}

impl Default for GridTracked {
    fn default() -> Self {
        GridTracked {
            position: GridPosition::default(),
        }
    }
}

/// GridLocation an integer based vector for a entity location on the grid.
/// This requires a Grid to translate back to world space as the grid size is not fixed.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> GridPosition {
        GridPosition { x, y }
    }
}

impl Default for GridPosition {
    fn default() -> Self {
        GridPosition { x: 0, y: 0 }
    }
}

#[derive(Reflect, Clone, Eq, PartialEq, Debug)]
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

/// Grid holds the locations of all elements on the grid
#[derive(Resource)]
pub struct Grid {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
    scale: Vec2,
    world_offset: Vec2,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(10, 10, ascii::TILE_TRUE_SCALE, Vec2::new(0., 0.))
    }
}

impl Grid {
    pub fn new(height: usize, width: usize, scale: Vec2, world_offset: Vec2) -> Self {
        Grid {
            tiles: vec![Tile::default(); height * width],
            height: height,
            width: width,
            scale: scale,
            world_offset: world_offset,
        }
    }

    pub fn clear(&mut self) {
        self.tiles.fill_with(Tile::default);
    }

    pub fn get_tile(&self, position: &GridPosition) -> Option<&Tile> {
        self.tiles.get(self.get_index_of(position))
    }

    pub fn new_grid_position_from_translation(&self, translation: Vec3) -> GridPosition {
        GridPosition::new(
            (translation.x / self.scale.x).round() as i32,
            (translation.y / self.scale.y).round() as i32,
        )
    }

    /// Converts a GridPosition to its location in world space
    pub fn new_translation_from_grid(&self, position: &GridPosition) -> Vec2 {
        Vec2::new(
            position.x as f32 * self.scale.x,
            position.y as f32 * self.scale.y,
        )
    }

    /// adds an entity at a new position
    pub fn add_entity(mut self, position: &GridPosition, entity: Entity) -> bool {
        let index = self.get_index_of(position);
        let tile = self.tiles.get_mut(index);
        match tile {
            Some(s) => {
                s.items.insert(entity);
                true
            }
            None => false,
        }
    }

    /// Fakes the 2d position into the grid, this should be the only location that ever does this.
    /// GridPositions should be the only thing to access the underlying grid.
    fn get_index_of(&self, position: &GridPosition) -> usize {
        return (position.y as usize * self.width) + position.x as usize;
    }
}
