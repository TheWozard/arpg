use crate::resources::*;
use crate::ui::button::*;
use crate::AppState;
use crate::Cleanup;
use crate::Clickable;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// We use macros to build common components for running the ui.
Cleanup!(MenuHint);
Clickable!(PlayHint(state: ResMut<NextState<AppState>>) => state.set(AppState::Town));
Clickable!(ExitHint(exit: EventWriter<AppExit>) => exit.send(AppExit));

// Mounts the menu systems
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu);
        app.add_systems(
            Update,
            (PlayHint::click, ExitHint::click).run_if(in_state(AppState::Menu)),
        );
        app.add_systems(OnExit(AppState::Menu), MenuHint::cleanup);
    }
}

fn setup_menu(mut commands: Commands, res: Res<Resources>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.),
                    ..default()
                },
                background_color: palette::menu::main::BACKGROUND.into(),
                ..default()
            },
            MenuHint {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ARPG".to_string(),
                TextStyle {
                    font: res.fonts.mono.to_owned(),
                    font_size: 60.0,
                    color: palette::menu::TEXT_COLOR,
                },
            ));
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Play".to_string(),
                text_style: TextStyle {
                    font: res.fonts.mono.clone(),
                    font_size: 40.0,
                    color: palette::menu::TEXT_COLOR,
                },
                width: Val::Px(300.),
                height: Val::Px(110.),
                ..default()
            }
            .initialize(parent, PlayHint {});
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Options".to_string(),
                text_style: TextStyle {
                    font: res.fonts.mono.clone(),
                    font_size: 40.0,
                    color: palette::menu::TEXT_COLOR,
                },
                width: Val::Px(200.),
                height: Val::Px(100.),
                ..default()
            }
            .initialize(parent, ());
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Exit".to_string(),
                text_style: TextStyle {
                    font: res.fonts.mono.clone(),
                    font_size: 40.0,
                    color: palette::menu::TEXT_COLOR,
                },
                width: Val::Px(200.),
                height: Val::Px(100.),
                ..default()
            }
            .initialize(parent, ExitHint {});
        });
}
