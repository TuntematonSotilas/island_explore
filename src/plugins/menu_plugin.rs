use bevy::prelude::*;
use seldom_pixel::prelude::*;

use crate::{states::AppState, Layer};

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
    // let bkg = sprites.load("/public/sprite/bkg.png");
    // commands.spawn(PxSpriteBundle::<Layer> {
    //     sprite: bkg,
    //     position: IVec2::new(0, 0).into(),
    //     ..Default::default()
    // });

    // Filter-based button
    let button_idle = sprites.load("/public/sprite/btn.png");
    commands.spawn((
        PxSpriteBundle::<Layer> {
            sprite: button_idle,
            position: IVec2::new(32, 32).into(),
            ..default()
        },
        PxButtonFilterBundle {
            bounds: UVec2::new(32, 8).into(),
            idle: filters.load("/public/palette/palette.png").into(),
            hover: filters.load("/public/filter/btnhover.png").into(),
            click: filters.load("/public/filter/btnclick.png").into(),
        },
        Button,
        Menu,
    ));

    let typeface = typefaces.load_animated(
        "/public/typeface/anim_typeface.png",
        // See the function signature of `load_animated`
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .map(|character| (character, 2)),
        // Equivalent to, for example, `vec![PxSeparatorConfig { character: ' ', width: 4 }]`
        [(' ', 4)],
    );

    // Spawn text
    commands.spawn((
        PxTextBundle::<Layer> {
            text: "ISL".into(),
            typeface,
            rect: IRect::new(IVec2::ZERO, IVec2::splat(64)).into(),
            alignment: PxAnchor::Custom(Vec2::new(0.5, 0.9)),
            ..default()
        },
        PxAnimationBundle {
            // Use millis_per_animation to have each character loop at the same time
            duration: PxAnimationDuration::millis_per_frame(333),
            on_finish: PxAnimationFinishBehavior::Loop,
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
    for _ in &clicks {
        next_state.set(AppState::InGame);
    }
}

fn exit(mut commands: Commands, query: Query<Entity, &Menu>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

