use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Life>();
        app.register_type::<Damage>();
    }
}

#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Life(pub i32);

#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Damage(pub i32);
