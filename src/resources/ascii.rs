#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub const ITEMS_PER_COLUMN: isize = 16;
macro_rules! sheet_index {
    // Converts a zero indexed x,y coordinate to a index on a sheet
    // Currently limited to only the ascii sheet
    ($row:expr, $column:expr) => {
        ($row * ITEMS_PER_COLUMN) + $column
    };
}

pub const ASCII_SCALE: Vec3 = Vec3::splat(6.0);
pub enum AsciiIndex {
    QuarterSquare = sheet_index!(11, 0),
    HalfSquare = sheet_index!(11, 1),
    ThreeQuarterSquare = sheet_index!(11, 2),
    FullSquare = sheet_index!(13, 11),

    P = sheet_index!(5, 0),
}

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
        image,
        Vec2::splat(9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        None,
    );

    commands.insert_resource(AsciiSheet(texture_atlases.add(atlas)));
}
