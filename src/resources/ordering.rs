pub enum LayerOrder {
    BackgroundLayer,
    PlayerLayer,
}

impl Into<f32> for LayerOrder {
    fn into(self: Self) -> f32 {
        (self as i16).into()
    }
}