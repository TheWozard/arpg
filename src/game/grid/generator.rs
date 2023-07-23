use crate::game::*;
use crate::resources::*;

pub struct Cursor;
impl Generator for Cursor {
    fn generate(&self, commands: &mut Commands, ascii: &Res<AsciiSheet>) {
        spawn_cursor(commands, ascii)
    }
}

pub fn spawn_cursor(commands: &mut Commands, ascii: &Res<ascii::AsciiSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ascii.atlas.clone(),
            sprite: TextureAtlasSprite {
                index: ascii::AsciiIndex::FullSquare.into(),
                color: palette::game::ITEM.with_a(0.2),
                ..default()
            },
            transform: Transform::from_scale(ascii::ASCII_SCALE).with_translation(Vec3::new(
                0.,
                0.,
                layers::PLAYER,
            )),
            ..default()
        },
        Name::new("Cursor"),
        grid::GridTracked::default(),
        grid::GridCursorFollowHint,
        actors::GameHint::default(),
    ));
}

pub fn spawn_selection_cursor(
    transform: Vec2,
    commands: &mut Commands,
    ascii: &Res<ascii::AsciiSheet>,
) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: ascii.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: ascii::AsciiIndex::FullSquare.into(),
                    color: palette::game::ITEM.with_a(0.1),
                    ..default()
                },
                transform: Transform::from_scale(ascii::ASCII_SCALE)
                    .with_translation(transform.extend(layers::PLAYER)),
                ..default()
            },
            Name::new("Selection"),
            grid::GridTracked::default(),
            actors::GameHint::default(),
        ))
        .id()
}
