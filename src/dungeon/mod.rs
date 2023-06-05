use crate::resources::ascii::*;
use crate::resources::palette::*;
use crate::resources::LayerOrder;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

/// DungeonPlugin is responsible for controlling dungeon generation
pub struct DungeonPlugin;
impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin);
        app.add_startup_system(startup);
    }
}

fn startup(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let map_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    let mut rng = rand::thread_rng();
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(16 as u32),
                    // color: Color::RED.into(),
                    color: TILE_COLORS[rng.gen_range(0..TILE_COLORS.len())].into(),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 11., y: 11. };
    // let tile_size = TILE_SIZE.into();
    let grid_size = TILE_SIZE.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(ascii.image.clone()),
        tile_size,
        // spacing: TilemapSpacing { x: 1., y: 1. },
        transform: Transform {
            scale: ASCII_SCALE,
            translation: Vec3::new(0., 0., LayerOrder::BackgroundLayer.index()),
            ..default()
        },
        // .with_scale(ASCII_SCALE),
        ..Default::default()
    });
}
