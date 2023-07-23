#![allow(dead_code)]
use bevy::prelude::*;

pub mod ascii;
pub mod fonts;
pub mod layers;
pub mod palette;

/// ResourcePlugin handles loading and management of resources.
pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (ascii::load_ascii, fonts::load_fonts));
    }
}
