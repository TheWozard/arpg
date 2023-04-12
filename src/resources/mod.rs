#![allow(dead_code)]
use bevy::prelude::*;

pub mod ascii;
pub mod palette;

/// ResourcePlugin handles loading and management of resources.
pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ascii::load_ascii.in_base_set(StartupSet::PreStartup));
    }
}

/// LayerOrder defines the z height/order elements should be rendered in.
pub enum LayerOrder {
    BackgroundLayer,
    EnemyLayer,
    PlayerLayer,
}

/// We will mostly be using LayerOrder as a part of Vec3 so being able to easily cast to f32 is important.
#[allow(clippy::from_over_into)] // We cant create a From for f32.
impl Into<f32> for LayerOrder {
    fn into(self: Self) -> f32 {
        (self as i16).into()
    }
}
