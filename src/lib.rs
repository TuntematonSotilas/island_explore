#![allow(clippy::needless_pass_by_value)]

extern crate wasm_bindgen;

use bevy::{asset::AssetMetaCheck, prelude::*};
use wasm_bindgen::prelude::*;
use bevy_wasm_window_resize::WindowResizePlugin;

use plugins::{
    map_plugin::MapPlugin, menu_plugin::MenuPlugin, obj_plugin::ObjPlugin,
    player_plugin::PlayerPlugin,
};
use states::AppState;

mod components;
mod plugins;
mod states;

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)

        .add_plugins((
            WindowResizePlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game".into()),
                    ..default()
                }),
                ..default()
            }),
            MenuPlugin,
            MapPlugin,
            PlayerPlugin,
            ObjPlugin,
        ))
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::hex("#1f93ac").unwrap_or_default()))
        .run();
}