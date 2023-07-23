#![allow(dead_code)]
use bevy::prelude::*;

pub const ITEMS_PER_COLUMN: isize = 16;
pub const TOTAL_COLUMNS: isize = 16;

pub const ASCII_SCALE: Vec3 = Vec3::splat(6.0);
pub const TILE_SIZE: Vec2 = Vec2::splat(9.0);
pub const TILE_TRUE_SCALE: Vec2 =
    Vec2::new(ASCII_SCALE.x * TILE_SIZE.x, ASCII_SCALE.y * TILE_SIZE.y);

macro_rules! sheet_index {
    // Converts a zero indexed x,y c oordinate to a index on a sheet
    // Currently limited to only the ascii sheet
    ($row:expr, $column:expr) => {
        ($row * ITEMS_PER_COLUMN) + $column
    };
}

// ENUM for index of locations for specific tiles in the ascii tile index
pub enum AsciiIndex {
    Square = sheet_index!(0, 0),
    Heart = sheet_index!(0, 3),
    Diamond = sheet_index!(0, 4),
    Club = sheet_index!(0, 5),
    Spade = sheet_index!(0, 6),

    QuarterSquare = sheet_index!(11, 0),
    HalfSquare = sheet_index!(11, 1),
    ThreeQuarterSquare = sheet_index!(11, 2),
    FullSquare = sheet_index!(13, 11),

    A = sheet_index!(4, 1),
    B = sheet_index!(4, 2),
    C = sheet_index!(4, 3),
    D = sheet_index!(4, 4),
    E = sheet_index!(4, 5),
    P = sheet_index!(5, 0),
}

#[allow(clippy::from_over_into)] // We cant create a From for usize
impl Into<usize> for AsciiIndex {
    fn into(self: Self) -> usize {
        self as usize
    }
}

#[derive(Resource)]
pub struct AsciiSheet {
    pub atlas: Handle<TextureAtlas>,
    pub image: Handle<Image>,
}
impl AsciiSheet {
    pub fn load(
        assets: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> AsciiSheet {
        let image = assets.load("Ascii.png");
        let atlas = TextureAtlas::from_grid(
            image.clone(),
            TILE_SIZE,
            TOTAL_COLUMNS as usize,
            ITEMS_PER_COLUMN as usize,
            Some(Vec2::splat(2.0)),
            None,
        );

        AsciiSheet {
            atlas: texture_atlases.add(atlas),
            image: image,
        }
    }
}
