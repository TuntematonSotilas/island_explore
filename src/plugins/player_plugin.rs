use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{states::AppState, Layer, Player};

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
	let runner = sprites.load_animated("/public/sprite/player.png", 2);
    
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: runner.clone(),
            position: IVec2::new(32, 32).into(),
            ..default()
        },
		Player { dest: IVec2::new(32,32) }
        /*PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Loop,
            ..default()
        },*/
    ));
}


fn move_player(
    time: Res<Time>,
    mut player_q: Query<(&Player, &mut PxPosition), With<Player>>) {

    let (player, mut pos) = player_q.single_mut();
    
    if !pos.eq(&player.dest)
    {
        debug!("move");
		// move our asteroids along the X axis
        // at a speed of 1.0 units per second

		let inc = (1. * time.delta_seconds()) as i32;

		debug!("inc {0}", inc);

		let x = if player.dest.x > pos.x {
			pos.x + inc
		} else {
			pos.x - inc
		};
		let y = if player.dest.y > pos.y {
			pos.y + inc
		} else {
			pos.y - inc
		};
        
		debug!("x/y {0},{1}", x, y);

        **pos = IVec2::new(x, y);
    }
}