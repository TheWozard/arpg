use self::grid::dragndrop;
use crate::resources;
use crate::AppState;
use bevy::prelude::*;

mod cleanup;
mod generator;
pub mod grid;
mod pools;

crate::StateBasedPlugin!(MapPlugin);
impl<S: States> Plugin for MapPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default());
        app.add_plugins((grid::GridPlugin::new(AppState::Game), dragndrop::DragNDrop));
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
                Box::new(TileSet::Enemies(3, IVec2::new(10, 10))),
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
        grid: Res<grid::Grid>,
    ) {
        map.generator.generate(&mut commands, &resources, &grid)
    }
}

pub trait Generator {
    fn generate(
        &self,
        commands: &mut Commands,
        resources: &Res<resources::Resources>,
        grid: &Res<grid::Grid>,
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
        grid: &Res<grid::Grid>,
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
        grid: &Res<grid::Grid>,
    ) {
        match self {
            TileSet::Basic(size) => {
                generator::spawn_boxes(size.to_owned(), grid, commands, resources);
            }
            TileSet::Player(location) => {
                generator::spawn_player(
                    grid.world_position_on_top_from_grid_position(&location),
                    commands,
                    resources,
                );
            }
            TileSet::Enemies(count, size) => {
                generator::spawn_enemies(count, size, grid, commands, resources);
            }
            TileSet::Default => return,
        }
    }
}
