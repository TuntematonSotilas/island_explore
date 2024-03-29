use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use seldom_pixel::{cursor::PxCursorPosition, prelude::*};

use crate::{
    components::{Direct, Layer, Map, MapClick, MapIdx, Player, TileBorder, TileType},
    states::AppState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (click, hide_mapclick, change_map).run_if(in_state(AppState::InGame)),
            );
    }
}

fn setup(commands: Commands, tilesets: PxAssets<PxTileset>, sprites: PxAssets<PxSprite>) {
    map_spawn(commands, tilesets, MapIdx::LeftTop, sprites, false);
}

fn map_spawn(
    mut commands: Commands,
    mut tilesets: PxAssets<PxTileset>,
    map_idx: MapIdx,
    mut sprites: PxAssets<PxSprite>,
    is_change: bool,
) {
    let map_size = TilemapSize { x: 8, y: 8 };
    let mut storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        let modu_x = x % 2;
        for y in 0..map_size.y {
            let modu_y = y % 2;
            let modu = u32::from(modu_x != modu_y);

            let sea: bool;
            let mut border_bott = false;
            let mut border_left = false;
            let mut border_left_top = false;
            let mut border_right_top = false;
            let mut border_bottom_right = false;

            if map_idx == MapIdx::LeftTop {
                sea = y == 7;
                border_left = x == 0 && y != 7;
                border_left_top = x == 0 && y == 6;
            } else if map_idx == MapIdx::RightTop {
                sea = x == 7 || y == 7;
                border_right_top = x == 6 && y == 6;
            } else if map_idx == MapIdx::LeftBottom {
                sea = x == 0 || y == 0;
                border_bott = y == 0 && x != 0;
                border_left = x == 0 && y != 0;
            } else {
                sea = x == 7;
                border_bott = y == 0 && x != 7;
                border_bottom_right = y == 0 && x == 6;
            }

            let mut isl = false;
            let idx = if border_bottom_right {
                7  
            } else if border_right_top {
                6
            } else if border_left_top {
                5
            } else if border_left {
                4
            } else if border_bott {
                3
            } else if sea {
                0
            } else {
                isl = true;
                1 + modu
            };

            let border: Option<TileBorder> = if isl { get_border(x, y, map_idx) } else { None };

            storage.set(
                &TilePos { x, y },
                commands
                    .spawn((
                        PxTileBundle {
                            texture: TileTextureIndex(idx),
                            ..default()
                        },
                        TileType {
                            clickable: isl,
                            border,
                        },
                    ))
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
        Map { is_new: true },
    ));

    // Spawn the Map Click Sprite
    if !is_change {
        let mapclick = sprites.load("/public/sprite/mapclick.png");
        commands.spawn((
            PxSpriteBundle::<Layer> {
                sprite: mapclick,
                position: IVec2::new(0, 0).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            MapClick,
        ));
    }
}

fn hide_mapclick(player_q: Query<&Player>, mut mapclick_q: Query<&mut Visibility, With<MapClick>>) {
    let player = player_q.single();
    if !player.moving {
        // Hide Map Click Sprite
        let mut visibility = mapclick_q.single_mut();
        *visibility = Visibility::Hidden;
    }
}

#[allow(clippy::cast_possible_truncation)]
fn click(
    cursor_pos: Res<PxCursorPosition>,
    buttons: Res<Input<MouseButton>>,
    mut player_q: Query<&mut Player>,
    tilemap_q: Query<&TileStorage>,
    tile_query: Query<&mut TileType>,
    mut mapclick_q: Query<(&mut Visibility, &mut PxPosition), With<MapClick>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let tile_storage = tilemap_q.single();
        let mut player = player_q.single_mut();

        // Only when player not moving
        if !player.moving {
            if let Some(cur_pos) = **cursor_pos {
                //Get center of the clicked tile

                let f_x = f64::from(cur_pos.x);
                let f_y = f64::from(cur_pos.y);
                let mut x = ((f_x / 8.).ceil() * 8.) as i32;
                if x > 0 {
                    x -= 4;
                };
                let mut y = ((f_y / 8.).ceil() * 8.) as i32;
                if y > 0 {
                    y -= 4;
                };
                let dest = IVec2::new(x, y);

                let tile_x = (x / 8).unsigned_abs();
                let tile_y = (y / 8).unsigned_abs();

                let mut clickable: bool = false;
                let tile_pos = TilePos {
                    x: tile_x,
                    y: tile_y,
                };
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    if let Ok(tile_type) = tile_query.get(tile_entity) {
                        clickable = tile_type.clickable;
                    }
                };

                if clickable {
                    // Show Map Click Sprite
                    let (mut visibility, mut pos) = mapclick_q.single_mut();
                    *visibility = Visibility::Visible;
                    **pos = dest;
                    player.moving = true;
                    player.dest = dest;
                }
            }
        }
    }
}

fn change_map(
    mut player_q: Query<&mut Player>,
    mut commands: Commands,
    map_q: Query<Entity, &Map>,
    tilesets: PxAssets<PxTileset>,
    sprites: PxAssets<PxSprite>,
) {
    let mut player = player_q.single_mut();
    if player.next_map.is_some() {
        let map_idx = player.next_map.unwrap();
        player.next_map = None;
        player.current_map = map_idx;
        // Despawn the map
        let map = map_q.single();
        commands.entity(map).despawn();
        // Spawn the map
        map_spawn(commands, tilesets, map_idx, sprites, true);
    }
}

fn get_border(tile_x: u32, tile_y: u32, map_idx: MapIdx) -> Option<TileBorder> {
    let mut border: Option<TileBorder> = None;
    let mut goto_map = None;
    let mut teleport_x = None;
    let mut teleport_y = None;
    let mut direct = Direct::Right;
    if tile_x == 7 {
        if map_idx == MapIdx::LeftTop {
            goto_map = Some(MapIdx::RightTop);
        }
        if map_idx == MapIdx::LeftBottom {
            goto_map = Some(MapIdx::RightBottom);
        }
        direct = Direct::Right;
        teleport_x = Some(4);
    } else if tile_x == 0 {
        if map_idx == MapIdx::RightTop {
            goto_map = Some(MapIdx::LeftTop);
        }
        if map_idx == MapIdx::RightBottom {
            goto_map = Some(MapIdx::LeftBottom);
        }
        direct = Direct::Left;
        teleport_x = Some(7 * 8 + 4);
    } else if tile_y == 0 {
        if map_idx == MapIdx::LeftTop {
            goto_map = Some(MapIdx::LeftBottom);
        }
        if map_idx == MapIdx::RightTop {
            goto_map = Some(MapIdx::RightBottom);
        }
        direct = Direct::Bottom;
        teleport_y = Some(7 * 8 + 4);
    } else if tile_y == 7 {
        if map_idx == MapIdx::LeftBottom {
            goto_map = Some(MapIdx::LeftTop);
        }
        if map_idx == MapIdx::RightBottom {
            goto_map = Some(MapIdx::RightTop);
        }
        direct = Direct::Top;
        teleport_y = Some(4);
    }
    if let Some(goto_map) = goto_map {
        border = Some(TileBorder {
            goto_map,
            direct,
            teleport_x,
            teleport_y,
        });
    }
    border
}
