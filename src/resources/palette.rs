/// Allows the usage of 255 based values for defining colors.
/// This enables some vs code extensions to match the color and display.
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color::rgb($r as f32 / 255., $g as f32 / 255., $b as f32 / 255.)
    };
}

pub mod game {
    use bevy::prelude::Color;
    pub const BACKGROUND: Color = rgb!(00, 21, 36);
    pub const MIDGROUND: Color = rgb!(21, 97, 109);
    pub const PLAYER: Color = rgb!(255, 236, 209);
    pub const ENEMY: Color = rgb!(120, 41, 15);
    pub const ITEM: Color = rgb!(255, 125, 0);
}

pub mod menu {
    use bevy::prelude::Color;
    pub const TEXT_COLOR: Color = rgb!(00, 21, 36);

    pub mod button {
        use bevy::prelude::Color;
        pub const BACKGROUND: Color = rgb!(255, 236, 209);
        pub const HOVERED_BACKGROUND: Color = rgb!(255, 246, 217);
        pub const CLICKED_BACKGROUND: Color = rgb!(120, 41, 15);
    }

    pub mod main {
        use bevy::prelude::Color;
        pub const BACKGROUND: Color = rgb!(21, 97, 109);
    }

    pub mod town {
        use bevy::prelude::Color;
        pub const BACKGROUND: Color = rgb!(120, 41, 15);
    }
}
