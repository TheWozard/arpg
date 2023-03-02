use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, WorldInspectorParams};

#[derive(Debug, Reflect, Resource)]
pub struct DebugSettings {
    pub debug_key_code: KeyCode
}

fn debug_toggle(debug_settings: Res<DebugSettings>, input: Res<Input<KeyCode>>, mut inspector: ResMut<WorldInspectorParams>) {
    if input.just_pressed(debug_settings.debug_key_code) {
        inspector.enabled = !inspector.enabled
    }
}

fn setup_debug(mut inspector: ResMut<WorldInspectorParams>) {
    inspector.enabled = false;
    inspector.sort_components = true;
}

pub struct DebugPlugin {
    pub debug_key_code: KeyCode
}

impl Default for DebugPlugin {
    fn default() -> Self { DebugPlugin {
        debug_key_code: KeyCode::Grave,
    }}
}

// Plugin grouping all debug functionality
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .insert_resource(DebugSettings{
                debug_key_code: self.debug_key_code
            })
            .add_system(debug_toggle)
            .add_startup_system(setup_debug);
    }
}
