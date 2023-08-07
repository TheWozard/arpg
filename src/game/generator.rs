use crate::game::cleanup;
use crate::game::grid;
use crate::game::pools::{tiles, Pickable};
use crate::resources::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use noise::NoiseFn;
use rand::prelude::*;

use super::stats;

pub fn spawn_boxes(
    size: IVec2,
    grid: &grid::Grid,
    commands: &mut Commands,
    resources: &Res<Resources>,
) {
    let mut rng = rand::thread_rng();
    let generator = noise::OpenSimplex::new(rng.gen());
    let scale = Vec2::new(0.1, 0.1);
    let mut root = commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        Name::new(format!("Map")),
        cleanup::CleanupHint,
    ));
    for x in 0..size.x {
        for y in 0..size.y {
            let noise = generator.get([(x as f32 * scale.x) as f64, (y as f32 * scale.y) as f64]);
            let position = grid.world_position_from_grid_position(&IVec2::new(x, y));
            let scale = Vec3::new([-1., 1.].choose(&mut rng).unwrap().to_owned(), 1., 1.);
            match noise {
                v if v < -0.15 => {
                    spawn_tile(
                        position,
                        scale,
                        tiles::WATER.pick(&mut rng),
                        &mut root,
                        resources,
                    );
                }
                v if v < -0.0 => {
                    spawn_tile(
                        position,
                        scale,
                        tiles::DIRT.pick(&mut rng),
                        &mut root,
                        resources,
                    );
                }
                v if v < 0.20 => {
                    spawn_tile(
                        position,
                        scale,
                        tiles::SOIL.pick(&mut rng),
                        &mut root,
                        resources,
                    );
                }
                _ => {
                    spawn_tile(
                        position,
                        scale,
                        tiles::GRASS.pick(&mut rng),
                        &mut root,
                        resources,
                    );
                }
            }
        }
    }
}

pub fn spawn_tile(
    translation: Vec3,
    scale: Vec3,
    index: isometric::Index,
    commands: &mut EntityCommands,
    resources: &Res<Resources>,
) {
    let entity = commands
        .commands()
        .spawn((
            SpriteSheetBundle {
                texture_atlas: resources.isometric.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: index as usize,
                    ..default()
                },
                transform: Transform::from_scale(isometric::SCALE * scale)
                    .with_translation(translation),
                ..default()
            },
            Name::new(format!("Tile")),
        ))
        .id();
    commands.add_child(entity);
}

pub fn spawn_player(
    translation: Vec3,
    commands: &mut Commands,
    resources: &Res<Resources>,
) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: resources.ascii.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: ascii::AsciiIndex::P as usize,
                    color: palette::game::PLAYER,
                    ..default()
                },
                transform: Transform::from_scale(ascii::ASCII_SCALE).with_translation(translation),
                ..default()
            },
            Name::new(format!("Player")),
            stats::Life(100),
            stats::Damage(10),
            cleanup::CleanupHint,
        ))
        .id()
}

pub fn spawn_enemy(
    translation: Vec3,
    commands: &mut Commands,
    resources: &Res<Resources>,
) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: resources.ascii.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: ascii::AsciiIndex::E as usize,
                    color: palette::game::ENEMY,
                    ..default()
                },
                transform: Transform::from_scale(ascii::ASCII_SCALE).with_translation(translation),
                ..default()
            },
            Name::new(format!("Enemy")),
            stats::Life(50),
            stats::Damage(5),
            cleanup::CleanupHint,
        ))
        .id()
}

pub fn spawn_multiple(
    count: &i32,
    size: &IVec2,
    commands: &mut Commands,
    spawn: impl Fn(IVec2, &mut Commands),
) {
    let mut rng = rand::thread_rng();
    for i in 0..count.to_owned() {
        let x = rng.gen_range(0..size.x);
        let y = rng.gen_range(0..size.y);
        spawn(IVec2::new(x, y), commands);
    }
}
