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
	App::new()
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					resolution: Vec2::splat(512.).into(),
					..default()
				}),
				..default()
			}),
			MenuPlugin
		))
		.insert_resource(ClearColor(Color::hex("#aad9ff").unwrap()))
		.run();
}
