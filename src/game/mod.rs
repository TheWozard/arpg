use self::grid::dragndrop;
use crate::resources;
use crate::AppState;
use bevy::prelude::*;

mod cleanup;
mod generator;
pub mod grid;

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
                Box::new(TileSet::Basic(5)),
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
    Basic(i32),
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
            TileSet::Basic(count) => {
                generator::spawn_boxes(IVec2::new(20, 20), grid, commands, resources)
            }
            TileSet::Default => return,
        }
    }
}
