#![allow(dead_code)]
use bevy::prelude::Color;

/// Allows the usage of 255 based values for defining colors.
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::rgb($r / 255., $g / 255., $b / 255.)
    };
}

// https://coolors.co/palette/001524-15616d-ffecd1-ff7d00-78290f
pub const BACKGROUND: Color = color!(00., 21., 36.); // #001524
pub const MIDGROUND: Color = color!(21., 97., 109.); // #15616D
pub const PLAYER: Color = color!(255., 236., 209.); // #FFECD1
pub const ENEMY: Color = color!(120., 41., 15.); // #78290F

// pub const ITEM: Color = color!(255., 125., 0.); // #FF7D00

pub const TILE_COLORS: [Color; 3] = [PLAYER, ENEMY, MIDGROUND];

// UI Components
pub const MENU_BACKGROUND: Color = color!(21., 97., 109.); // #15616D
pub const MENU_BUTTON_BACKGROUND: Color = color!(255., 236., 209.); // #FFECD1
pub const MENU_BUTTON_HOVERED_BACKGROUND: Color = color!(255., 246., 217.);
pub const MENU_BUTTON_CLICKED_BACKGROUND: Color = color!(120., 41., 15.); // #78290F
pub const MENU_TEXT_COLOR: Color = color!(00., 21., 36.); // #001524

pub const TOWN_BACKGROUND: Color = color!(120., 41., 15.); // #78290F
