extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use bevy::prelude::*;

use plugins::menu_plugin::MenuPlugin;

mod plugins;

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
	let (mut w, mut h) = get_window_size();
	if w > 512. {
		w = 512.;
	}
	if h > 512. {
		h = 512.;
	}
	App::new()
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					resolution: Vec2::new(w, h).into(),
					..default()
				}),
				..default()
			}),
			MenuPlugin
		))
		.insert_resource(ClearColor(Color::hex("aad9ff").unwrap()))
		.run();
}

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