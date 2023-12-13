use bevy::prelude::*;

#[derive(Component)]
pub struct Player 
{
    pub dest: IVec2,
	pub time: f32,
}

#[derive(Component)]
pub struct MapClick;