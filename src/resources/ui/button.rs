use crate::resources::palette;
use bevy::prelude::*;

#[derive(Component)]
pub struct InteractiveButton {
    pub default_color: Color,
    pub hovered_color: Color,
    pub clicked_color: Color,
}

pub fn interact_with_interactive_buttons(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &InteractiveButton),
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
    pub text_color: Color,
    pub color: Color,
    pub font: Handle<Font>,
    pub size: Size,
}

impl Default for InteractiveTextButton {
    fn default() -> Self {
        InteractiveTextButton {
            text: "Placeholder".to_string(),
            text_color: palette::MENU_TEXT_COLOR,
            color: palette::MENU_BUTTON_BACKGROUND,
            font: Handle::default(),
            size: Size {
                width: Val::Px(200.),
                height: Val::Px(100.),
            },
        }
    }
}

impl InteractiveTextButton {
    pub fn build(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        size: self.size,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: self.color.to_owned().into(),
                    ..default()
                },
                InteractiveButton {
                    default_color: self.color.to_owned().into(),
                    hovered_color: Color::RED.into(),
                    clicked_color: Color::GREEN.into(),
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    self.text.to_owned(),
                    TextStyle {
                        font_size: 40.0,
                        font: self.font.to_owned(),
                        color: self.text_color.to_owned(),
                        ..default()
                    },
                ));
            })
            .id()
    }
}
