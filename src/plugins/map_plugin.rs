use bevy::prelude::*;
use seldom_pixel::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{states::AppState, Layer};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
    }
}

fn setup(
	mut commands: Commands, 
	mut tilesets: PxAssets<PxTileset>,
) {
	let map_size = TilemapSize { x: 4, y: 4 };
    let mut storage = TileStorage::empty(map_size);

    for x in 0..4 {
        for y in 0..4 {
            // Each tile must be added to the `TileStorage`
            storage.set(
                &TilePos { x, y },
                commands
                    .spawn(PxTileBundle {
                        texture: TileTextureIndex(0),
                        ..default()
                    })
                    .id(),
            );
        }
    }

    // Spawn the map
    commands.spawn(PxMapBundle::<MapLayer> {
        size: map_size,
        storage,
        tileset: tilesets.load("/public/tileset/tileset.png", UVec2::splat(4)),
        ..default()
    });
}

#[px_layer]
struct MapLayer;
