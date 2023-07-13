#![allow(dead_code)]
use bevy::prelude::*;

pub mod ascii;
pub mod fonts;
pub mod palette;
pub mod ui;

/// ResourcePlugin handles loading and management of resources.
pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (ascii::load_ascii, fonts::load_fonts));
        app.add_plugins(ui::UiPlugin);
    }
}

/// LayerOrder defines the z height/order elements should be rendered in.
pub enum LayerOrder {
    BackgroundLayer,
    EnemyLayer,
    PlayerLayer,
}

impl LayerOrder {
    pub fn index(&self) -> f32 {
        match *self {
            LayerOrder::BackgroundLayer => 0.,
            LayerOrder::EnemyLayer => 100.,
            LayerOrder::PlayerLayer => 200.,
        }
    }
}
