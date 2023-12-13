use bevy::prelude::*;
use seldom_pixel::{prelude::*, cursor::PxCursorPosition};
use bevy_ecs_tilemap::prelude::*;

use crate::{states::AppState, Layer, components::Player, components::MapClick};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
			.add_systems(Update, (click, mapclick_hide).run_if(in_state(AppState::InGame)));

    }
}

fn setup(
	mut commands: Commands, 
	mut tilesets: PxAssets<PxTileset>
) {
	let map_size = TilemapSize { x: 8, y: 8 };
    let mut storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {

            let isl = y >= 1 && y <= 5 && x >= 1 && x <= 6;
            let idx = if isl {
				x + (6 * (y - 1)) // Island
			} else {
				0 //Sea
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
    ));
}

fn mapclick_hide(
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
) {

	if buttons.just_released(MouseButton::Left) {

        let mut player = player_q.single_mut();
        
        // Only when player not moving 
        if !player.moving {
            
            if let Some(cur_pos) = **cursor_pos {

                let x = cur_pos.x;
                let y = cur_pos.y;
                let dest = IVec2::new(x as i32, y as i32);
    
                info!("click : {0} {1}", x, y);
    
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
