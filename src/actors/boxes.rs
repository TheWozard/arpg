use bevy::prelude::*;
use rand::prelude::*;
use crate::resources::ascii::*;
use crate::resources::ordering::*;

pub fn spawn_boxes(mut commands: Commands, ascii: Res<AsciiSheet>) {
    for n in 0..10 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: ascii.0.clone(),
                sprite: TextureAtlasSprite {
                    index: AsciiIndex::HalfSquare.into(),
                    color: Color::GREEN,
                    ..default()
                },
                transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    LayerOrder::BackgroundLayer.into(),
                )),
                ..default()
            },
            Name::new(format!("Item{:?}", n)),
        ));
    }
}