#![allow(clippy::needless_pass_by_value)]

extern crate wasm_bindgen;

use bevy::{asset::AssetMetaCheck, prelude::*};
use wasm_bindgen::prelude::*;

use plugins::{
    map_plugin::MapPlugin, menu_plugin::MenuPlugin, 
    //obj_plugin::ObjPlugin,
    //player_plugin::PlayerPlugin,
};
use states::AppState;

mod components;
mod plugins;
mod states;
mod resources;

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    let (w, h) = get_window_size();
    let size = if h > w {
        w // Mobile
    } else {
        h * 0.8 // Desktop
    };

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: Vec2::splat(size).into(),
                    canvas: Some("#game".into()),
                    ..default()
                }),
                ..default()
            })
			.set(ImagePlugin::default_nearest()),
            MenuPlugin,
            MapPlugin,
            //PlayerPlugin,
            //ObjPlugin,
        ))
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::hex("#1f93ac").unwrap_or_default()))
        .run();
}

#[allow(clippy::cast_possible_truncation)]
fn get_window_size() -> (f32, f32) {
    let w = web_sys::window()
        .unwrap()
        .inner_width()
        .ok()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    let h = web_sys::window()
        .unwrap()
        .inner_height()
        .ok()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    (w, h)
}
