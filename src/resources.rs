use bevy::prelude::*;

/*
#[derive(Resource)]
pub struct Trees(pub Vec<IVec2>);
*/
    
#[derive(Default, Deref, DerefMut, Resource)]
pub struct CursorPos(Option<Vec2>);
