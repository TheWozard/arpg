use crate::game::*;
use crate::resources::*;

pub struct Cursor;
impl Generator for Cursor {
    fn generate(&self, commands: &mut Commands, resources: &Res<Resources>, _: &Res<grid::Grid>) {
        spawn_cursor(commands, resources)
    }
}

pub fn spawn_cursor(commands: &mut Commands, resources: &Res<Resources>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: resources.ascii.atlas.clone(),
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
        cleanup::CleanupHint,
    ));
}

pub fn spawn_selection_cursor(
    transform: Vec3,
    commands: &mut Commands,
    resources: &Res<Resources>,
) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: resources.ascii.atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: ascii::AsciiIndex::FullSquare.into(),
                    color: palette::game::ITEM.with_a(0.1),
                    ..default()
                },
                transform: Transform::from_scale(ascii::ASCII_SCALE).with_translation(transform),
                ..default()
            },
            Name::new("Selection"),
            grid::GridTracked::default(),
            cleanup::CleanupHint,
        ))
        .id()
}
