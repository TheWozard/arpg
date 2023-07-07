use crate::resources::palette;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct InteractiveBackgroundColor {
    pub default_color: Color,
    pub hovered_color: Color,
    pub clicked_color: Color,
}

impl Default for InteractiveBackgroundColor {
    fn default() -> Self {
        InteractiveBackgroundColor {
            default_color: palette::MENU_BUTTON_BACKGROUND,
            hovered_color: palette::MENU_BUTTON_HOVERED_BACKGROUND,
            clicked_color: palette::MENU_BUTTON_CLICKED_BACKGROUND,
        }
    }
}

pub fn interact_with_interactive_buttons(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &InteractiveBackgroundColor,
        ),
        Changed<Interaction>,
    >,
) {
    if let Ok((interaction, mut background_color, button)) = query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = button.clicked_color.into();
            }
            Interaction::Hovered => {
                *background_color = button.hovered_color.into();
            }
            Interaction::None => {
                *background_color = button.default_color.into();
            }
        }
    }
}

// InteractiveTextButton is a temporary structure to build button entities.
// This probably isn't the rust way to do this as the struct is all owned info
// and we are only trying to create optional params with defaults to the function.
pub struct InteractiveTextButton {
    pub text: String,
    pub text_style: TextStyle,
    pub color: InteractiveBackgroundColor,
    pub size: Size,
}

impl Default for InteractiveTextButton {
    fn default() -> Self {
        InteractiveTextButton {
            text: "Default".to_string(),
            text_style: TextStyle {
                font_size: 40.0,
                color: palette::MENU_TEXT_COLOR,
                ..default()
            },
            color: InteractiveBackgroundColor::default(),
            size: Size {
                width: Val::Px(200.),
                height: Val::Px(100.),
            },
        }
    }
}

impl InteractiveTextButton {
    pub fn initialize(&self, commands: &mut ChildBuilder, bundle: impl Bundle) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        size: self.size,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: self.color.default_color.to_owned().into(),
                    ..default()
                },
                self.color.to_owned(),
                bundle,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    self.text.to_owned(),
                    self.text_style.to_owned(),
                ));
            })
            .id()
    }
}
