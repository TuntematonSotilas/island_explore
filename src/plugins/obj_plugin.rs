use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{components::{Layer, Player, Tree}, states::AppState};

pub struct ObjPlugin;

impl Plugin for ObjPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), tree_spawn)
            .add_systems(
                Update,
                set_to_top.run_if(in_state(AppState::InGame)),
            );
    }
}

fn tree_spawn(mut commands: Commands, mut sprites: PxAssets<PxSprite>) {
	let tree = sprites.load("/public/sprite/tree.png");
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: tree,
            position: IVec2::new(44, 44).into(),
            ..default()
        },
        Tree { on_top: false },
    ));
}

fn set_to_top(
    mut tree_q: Query<(&mut Tree, &mut PxPosition), (With<Tree>, Without<Player>)>,
    player_q: Query<(&Player, &PxPosition), With<Player>>,
) {
    let (mut tree, mut pos_tree) = tree_q.single_mut();
    let (player, pos_player) = player_q.single();
    let collide = pos_player.x > pos_tree.x - 4 &&
            pos_player.x < pos_tree.x + 4 &&
            pos_player.y > pos_tree.y - 4 && 
            pos_player.y < pos_tree.y + 4;

    if collide && !tree.on_top {
        info!("set to top");
        pos_tree.x = pos_tree.x;
        if !player.moving {
            tree.on_top = true;
        }
    }
}