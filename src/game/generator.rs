use crate::game::cleanup;
use crate::game::grid;
use crate::resources::*;
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_boxes(
    size: IVec2,
    grid: &grid::Grid,
    commands: &mut Commands,
    resources: &Res<Resources>,
) {
    for x in 0..size.x {
        for y in 0..size.y {
            match x {
                _ if x < 5 => spawn_tile(
                    grid.world_position_from_grid_position(&IVec2::new(x, y)),
                    [
                        isometric::Index::Dirt,
                        isometric::Index::DirtLight,
                        isometric::Index::DirtMedium,
                    ]
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_owned(),
                    commands,
                    resources,
                ),
                _ if x < 10 => spawn_tile(
                    grid.world_position_from_grid_position(&IVec2::new(x, y)),
                    [
                        isometric::Index::Soil,
                        isometric::Index::SoilBlades,
                        isometric::Index::SoilLeaves,
                        isometric::Index::SoilCracked,
                        isometric::Index::SoilCrackedAlt,
                    ]
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_owned(),
                    commands,
                    resources,
                ),
                _ if x < 15 => spawn_tile(
                    grid.world_position_from_grid_position(&IVec2::new(x, y)),
                    [
                        isometric::Index::GrassShort,
                        isometric::Index::GrassMedium,
                        isometric::Index::GrassTall,
                    ]
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_owned(),
                    commands,
                    resources,
                ),
                _ => spawn_tile(
                    grid.world_position_from_grid_position(&IVec2::new(x, y)),
                    [
                        isometric::Index::ShrubA,
                        isometric::Index::ShrubB,
                        isometric::Index::ShrubC,
                        isometric::Index::ShrubD,
                    ]
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_owned(),
                    commands,
                    resources,
                ),
            }
        }
    }
}

pub fn spawn_tile(
    translation: Vec3,
    index: isometric::Index,
    commands: &mut Commands,
    resources: &Res<Resources>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: resources.isometric.atlas.clone(),
            sprite: TextureAtlasSprite {
                index: index as usize,
                ..default()
            },
            transform: Transform::from_scale(isometric::SCALE).with_translation(translation),
            ..default()
        },
        Name::new(format!("Tile")),
        cleanup::CleanupHint,
    ));
}
