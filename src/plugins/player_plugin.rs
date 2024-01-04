use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{states::AppState, Layer, components::{Player, MapIdx}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, (move_player).run_if(in_state(AppState::InGame)));
    }
}

fn setup(
	mut commands: Commands, 
	mut sprites: PxAssets<PxSprite>,
) {
	let player = sprites.load("/public/sprite/player.png");
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: player,
            position: IVec2::new(36, 36).into(),
            ..default()
        },
		Player { 
			dest: IVec2::new(36,36), 
			time: 0., 
			moving: false,
			next_map: None,
		}
    ));
}


fn move_player(
    time: Res<Time>,
    mut player_q: Query<(&mut Player, &mut PxPosition), With<Player>>) {

    let (mut player, mut pos) = player_q.single_mut();
    
    if !pos.eq(&player.dest)
    {
		let time = time.delta_seconds();
		player.time += time;
		
		if player.time > 0.1 {

			let x = if player.dest.x > pos.x {
				pos.x + 1
			} else if player.dest.x < pos.x {
				pos.x - 1
			} else {
				pos.x
			};
			let y = if player.dest.y > pos.y {
				pos.y + 1
			} else if player.dest.y < pos.y {
				pos.y - 1
			} else {
				pos.y
			};
			
			**pos = IVec2::new(x, y);
			player.time = 0.;
		}
    } else if player.moving {
		player.moving = false;
		let tile_x = player.dest.x as u32 / 8;
		if tile_x == 7 {
			// Change the map
			player.next_map = Some(MapIdx::Right);
			// Teleport the player
			player.dest.x = 4;
			**pos = IVec2::new(4, player.dest.y);
		}
		if tile_x == 0 {
			// Change the map
			player.next_map = Some(MapIdx::Start);
			// Teleport the player
			player.dest.x = 7 * 8 + 4;
			**pos = IVec2::new(player.dest.x, player.dest.y);
		}
	}
}