mod menu;

#[macro_export]
macro_rules! Cleanup {
    ( $name:tt ) => {
        #[derive(Component, Reflect, Default, InspectorOptions)]
        #[reflect(Component, InspectorOptions)]
        pub struct $name;

        impl $name {
            pub fn cleanup(mut commands: Commands, q: Query<Entity, With<$name>>) {
                for e in q.iter() {
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! DebugOnly {
    ( $($tts:tt)* ) => {
        #[cfg(all(not(target_arch = "wasm32")))]
        {
            $($tts)*
        }
    };
}

#[macro_export]
macro_rules! DebugAttributes {
    ( $attr:tt, $($tts:tt)* ) => {
        #[derive($attr)]
        #[cfg_attr(
            all(not(target_arch = "wasm32")),
            derive(Reflect, Default, InspectorOptions)
        )]
        #[cfg_attr(all(not(target_arch = "wasm32")), reflect($attr, InspectorOptions))]
        $($tts)*
    };
}
