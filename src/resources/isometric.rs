#![allow(dead_code)]
use bevy::prelude::*;

pub const ROWS: isize = 11;
pub const COLUMNS: isize = 11;
pub const SCALE: Vec3 = Vec2::splat(2.0).extend(0.);
pub const SIZE: Vec2 = Vec2::new(32., 32.);
pub const SPACING: Vec2 = Vec2::new(SCALE.x * SIZE.x, SCALE.y * SIZE.y);

// Converts a zero indexed x,y coordinate to a index on the sheet
macro_rules! sheet_index {
    ($row:expr, $column:expr) => {
        ($row * COLUMNS) + $column
    };
}

#[derive(Clone, Copy)]
pub enum Index {
    Dirt = sheet_index!(0, 3),
    DirtLight = sheet_index!(0, 9),
    DirtMedium = sheet_index!(0, 8),

    Soil = sheet_index!(1, 10),
    SoilCracked = sheet_index!(1, 6),
    SoilCrackedAlt = sheet_index!(1, 7),
    SoilLeaves = sheet_index!(1, 8),
    SoilBlades = sheet_index!(1, 9),

    GrassShort = sheet_index!(2, 2),
    GrassMedium = sheet_index!(2, 0),
    GrassTall = sheet_index!(2, 1),

    ShrubA = sheet_index!(2, 7),
    ShrubB = sheet_index!(2, 8),
    ShrubC = sheet_index!(2, 9),
    ShrubD = sheet_index!(2, 10),

    Water = sheet_index!(8, 6),
    WaterWavy = sheet_index!(8, 7),
    WaterWavyLight = sheet_index!(8, 8),
}

pub struct IsometricSheet {
    pub atlas: Handle<TextureAtlas>,
    pub image: Handle<Image>,
}
impl IsometricSheet {
    pub fn load(
        assets: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> IsometricSheet {
        let image = assets.load("spritesheet.png");
        let atlas = TextureAtlas::from_grid(
            image.clone(),
            SIZE,
            COLUMNS as usize,
            ROWS as usize,
            None,
            None,
        );

        IsometricSheet {
            atlas: texture_atlases.add(atlas),
            image: image,
        }
    }
}
