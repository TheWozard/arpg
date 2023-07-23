use bevy::{prelude::*, window::*};
use bevy_inspector_egui::prelude::*;

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct CameraSettings {
    pub move_camera_forward: KeyCode,
    pub move_camera_left: KeyCode,
    pub move_camera_backward: KeyCode,
    pub move_camera_right: KeyCode,
}
impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            move_camera_forward: KeyCode::W,
            move_camera_left: KeyCode::A,
            move_camera_backward: KeyCode::S,
            move_camera_right: KeyCode::D,
        }
    }
}

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct WorldCursor {
    pub position: Vec2,
}
impl WorldCursor {
    fn track(
        mut cursor: ResMut<WorldCursor>,
        primary_query: Query<&Window, With<PrimaryWindow>>,
        query: Query<(&Camera, &GlobalTransform), With<CameraControl>>,
    ) {
        let (camera, camera_transform) = query.single();
        let primary = primary_query.single();
        if let Some(screen_pos) = primary.cursor_position() {
            cursor.position = camera
                .viewport_to_world_2d(camera_transform, screen_pos)
                .unwrap_or_default();
        }
    }
}

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct CameraControl {
    pub movement_speed: f32,
}
impl Default for CameraControl {
    fn default() -> Self {
        CameraControl {
            movement_speed: 500.,
        }
    }
}
impl CameraControl {
    // System for handling camera movement based on the configured key codes
    fn movement(
        time: Res<Time>,
        keyboard_input: Res<Input<KeyCode>>,
        camera_settings: Res<CameraSettings>,
        mut query: Query<(&mut Transform, &CameraControl)>,
    ) {
        for (mut transform, controller) in &mut query {
            let mut direction = Vec3::new(0., 0., 0.);
            if keyboard_input.pressed(camera_settings.move_camera_forward) {
                direction += Vec3::Y
            }
            if keyboard_input.pressed(camera_settings.move_camera_backward) {
                direction += Vec3::NEG_Y
            }
            if keyboard_input.pressed(camera_settings.move_camera_right) {
                direction += Vec3::X
            }
            if keyboard_input.pressed(camera_settings.move_camera_left) {
                direction += Vec3::NEG_X
            }
            transform.translation +=
                direction.normalize_or_zero() * controller.movement_speed * time.delta_seconds()
        }
    }
}

// Spawns a camera with a CameraController
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraControl {
            movement_speed: 500.,
        },
        Name::new("Camera"),
    ));
}

// Plugin grouping all basic camera functionality
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<CameraSettings>();
        app.register_type::<WorldCursor>();
        app.register_type::<CameraControl>();

        app.insert_resource(CameraSettings::default());
        app.insert_resource(WorldCursor::default());

        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (CameraControl::movement, WorldCursor::track));
    }
}
