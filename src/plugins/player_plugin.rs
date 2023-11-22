use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{states::AppState, Layer, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
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
		Player
        /*PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Loop,
            ..default()
        },*/
    ));
	

}
