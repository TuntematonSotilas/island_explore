use std::cmp::Ordering;

use bevy::prelude::*;
use seldom_pixel::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    components::{MapIdx, Player, TileType, Direct},
    states::AppState,
    Layer,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, (move_player).run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, mut sprites: PxAssets<PxSprite>) {
    let player = sprites.load("/public/sprite/player.png");
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: player,
            position: IVec2::new(36, 36).into(),
            ..default()
        },
        Player {
			prev: IVec2::new(36, 36),
            dest: IVec2::new(36, 36),
            time: 0.,
            moving: false,
            next_map: None,
            current_map: MapIdx::LeftTop,
        },
    ));
}

fn move_player(
    time: Res<Time>, 
    mut player_q: Query<(&mut Player, &mut PxPosition), With<Player>>,
    tilemap_q: Query<&TileStorage>,
    tile_query: Query<&mut TileType>,
) {
    let (mut player, mut pos) = player_q.single_mut();

    if !pos.eq(&player.dest) {
        let time = time.delta_seconds();
        player.time += time;

        if player.time > 0.1 {
			player.prev = **pos;

            let x = match player.dest.x.cmp(&pos.x) {
                Ordering::Greater => pos.x + 1,
                Ordering::Less => pos.x - 1,
                Ordering::Equal => pos.x,
            };
            let y = match player.dest.y.cmp(&pos.y) {
                Ordering::Greater => pos.y + 1,
                Ordering::Less => pos.y - 1,
                Ordering::Equal => pos.y,
            };

            **pos = IVec2::new(x, y);
            player.time = 0.;
        }
    } else if player.moving {
        player.moving = false;
        // Get the tile border infos
        let mut border = None;
        let tile_storage = tilemap_q.single();

		let tile_x = player.dest.x.unsigned_abs() / 8;
        let tile_y = player.dest.y.unsigned_abs() / 8;
        let tile_pos = TilePos {
            x: tile_x,
            y: tile_y,
        };
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            if let Ok(tile_type) = tile_query.get(tile_entity) {
                border = tile_type.border;
            }
        };
        if let Some(border) = border {
			
			let is_good_direct = (border.direct == Direct::Right && player.dest.x > player.prev.x) 
				|| (border.direct == Direct::Left && player.dest.x < player.prev.x) 
				|| (border.direct == Direct::Top && player.dest.y > player.prev.y) 
				|| (border.direct == Direct::Bottom && player.dest.y < player.prev.y); 
			
			if is_good_direct {
				// Change the map
				player.next_map = Some(border.goto_map);
				// Teleport the player
				let mut teleport_x = player.dest.x;
				let mut teleport_y = player.dest.y;
				if let Some(tele_x) = border.teleport_x {
					player.dest.x = tele_x;
					teleport_x = tele_x;
				}
				if let Some(tele_y) = border.teleport_y {
					player.dest.y = tele_y;
					teleport_y = tele_y;
				}
				**pos = IVec2::new(teleport_x, teleport_y);
			}
           
        }        
    }
}
