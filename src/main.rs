//! A basic APRG game
use bevy::{app::AppExit, prelude::*, window::PresentMode};
use rand::prelude::*;

pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 900.;

mod resources;
mod camera;
mod debug;
mod ui;

use resources::ascii::*;
use resources::ordering::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use ui::gui::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Mapping".to_string(),
                        height: HEIGHT,
                        width: HEIGHT * RESOLUTION,
                        present_mode: PresentMode::AutoVsync,
                        position: WindowPosition::Automatic,
                        decorations: true,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_startup_system(spawn_player)
        .add_system(quick_close)
        .add_startup_system(setup)
        .add_system(button_system)
        .add_plugin(CameraPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(DebugPlugin::default())
        .run();
}

fn quick_close(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    let alt = keyboard_input.any_pressed([KeyCode::LAlt, KeyCode::RAlt]);
    if (alt && keyboard_input.pressed(KeyCode::F4)) || keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit)
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite: TextureAtlasSprite {
                index: AsciiIndex::P.into(),
                color: Color::CYAN,
                ..default()
            },
            transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(0.0,0.0,LayerOrder::PlayerLayer.into())),
            ..default()
        },
        Name::new("Player"),
    ));
    for n in 0..10 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: ascii.0.clone(),
                sprite: TextureAtlasSprite {
                    index: AsciiIndex::HalfSquare.into(),
                    color: Color::GREEN,
                    ..default()
                },
                transform: Transform::from_scale(ASCII_SCALE).with_translation(Vec3::new(
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    LayerOrder::BackgroundLayer.into(),
                )),
                ..default()
            },
            Name::new(format!("Item{:?}", n)),
        ));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
