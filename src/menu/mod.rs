use crate::resources::ui::button::*;
use crate::resources::*;
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(crate::resources::ui::UiPlugin)
            .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

#[derive(Resource)]
struct MenuData {
    root: Entity,
}

fn setup_menu(mut commands: Commands, font: Res<fonts::Fonts>) {
    let root = build_menu(&mut commands, font.as_ref());
    commands.insert_resource(MenuData { root });
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.root).despawn_recursive();
}

fn build_menu(commands: &mut Commands, font: &fonts::Fonts) -> Entity {
    let play_button = InteractiveTextButton {
        text: "Play".to_string(),
        font: font.mono.clone(),
        ..default()
    }
    .build(commands);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: palette::MENU_BACKGROUND.into(),
            ..default()
        })
        .add_child(play_button)
        .id()
}
