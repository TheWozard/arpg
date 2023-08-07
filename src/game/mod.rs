use crate::resources;
use crate::AppState;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

mod cleanup;
mod generator;
pub mod grid;
mod pools;
mod stats;

crate::StateBasedPlugin!(MapPlugin);
impl<S: States> Plugin for MapPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default());
        app.add_plugins((grid::GridPlugin::new(AppState::Game), stats::StatsPlugin));
        app.add_systems(OnEnter(self.state()), Map::generate);
        app.add_systems(Update, return_to_town.run_if(in_state(self.state())));
        app.add_systems(OnExit(self.state()), cleanup::CleanupHint::cleanup);
    }
}

fn return_to_town(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state.set(AppState::Town)
    }
}

#[derive(Resource)]
struct Map {
    generator: Box<dyn Generator + Sync + Send>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            generator: SequentialGeneration::new(vec![
                Box::new(TileSet::Basic(IVec2::new(10, 10))),
                Box::new(TileSet::Player(IVec2::new(4, 4))),
                Box::new(TileSet::Enemies(5, IVec2::new(10, 10))),
                Box::new(grid::generator::Cursor),
            ]),
        }
    }
}

impl Map {
    fn generate(
        mut commands: Commands,
        resources: Res<resources::Resources>,
        map: Res<Map>,
        mut grid: ResMut<grid::Grid>,
    ) {
        grid.clear();
        map.generator.generate(&mut commands, &resources, &mut grid);
    }
}

pub trait Generator {
    fn generate(
        &self,
        commands: &mut Commands,
        resources: &Res<resources::Resources>,
        grid: &mut ResMut<grid::Grid>,
    );
}

struct SequentialGeneration(Vec<Box<dyn Generator + Sync + Send>>);

impl SequentialGeneration {
    pub fn new(values: Vec<Box<dyn Generator + Sync + Send>>) -> Box<Self> {
        Box::new(Self(values))
    }
}

impl Generator for SequentialGeneration {
    fn generate(
        &self,
        commands: &mut Commands,
        resources: &Res<resources::Resources>,
        grid: &mut ResMut<grid::Grid>,
    ) {
        for generator in self.0.iter() {
            generator.generate(commands, resources, grid)
        }
    }
}

#[allow(dead_code)]
enum TileSet {
    Basic(IVec2),
    Player(IVec2),
    Enemies(i32, IVec2),
    Default,
}
impl Generator for TileSet {
    fn generate(
        &self,
        commands: &mut Commands,
        resources: &Res<resources::Resources>,
        grid: &mut ResMut<grid::Grid>,
    ) {
        match self {
            TileSet::Basic(size) => {
                generator::spawn_boxes(size.to_owned(), grid, commands, resources);
            }
            TileSet::Player(location) => {
                let entity = generator::spawn_player(
                    grid.world_position_on_top_from_grid_position(&location),
                    commands,
                    resources,
                );
                if !grid.add_entity(location, entity) {
                    if let Some(entity) = commands.get_entity(entity) {
                        entity.despawn_recursive()
                    }
                }
            }
            TileSet::Enemies(count, size) => {
                let grid_mutex = Arc::new(Mutex::new(grid));
                generator::spawn_multiple(count, size, commands, |position, commands| {
                    if let Ok(mut g) = grid_mutex.lock() {
                        if g.get_entity(&position).is_none() {
                            let translation = g.world_position_on_top_from_grid_position(&position);
                            g.add_entity(
                                &position,
                                generator::spawn_enemy(translation, commands, resources),
                            );
                        }
                    }
                });
            }
            TileSet::Default => return,
        }
    }
}
