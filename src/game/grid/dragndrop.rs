use crate::camera::WorldCursor;
use bevy::prelude::*;

pub struct DragNDrop;
impl Plugin for DragNDrop {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (dragndrop_activation, dragndrop_movement))
            .register_type::<Draggable>();
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Draggable {
    active: bool,
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
