use bevy::prelude::*;

use crate::{resources::ascii::AsciiSheet, AppState, StateBasedPlugin};

use self::grid::dragndrop;
mod actors;
pub mod grid;

StateBasedPlugin!(MapPlugin);
impl<S: States> Plugin for MapPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default());
        app.add_plugins((grid::GridPlugin::new(AppState::Game), dragndrop::DragNDrop));
        app.add_systems(OnEnter(self.state()), Map::generate);
        app.add_systems(Update, return_to_town.run_if(in_state(self.state())));
        app.add_systems(OnExit(self.state()), actors::GameHint::cleanup);
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
            generator: SequentialGeneration::new(vec![TileSet::Basic(5).new()]),
        }
    }
}

impl Map {
    fn generate(mut commands: Commands, ascii: Res<AsciiSheet>, map: Res<Map>) {
        map.generator.generate(&mut commands, &ascii)
    }
}

pub trait Generator {
    fn generate(&self, commands: &mut Commands, ascii: &Res<AsciiSheet>);
}

struct SequentialGeneration(Vec<Box<dyn Generator + Sync + Send>>);

impl SequentialGeneration {
    pub fn new(values: Vec<Box<dyn Generator + Sync + Send>>) -> Box<Self> {
        Box::new(Self(values))
    }
}

impl Generator for SequentialGeneration {
    fn generate(&self, commands: &mut Commands, ascii: &Res<AsciiSheet>) {
        for generator in self.0.iter() {
            generator.generate(commands, ascii)
        }
    }
}

#[allow(dead_code)]
enum TileSet {
    Basic(i32),
    Default,
}

impl TileSet {
    pub fn new(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Generator for TileSet {
    fn generate(&self, commands: &mut Commands, ascii: &Res<AsciiSheet>) {
        match self {
            TileSet::Basic(count) => actors::spawn_boxes(count, commands, ascii),
            TileSet::Default => return,
        }
    }
}
