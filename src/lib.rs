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
    let (w, h) = get_window_size();
    let size = if h > w {
        w // Mobile
    } else {
        h * 0.8 // Desktop
    };
    
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: Vec2::splat(size).into(),
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

#[px_layer]
struct Layer;