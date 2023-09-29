use bevy::prelude::*;
use seldom_pixel::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
			PxPlugin::<Layer>::new(UVec2::splat(16), "/public/palette/palette.png".into()),
		)
		.add_systems(Startup, setup)
		.add_systems(Update, interact_buttons);
    }
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

	// Switch to an in-game cursor to show the player that they can click on things
	let idle = filters.load("/public/filter/mouse.png");
	*cursor = PxCursor::Filter {
        idle: idle.clone(),
        left_click: filters.load("/public/filter/mouseclick.png"),
        right_click: idle,
    };

	// Background
    let bkg = sprites.load("/public/sprite/bkg.png");
    commands.spawn(
        PxSpriteBundle ::<Layer> {
            sprite: bkg,
            position: IVec2::new(0, 0).into(),
            ..Default::default()
        });
	
    // Filter-based button
    let button_idle = sprites.load("/public/sprite/btn.png");
	commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: button_idle,
            position: IVec2::new(8, 12).into(),
            ..default()
        },
        PxButtonFilterBundle {
            bounds: UVec2::new(8, 4).into(),
            idle: filters.load("/public/palette/palette.png").into(),
            hover: filters.load("/public/filter/btnhover.png").into(),
            click: filters.load("/public/filter/btnclick.png").into(),
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
