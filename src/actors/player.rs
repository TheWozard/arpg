use bevy::prelude::*;
use crate::resources::ascii::*;
use crate::resources::ordering::*;

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite: TextureAtlasSprite {
                index: AsciiIndex::P.into(),
                color: Color::CYAN,
                ..default()
            },
            transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(0.0,0.0,LayerOrder::PlayerLayer.into())),
            ..default()
        },
        Name::new("Player"),
    ));
}