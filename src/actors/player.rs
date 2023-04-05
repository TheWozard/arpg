use crate::dragndrop::Draggable;
use crate::grid;
use crate::palette;
use crate::resources::ascii::*;
use crate::resources::ordering::*;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite: TextureAtlasSprite {
                index: AsciiIndex::P.into(),
                color: palette::PLAYER,
                ..default()
            },
            transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(
                0.0,
                0.0,
                LayerOrder::PlayerLayer.into(),
            )),
            ..default()
        },
        Draggable::default(),
        grid::GridTracked::default(),
        Name::new("Player"),
    ));
}
