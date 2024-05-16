use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{components::Layer, states::AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PxPlugin::<Layer>::new(
            UVec2::splat(64),
            "/public/palette/palette.png".into(),
        ))
        .add_systems(OnEnter(AppState::MainMenu), setup)
        .add_systems(OnExit(AppState::MainMenu), exit)
        .add_systems(Update, interact_buttons);
    }
}

fn setup(
    mut commands: Commands,
    mut sprites: PxAssets<PxSprite>,
    mut filters: PxAssets<PxFilter>,
    mut cursor: ResMut<PxCursor>,
    mut typefaces: PxAssets<PxTypeface>,
) {
    commands.spawn(Camera2dBundle {
        // Centering the camera
        transform: Transform::from_translation((Vec2::new(0., 0.)).extend(999.9)),
        ..Default::default()
    });

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

    // Filter-based button
    let button_idle = sprites.load("/public/sprite/btn.png");
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: button_idle,
            position: IVec2::new(32, 32).into(),
            ..default()
        },
        PxButtonFilterBundle {
            bounds: UVec2::new(31, 8).into(),
            idle: filters.load("/public/palette/palette.png").into(),
            hover: filters.load("/public/filter/inverse.png").into(),
            click: filters.load("/public/filter/inverse.png").into(),
        },
        Button,
        Menu,
    ));

    // Spawn text

    let typeface_bold = typefaces.load(
        "/public/typeface/typeface_bold.png",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        // Equivalent to, for example, `vec![PxSeparatorConfig { character: ' ', width: 4 }]`
        [(' ', 4)],
    );

    commands.spawn((
        PxTextBundle::<Layer> {
            text: "ISLAND".into(),
            typeface: typeface_bold,
            rect: IRect::new(0, 0, 64, 64).into(),
            alignment: PxAnchor::Custom(Vec2::new(0.5, 0.9)),
            ..default()
        },
        Menu,
    ));

    let typeface = typefaces.load(
        "/public/typeface/typeface.png",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        // Equivalent to, for example, `vec![PxSeparatorConfig { character: ' ', width: 4 }]`
        [(' ', 4)],
    );

    commands.spawn((
        PxTextBundle::<Layer> {
            text: "EXPLORE".into(),
            typeface,
            rect: IRect::new(0, 0, 64, 64).into(),
            alignment: PxAnchor::Custom(Vec2::new(0.5, 0.7)),
            ..default()
        },
        Menu,
    ));
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct Button;

fn interact_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    clicks: Query<(), (With<Button>, Added<PxClick>)>,
) {
    for () in &clicks {
        next_state.set(AppState::InGame);
    }
}

fn exit(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
