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
                        Interaction::Pressed => $clicked,
                        _ => return,
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! StateBasedPlugin {
    ( $name:tt ) => {
        pub struct $name<S: States> {
            s: S,
        }

        impl<S: States> $name<S> {
            pub fn new(state: S) -> Self {
                Self { s: state }
            }

            fn state(&self) -> S {
                self.s.clone()
            }
        }
    };
}
