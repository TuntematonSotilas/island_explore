use bevy::prelude::*;

#[derive(Component)]
pub struct Player 
{
    pub dest: IVec2,
	pub time: f32,
    pub moving: bool,
}

#[derive(Component)]
pub struct MapClick;