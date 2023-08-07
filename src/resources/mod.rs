#![allow(dead_code)]
use bevy::prelude::*;

pub mod ascii;
pub mod fonts;
pub mod generated;
pub mod isometric;
pub mod layers;
pub mod palette;

#[derive(Resource)]
pub struct Resources {
    pub isometric: isometric::IsometricSheet,
    pub ascii: ascii::AsciiSheet,
    pub fonts: fonts::Fonts,
    pub generated: generated::GeneratedMeshes,
}

/// ResourcePlugin handles loading and management of resources.
pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load);
    }
}

fn load(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Resources {
        isometric: isometric::IsometricSheet::load(&assets, &mut texture_atlases),
        ascii: ascii::AsciiSheet::load(&assets, &mut texture_atlases),
        fonts: fonts::Fonts::load(&assets),
        generated: generated::GeneratedMeshes::load(&mut meshes, &mut materials),
    })
}
