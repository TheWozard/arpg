use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    pub mono: Handle<Font>,
}

pub fn load_fonts(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(Fonts {
        mono: assets.load("fonts/FiraMono-Medium.ttf"),
    });
}
