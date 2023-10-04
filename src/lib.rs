#![allow(clippy::needless_pass_by_value)]

extern crate wasm_bindgen;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use plugins::menu_plugin::MenuPlugin;
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
        ))
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::hex("aad9ff").unwrap_or_default()))
        .run();
}
