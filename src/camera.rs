use bevy::{prelude::*, window::*};
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, InspectorOptions)]
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

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct WorldCursor {
    pub position: Vec2,
}

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct ControlledCamera {
    pub movement_speed: f32,
}

impl Default for ControlledCamera {
    fn default() -> Self {
        ControlledCamera {
            movement_speed: 500.,
        }
    }
}

// Spawns a camera with a CameraController
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        ControlledCamera {
            movement_speed: 500.,
        },
        Name::new("Camera"),
    ));
}

// System for handling camera movement based on the configured key codes
fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    camera_settings: Res<CameraSettings>,
    mut query: Query<(&mut Transform, &ControlledCamera)>,
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

fn world_cursor_tracker(
    mut cursor: ResMut<WorldCursor>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<(&Camera, &GlobalTransform), With<ControlledCamera>>,
) {
    let (camera, camera_transform) = query.single();
    let primary = primary_query.single();
    if let Some(screen_pos) = primary.cursor_position() {
        cursor.position = camera
            .viewport_to_world_2d(camera_transform, screen_pos)
            .unwrap_or_default();
    }
}

pub struct CameraPlugin;

// Plugin grouping all basic camera functionality
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraSettings::default());
        app.insert_resource(WorldCursor::default());

        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (camera_movement, world_cursor_tracker));
    }
}
