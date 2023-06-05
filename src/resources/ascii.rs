#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Resource)]
pub struct AsciiSheet {
    pub atlas: Handle<TextureAtlas>,
    pub image: Handle<Image>,
}

pub const ITEMS_PER_COLUMN: isize = 16;
pub const TOTAL_COLUMNS: isize = 16;
macro_rules! sheet_index {
    // Converts a zero indexed x,y c oordinate to a index on a sheet
    // Currently limited to only the ascii sheet
    ($row:expr, $column:expr) => {
        ($row * ITEMS_PER_COLUMN) + $column
    };
}

pub const ASCII_SCALE: Vec3 = Vec3::splat(6.0);
pub const TILE_SIZE: Vec2 = Vec2::splat(9.0);
pub const TILE_TRUE_SCALE: Vec2 =
    Vec2::new(ASCII_SCALE.x * TILE_SIZE.x, ASCII_SCALE.y * TILE_SIZE.y);

// ENUM for index of locations for specific tiles in the ascii tile index
pub enum AsciiIndex {
    QuarterSquare = sheet_index!(11, 0),
    HalfSquare = sheet_index!(11, 1),
    ThreeQuarterSquare = sheet_index!(11, 2),
    FullSquare = sheet_index!(13, 11),

    E = sheet_index!(4, 5),
    P = sheet_index!(5, 0),
}

#[allow(clippy::from_over_into)] // We cant create a From for usize
impl Into<usize> for AsciiIndex {
    fn into(self: Self) -> usize {
        self as usize
    }
}

pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid(
        image.clone(),
        TILE_SIZE,
        TOTAL_COLUMNS as usize,
        ITEMS_PER_COLUMN as usize,
        Some(Vec2::splat(2.0)),
        None,
    );

    commands.insert_resource(AsciiSheet {
        atlas: texture_atlases.add(atlas),
        image: image,
    });
}
