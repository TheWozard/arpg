use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin {
    pub debug_key_code: KeyCode,
}

impl Default for DebugPlugin {
    fn default() -> Self {
        DebugPlugin {
            debug_key_code: KeyCode::Grave,
        }
    }
}

// Plugin grouping all debug functionality
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, self.debug_key_code)),
        );
    }
}
