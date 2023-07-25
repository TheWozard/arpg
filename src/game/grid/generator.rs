use crate::game::*;
use crate::resources::*;

pub struct Cursor;
impl Generator for Cursor {
    fn generate(
        &self,
        commands: &mut Commands,
        resources: &Res<Resources>,
        grid: &Res<grid::Grid>,
    ) {
        spawn_cursor(commands, resources, grid)
    }
}

pub fn spawn_cursor(commands: &mut Commands, resources: &Res<Resources>, grid: &Res<grid::Grid>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: resources.ascii.atlas.clone(),
            sprite: TextureAtlasSprite {
                index: ascii::AsciiIndex::FullSquare.into(),
                color: palette::game::PLAYER.with_a(0.5),
                ..default()
            },
            transform: Transform::from_scale(ascii::ASCII_SCALE)
                .with_translation(grid.world_position_on_top_from_grid_position(&IVec2::new(0, 0))),
            ..default()
        },
        Name::new("Cursor"),
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
                    color: palette::game::ITEM.with_a(0.5),
                    ..default()
                },
                transform: Transform::from_scale(ascii::ASCII_SCALE).with_translation(transform),
                ..default()
            },
            Name::new("Selection"),
            cleanup::CleanupHint,
        ))
        .id()
}
