extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use bevy::prelude::*;
use seldom_pixel::prelude::*;

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
			PxPlugin::<Layer>::new(UVec2::splat(16), "/public/palette/palette_1.png".into()),
		))
		.insert_resource(ClearColor(Color::hex("#aad9ff").unwrap()))
		.add_systems(Startup, setup)
        .add_systems(Update, interact_buttons)
		.run();
}


fn setup(
    mut commands: Commands,
    mut sprites: PxAssets<PxSprite>,
    mut filters: PxAssets<PxFilter>,
    mut cursor: ResMut<PxCursor>,
) {
    commands.spawn(Camera2dBundle::default());
    
    // Hide loader
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("loader")
        .unwrap()
        .set_class_name("hide");

	let idle = filters.load("/public/filter/invert.png");

    // Switch to an in-game cursor to show the player that they can click on things
    *cursor = PxCursor::Filter {
        idle: idle.clone(),
        left_click: filters.load("/public/filter/invert_dim.png"),
        right_click: idle,
    };

    let bkg = sprites.load("/public/sprite/bkg.png");

    commands.spawn(
        PxSpriteBundle ::<Layer> {
            sprite: bkg,
            position: IVec2::new(0, 0).into(),
           // layers: PxFilterLayers::single_clip(Layer),
            ..Default::default()
        });

    let button_idle = sprites.load("/public/sprite/button_idle.png");

    // Filter-based button
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: button_idle,
            position: IVec2::new(8, 12).into(),
            ..default()
        },
        PxButtonFilterBundle {
            bounds: UVec2::new(8, 4).into(),
            idle: filters.load("/public/palette/palette_1.png").into(),
            hover: filters.load("/public/filter/hover.png").into(),
            click: filters.load("/public/filter/click.png").into(),
        },
        Button,
    ));

}

#[derive(Component)]
struct Button;

fn interact_buttons(
    hovers: Query<(), (With<Button>, Added<PxHover>)>,
    clicks: Query<(), (With<Button>, Added<PxClick>)>,
) {
    for _ in &hovers {
        info!("Hover!");
    }

    for _ in &clicks {
        info!("Click!");
    }
}

#[px_layer]
struct Layer;
