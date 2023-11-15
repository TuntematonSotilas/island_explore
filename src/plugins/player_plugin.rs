use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{states::AppState, Layer};

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
	let runner = sprites.load_animated("/public/sprite/runner.png", 8);
    // Despawn at the end
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: runner.clone(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        PxAnimationBundle::default(),
    ));
    // Add the `PxAnimationFinished` component at the end
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: runner.clone(),
            position: IVec2::new(13, 0).into(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Mark,
            ..default()
        },
    ));
     // Loop
     commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: runner.clone(),
            position: IVec2::new(26, 0).into(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Loop,
            ..default()
        },
    ));
    // Backward
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: runner.clone(),
            position: IVec2::new(39, 0).into(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        PxAnimationBundle {
            direction: PxAnimationDirection::Backward,
            on_finish: PxAnimationFinishBehavior::Loop,
            ..default()
        },
    ));
    // Dither between frames
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: runner,
            position: IVec2::new(39, 18).into(),
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Loop,
            frame_transition: PxAnimationFrameTransition::Dither,
            ..default()
        },
    ));


}

