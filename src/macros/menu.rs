#[macro_export]
macro_rules! Clickable {
    ( $name:tt($($v:tt: $t:ty),+) => $clicked:expr ) => {
        #[derive(Component)]
        struct $name {}

        impl $name {
            fn click(
                $(mut $v: $t),+,
                query: Query<&Interaction, (Changed<Interaction>, With<$name>)>,
            ) {
                if let Ok(interaction) = query.get_single() {
                    match *interaction {
                        Interaction::Clicked => $clicked,
                        _ => return,
                    }
                }
            }
        }
    };
}
