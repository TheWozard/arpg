use crate::resources::ui::button::*;
use crate::resources::*;
use crate::AppState;
use crate::Cleanup;
use crate::Clickable;
use bevy::prelude::*;

// We use macros to build common components for running the ui.
Cleanup!(CleanupHint);
Clickable!(PlayHint(state: ResMut<NextState<AppState>>) => state.set(AppState::Game));
Clickable!(ExitHint(state: ResMut<NextState<AppState>>) => state.set(AppState::Menu));

// Mounts the menu systems
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(crate::resources::ui::UiPlugin)
            .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(PlayHint::click.run_if(in_state(AppState::Menu)))
            .add_system(ExitHint::click.run_if(in_state(AppState::Menu)))
            .add_system(CleanupHint::cleanup.in_schedule(OnExit(AppState::Menu)));
    }
}

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
            CleanupHint {},
        ))
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Launch".to_string(),
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
                text: "Menu".to_string(),
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
