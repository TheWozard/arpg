use crate::camera::WorldCursor;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Draggable {
    active: bool,
    // offset: Vec2,
}

pub fn dragndrop_activation(
    keyboard_input: Res<Input<MouseButton>>,
    world_location: Res<WorldCursor>,
    mut query: Query<(&GlobalTransform, &mut Draggable)>,
) {
    if keyboard_input.just_pressed(MouseButton::Left) {
        for (trans, mut dragable) in &mut query {
            let t = trans.translation().truncate();
            if t.distance(world_location.position) < 100. {
                dragable.active = true;
                // dragable.offset = t - world_location.position;
            }
        }
    }
    if keyboard_input.just_released(MouseButton::Left) {
        for (_trans, mut dragable) in &mut query {
            dragable.active = false
        }
    }
}

pub fn dragndrop_movement(
    mut cursor_evr: EventReader<CursorMoved>,
    world_location: Res<WorldCursor>,
    mut query: Query<(&mut Transform, &Draggable)>,
) {
    for _ev in cursor_evr.iter() {
        for (mut trans, dragable) in &mut query {
            if dragable.active == true {
                trans.translation = world_location.position.extend(trans.translation.z)
            }
        }
    }
}

pub struct DragNDrop;
impl Plugin for DragNDrop {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(dragndrop_activation)
            .add_system(dragndrop_movement)
            .register_type::<Draggable>();
    }
}
