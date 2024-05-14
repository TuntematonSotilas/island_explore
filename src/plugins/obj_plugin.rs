/*use bevy::prelude::*;
use seldom_pixel::prelude::*;
use rand::Rng;

use crate::{
    components::{Layer, Map, MapIdx, Player, Tree},
    states::AppState,
    resources::Trees
};

pub struct ObjPlugin;

impl Plugin for ObjPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Trees(Vec::new()))
            .add_systems(OnEnter(AppState::InGame), generate_pos)
            .add_systems(Update, (trees_spawn, set_to_top).run_if(in_state(AppState::InGame)),
        );
    }
}

fn trees_spawn(
    mut commands: Commands,
    sprites: PxAssets<PxSprite>,
    mut map_q: Query<&mut Map>,
    player_q: Query<&Player>,
    tree_q: Query<Entity, With<Tree>>,
) {
    let mut map = map_q.single_mut();
    let player = player_q.single();

    if map.is_new {
        map.is_new = false;
        // Despawn all trees
        for entity in &tree_q {
            commands.entity(entity).despawn();
        }
        // Spawn trees
        if player.current_map == MapIdx::LeftTop {
            let pos = IVec2::new(44, 44);
            tree_spawn(commands, sprites, pos, false);
        }
    }
}

fn tree_spawn(mut commands: Commands, mut sprites: PxAssets<PxSprite>, pos: IVec2, is_small: bool) {
    let name = if is_small { "tree_small" } else { "tree" };
    let path = format!("/public/sprite/{name}.png");
    let tree = sprites.load(path);
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: tree,
            position: pos.into(),
            ..default()
        },
        Tree,
    ));
}

fn set_to_top(
    mut tree_q: Query<&PxPosition, (With<Tree>, Without<Player>)>,
    mut player_q: Query<(Entity, &mut Player, &PxPosition), With<Player>>,
    mut commands: Commands,
) {
    if !tree_q.is_empty() {
        let (entity_p, mut player, pos_player) = player_q.single_mut();
        let pos_tree = tree_q.single_mut();

        let collide = pos_player.x > pos_tree.x - 4
            && pos_player.x < pos_tree.x + 4
            && pos_player.y > pos_tree.y - 4
            && pos_player.y < pos_tree.y + 4;

        if collide {
            info!("collide");
            player.collide = true;
            commands.entity(entity_p).remove::<PxAnimationBundle>();
        }

        if !collide && player.collide {
            info!("reset");
            player.collide = false;
            player.reset_dir = true;
        }
    }
}

fn generate_pos(/*trees: Res<Trees>*/) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..8);
    let y = rng.gen_range(0..8);
    let pos = IVec2::new(x, y);
    info!(pos.x, pos.y);
}*/