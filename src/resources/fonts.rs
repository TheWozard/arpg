use bevy::prelude::*;

pub struct Fonts {
    pub mono: Handle<Font>,
}
impl Fonts {
    pub fn load(assets: &Res<AssetServer>) -> Fonts {
        Fonts {
            mono: assets.load("fonts/FiraMono-Medium.ttf"),
        }
    }
}
