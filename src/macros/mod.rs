mod menu;

#[macro_export]
macro_rules! Cleanup {
    ( $name:tt ) => {
        #[derive(Component, Default)]
        pub struct $name {}

        impl $name {
            pub fn cleanup(mut commands: Commands, q: Query<Entity, With<$name>>) {
                for e in q.iter() {
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    };
}
