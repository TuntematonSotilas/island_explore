use bevy::prelude::*;
use seldom_pixel::prelude::*;

#[px_layer]
pub struct Layer;

#[derive(Component)]
pub struct Player;
    /*pub prev: IVec2,
    pub dest: IVec2,
    pub time: f32,
    pub moving: bool,
    pub next_map: Option<MapIdx>,
    pub current_map: MapIdx,
    pub prev_direct: Direct,
    pub new_direct: Direct,
    pub collide: bool,
    pub reset_dir: bool,*/

/*
#[derive(Component)]
pub struct Map {
    pub is_new: bool,
}

#[derive(Component)]
pub struct MapClick;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct TileType {
    pub clickable: bool,
    pub border: Option<TileBorder>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TileBorder {
    pub goto_map: MapIdx,
    pub direct: Direct,
    pub teleport_x: Option<i32>,
    pub teleport_y: Option<i32>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Direct {
    Right,
    Left,
    Top,
    Bottom,
    Stop,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MapIdx {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}
*/