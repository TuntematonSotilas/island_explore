use bevy::prelude::*;

#[derive(Component)]
pub struct Player 
{
    pub dest: IVec2,
	pub time: f32,
    pub moving: bool,
    pub go_next_map: bool,
}

#[derive(Component)]
pub struct MapClick;

#[derive(Component)]
pub struct TileType 
{
    pub clickable: bool,
}