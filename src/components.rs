use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub dest: IVec2,
    pub time: f32,
    pub moving: bool,
    pub next_map: Option<MapIdx>,
    pub current_map: MapIdx,
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct MapClick;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct TileType {
    pub clickable: bool,
    pub border: Option<TileBorder>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TileBorder {
    pub goto_map: MapIdx,
    pub teleport_x: Option<i32>,
    pub teleport_y: Option<i32>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MapIdx {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}
