use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    
    // hide loader
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("loader")
        .unwrap()
        .set_class_name("hide");

   // Circle
   commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });


}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.3, 0.)))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(400., 400.),
                    ..default()
                }),
                ..default()
            }))
        .add_systems(Startup, setup)
        .run();
}
