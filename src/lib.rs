#![allow(clippy::needless_pass_by_value)]

extern crate wasm_bindgen;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use seldom_pixel::prelude::*;

use plugins::{menu_plugin::MenuPlugin, map_plugin::MapPlugin};
use states::AppState;

mod plugins;
mod states;

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: Vec2::splat(512.).into(),
                    ..default()
                }),
                ..default()
            }),
            MenuPlugin,
			MapPlugin,
        ))
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::hex("#aad9ff").unwrap_or_default()))
        .run();
}


#[px_layer]
struct Layer;
