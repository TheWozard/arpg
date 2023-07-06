use crate::resources::ui;
use crate::resources::ui::button::*;
use crate::resources::*;
use crate::AppState;
use crate::Clickable;
use bevy::app::AppExit;
use bevy::prelude::*;

// Mounts the menu systems
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(crate::resources::ui::UiPlugin)
            .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(PlayHint::click.run_if(in_state(AppState::Menu)))
            .add_system(ExitHint::click.run_if(in_state(AppState::Menu)))
            .add_system(ui::cleanup_system::<MenuHint>.in_schedule(OnExit(AppState::Menu)));
    }
}

// MenuHint defines a menu item to be cleaned_up on exit.
#[derive(Component)]
struct MenuHint {}

Clickable!(PlayHint(state: ResMut<NextState<AppState>>) => state.set(AppState::InGame));
Clickable!(ExitHint(exit: EventWriter<AppExit>) => exit.send(AppExit));

fn setup_menu(mut commands: Commands, font: Res<fonts::Fonts>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    gap: Size {
                        width: Val::Px(20.),
                        height: Val::Px(20.),
                    },
                    ..default()
                },
                background_color: palette::MENU_BACKGROUND.into(),
                ..default()
            },
            MenuHint {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ARPG".to_string(),
                TextStyle {
                    font: font.mono.to_owned(),
                    font_size: 60.0,
                    color: palette::MENU_TEXT_COLOR,
                },
            ));
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Play".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 40.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                size: Size {
                    width: Val::Px(300.),
                    height: Val::Px(110.),
                },
                ..default()
            }
            .initialize(parent, PlayHint {});
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Options".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 40.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                size: Size {
                    width: Val::Px(200.),
                    height: Val::Px(100.),
                },
                ..default()
            }
            .initialize(parent, ());
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Exit".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 40.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                size: Size {
                    width: Val::Px(200.),
                    height: Val::Px(100.),
                },
                ..default()
            }
            .initialize(parent, ExitHint {});
        });
}
