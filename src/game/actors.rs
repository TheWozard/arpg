use crate::game::grid;
use crate::game::grid::dragndrop::*;
use crate::resources::ascii::*;
use crate::resources::palette;
use crate::resources::*;
use bevy::prelude::*;
use rand::prelude::*;

crate::Cleanup!(GameHint);

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ascii.atlas.clone(),
            sprite: TextureAtlasSprite {
                index: AsciiIndex::P.into(),
                color: palette::PLAYER,
                ..default()
            },
            transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(
                0.0,
                0.0,
                LayerOrder::PlayerLayer.index(),
            )),
            ..default()
        },
        Draggable::default(),
        grid::GridTracked::default(),
        Name::new("Player"),
        GameHint::default(),
    ));
}

pub fn spawn_boxes(mut commands: Commands, ascii: Res<AsciiSheet>) {
    for n in 0..10 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: ascii.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: AsciiIndex::HalfSquare.into(),
                    color: palette::MIDGROUND,
                    ..default()
                },
                transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(
                    rand::thread_rng().gen_range(-200.0..=200.0),
                    rand::thread_rng().gen_range(-200.0..=200.0),
                    LayerOrder::EnemyLayer.index(),
                )),
                ..default()
            },
            grid::GridTracked::default(),
            Name::new(format!("Item{:?}", n)),
            GameHint::default(),
        ));
    }
}

pub fn spawn_enemies(mut commands: Commands, ascii: Res<AsciiSheet>) {
    for n in 0..5 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: ascii.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: AsciiIndex::E.into(),
                    color: palette::ENEMY,
                    ..default()
                },
                transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(
                    rand::thread_rng().gen_range(-400.0..=400.0),
                    rand::thread_rng().gen_range(-400.0..=400.0),
                    LayerOrder::EnemyLayer.index(),
                )),
                ..default()
            },
            grid::GridTracked::default(),
            Name::new(format!("Item{:?}", n)),
            GameHint::default(),
        ));
    }
}
