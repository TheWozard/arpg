use crate::resources::ui::button::*;
use crate::resources::*;
use crate::AppState;
use crate::Cleanup;
use crate::Clickable;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum TownState {
    #[default]
    Closed,
    Base,
    Tree,
}

// We use macros to build common components for running the ui.
Cleanup!(BaseCleanupHint);
Cleanup!(TreeCleanupHint);
Clickable!(GameHint(state: ResMut<NextState<AppState>>) => state.set(AppState::Game));
Clickable!(ToTreeHint(state: ResMut<NextState<TownState>>) => state.set(TownState::Tree));
Clickable!(ToBaseHint(state: ResMut<NextState<TownState>>) => state.set(TownState::Base));
Clickable!(ToMenuHint(state: ResMut<NextState<AppState>>) => state.set(AppState::Menu));

// Mounts the town systems
pub struct TownPlugin;
impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TownState>();

        // High level state management
        app.add_systems(OnEnter(AppState::Town), on_enter);
        app.add_systems(OnExit(AppState::Town), on_exit);

        // Base
        app.add_systems(OnEnter(TownState::Base), setup_menu);
        app.add_systems(
            Update,
            (GameHint::click, ToTreeHint::click, ToMenuHint::click)
                .run_if(in_state(TownState::Base)),
        );
        app.add_systems(OnExit(TownState::Base), BaseCleanupHint::cleanup);

        // Tree
        app.add_systems(OnEnter(TownState::Tree), setup_tree_ui);
        app.add_systems(Update, ToBaseHint::click.run_if(in_state(TownState::Tree)));
        app.add_systems(OnExit(TownState::Tree), TreeCleanupHint::cleanup);
    }
}

fn on_enter(mut state: ResMut<NextState<TownState>>) {
    state.set(TownState::Base);
}

fn on_exit(mut state: ResMut<NextState<TownState>>) {
    state.set(TownState::Closed);
}

fn setup_menu(mut commands: Commands, font: Res<fonts::Fonts>) {
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
                background_color: palette::TOWN_BACKGROUND.into(),
                ..default()
            },
            BaseCleanupHint {},
        ))
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Launch".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 40.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                width: Val::Px(300.),
                height: Val::Px(110.),
                ..default()
            }
            .initialize(parent, GameHint {});
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Tree".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 40.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                width: Val::Px(200.),
                height: Val::Px(100.),
                ..default()
            }
            .initialize(parent, ToTreeHint {});
        })
        .with_children(|parent| {
            InteractiveTextButton {
                text: "Menu".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 40.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                width: Val::Px(200.),
                height: Val::Px(100.),
                ..default()
            }
            .initialize(parent, ToMenuHint {});
        });
}

fn setup_tree_ui(mut commands: Commands, font: Res<fonts::Fonts>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            },
            TreeCleanupHint {},
        ))
        .with_children(|parent| {
            InteractiveTextButton {
                text: "X".to_string(),
                text_style: TextStyle {
                    font: font.mono.clone(),
                    font_size: 30.0,
                    color: palette::MENU_TEXT_COLOR,
                },
                width: Val::Px(20.),
                height: Val::Px(20.),
                ..default()
            }
            .initialize(parent, ToBaseHint {});
        });
}
