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
	let map_size = TilemapSize { x: 8, y: 8 };
    let mut storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
			let mx = x  % 2;
			let my = y  % 2;

            let sea = x == 0 || y == 0 || x == 7 || y == 7;
            let idx = if (mx == 0 && my == 0) || ( mx == 1 && my == 1) {
                if sea { 2 } else { 0 }
            } else {
                if sea { 3 } else { 1 }
            };
			
            // Each tile must be added to the `TileStorage`
            storage.set(
                &TilePos { x, y },
                commands
                    .spawn(PxTileBundle {
                        texture: TileTextureIndex(idx),
                        ..default()
                    })
                    .id(),
            );
        }
    }

    // Spawn the map
    commands.spawn(PxMapBundle::<Layer> {
        size: map_size,
        storage,
        tileset: tilesets.load("/public/tileset/tileset.png", UVec2::splat(8)),
        ..default()
    });
}

