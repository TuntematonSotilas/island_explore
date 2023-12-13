use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{states::AppState, Layer, components::Player};

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
            position: IVec2::new(32, 32).into(),
            ..default()
        },
		Player { dest: IVec2::new(32,32), time: 0. }
    ));
}


fn move_player(
    time: Res<Time>,
    mut player_q: Query<(&mut Player, &mut PxPosition), With<Player>>) {

    let (mut player, mut pos) = player_q.single_mut();
    

    if !pos.eq(&player.dest)
    {
		
        //info!("move");

		let time = time.delta_seconds();
		player.time += time;
		
		if player.time > 0.1 {

			//info!("pos {0},{1} / dest {2}/{3}", pos.x, pos.y, player.dest.x, player.dest.y);

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
		
    }
}