use bevy::prelude::*;
use seldom_pixel::{prelude::*, cursor::PxCursorPosition};
use bevy_ecs_tilemap::prelude::*;

use crate::{states::AppState, Layer, components::{Map, Player, TileType}, components::{MapClick, MapIdx}};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
			.add_systems(Update, (click, despawn_mapclick, change_map)
				.run_if(in_state(AppState::InGame)));

    }
}

fn setup(
	commands: Commands, 
	tilesets: PxAssets<PxTileset>
) {
    map_spawn(commands, tilesets, MapIdx::Start);
}

fn map_spawn(
    mut commands: Commands, 
    mut tilesets: PxAssets<PxTileset>,
    map_idx: MapIdx) {
    
    let map_size = TilemapSize { x: 8, y: 8 };
    let mut storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        let modu_x = x % 2;
        for y in 0..map_size.y {    
            let modu_y = y % 2;
            let modu = if modu_x == modu_y { 0 } else { 1 };
            
            let sea: bool;
            let border_bott: bool;
            let mut border_left = false;
            
            if map_idx == MapIdx::Start {
                sea = x == 0 || y == 0 || y == 7;
                border_bott = y == 0 && x != 0;
                border_left = x == 0 && y != 0 && y != 7;
            } else {
                sea = y == 0 || y == 7 ||x == 6 || x == 7;
                border_bott = y == 0 && x != 6 && x != 7;
            }
            
            let mut isl = false;
            let idx = if border_left { 
                4 
            } else if border_bott {
                3
            } else if sea {
                0
            } else {
                isl = true;
				1 + modu 
			};
			
            storage.set(
                &TilePos { x, y },
                commands
                    .spawn((
                        PxTileBundle {
                            texture: TileTextureIndex(idx),
                            ..default()
                        },
                        TileType { clickable: isl })
                    )
                    .id(),
            );
        }
    }

    // Spawn the map
    commands.spawn((
		PxMapBundle::<Layer> {
			size: map_size,
			storage,
			tileset: tilesets.load("/public/tileset/tileset.png", UVec2::splat(8)),
			..default()
		},
		PxAnimationBundle {
            // Use millis_per_animation to have each tile loop at the same time
            duration: PxAnimationDuration::millis_per_frame(1000),
            on_finish: PxAnimationFinishBehavior::Loop,
			frame_transition: PxAnimationFrameTransition::None,
            ..default()
        },
        Map
    ));
}

fn despawn_mapclick(
	mut commands: Commands,
	player_q: Query<&Player>,
    mapclick_q: Query<Entity, &MapClick>,
) {	
	let player = player_q.single();
	// Remove Map Click Sprite
	if !player.moving && !mapclick_q.is_empty() {
		let mapclick = mapclick_q.single();
		commands.entity(mapclick).despawn();
	}    
}

fn click(
    mut commands: Commands, 
	cursor_pos: Res<PxCursorPosition>,
    buttons: Res<Input<MouseButton>>,
	mut player_q: Query<&mut Player>,
    mut sprites: PxAssets<PxSprite>,
    tilemap_q: Query<&TileStorage>,
    tile_query: Query<&mut TileType>,
) {

	if buttons.just_released(MouseButton::Left) {

        let tile_storage = tilemap_q.single();
        let mut player = player_q.single_mut();
        
        // Only when player not moving 
        if !player.moving {
            
            if let Some(cur_pos) = **cursor_pos {

				//Get center of the clicked tile
                let x = ((cur_pos.x as f64 / 8.).ceil() as i32) * 8 - 4;
                let y = ((cur_pos.y as f64 / 8.).ceil() as i32) * 8 - 4;
                let dest = IVec2::new(x as i32, y as i32);
                
                let tile_x = x as u32 / 8;
                let tile_y = y as u32 / 8;

                let mut clickable: bool = false;
                let tile_pos = TilePos { x: tile_x, y: tile_y };
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    if let Ok(tile_type) = tile_query.get(tile_entity) {
                        clickable = tile_type.clickable;
                    }
                };

                if clickable {
                    // Spawn Map Click Sprite
                    let mapclick = sprites.load("/public/sprite/mapclick.png");
                    commands.spawn((
                        PxSpriteBundle::<Layer> {
                            sprite: mapclick,
                            position: dest.into(),
                            ..default()
                        },
                        MapClick
                    ));

                    player.moving = true;
                    player.dest = dest;
                }
            }
        }
	}
}

fn change_map(
    mut player_q: Query<&mut Player>,
    mut commands: Commands, 
    map_q: Query<Entity, &Map>,
    tilesets: PxAssets<PxTileset>,
) {
    let mut player = player_q.single_mut();
    if player.next_map.is_some() {
        let map_idx = player.next_map.to_owned().unwrap();

        info!("go to next map : {:?}", map_idx);
        player.next_map = None;
        // Despawn the map
        let map = map_q.single();
		commands.entity(map).despawn();
        // Spawn the map
        
        map_spawn(commands, tilesets, map_idx);
    }
}
