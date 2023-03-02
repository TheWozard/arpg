use bevy::prelude::*;

// The global state of the UI
#[derive(Debug, Reflect, Resource)]
pub struct UIState {
    pub skill_tree_open: bool,
}

impl Default for UIState {
    fn default() -> Self {
        UIState {
            skill_tree_open: false,
        }
    }
}

// The settings for features of the UI
#[derive(Debug, Reflect, Resource)]
pub struct UISettings {
    pub skill_tree_key: KeyCode,
}

impl Default for UISettings {
    fn default() -> Self {
        UISettings {
            skill_tree_key: KeyCode::G,
        }
    }
}