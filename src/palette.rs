use bevy::prelude::Color;

// Allows the usage of 255 based values for defining colors.
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::rgb($r / 255., $g / 255., $b / 255.)
    };
}

pub const BACKGROUND: Color = color!(44.,51.,51.); // #2C3333
pub const PLAYER: Color = color!(203.,228.,222.); // #CBE4DE
