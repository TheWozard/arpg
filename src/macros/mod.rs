mod menu;

#[macro_export]
macro_rules! Cleanup {
    ( $name:tt ) => {
        #[derive(Component)]
        struct $name {}

        impl $name {
            fn cleanup(mut commands: Commands, q: Query<Entity, With<$name>>) {
                for e in q.iter() {
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    };
}
