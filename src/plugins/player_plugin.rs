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
    //player_q: Query<&Player>, // TODO : group query
    mut player_q: Query<(&Player, &mut PxPosition), With<Player>>) {

    let (player, mut pos) = player_q.single_mut();
    
    if !pos.eq(&player.dest)
    {
        debug!("move");

        **pos = player.dest;
    }
}