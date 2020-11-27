use bevy::prelude::*;

pub struct CameraDraggingState {
    cursor_event_reader: EventReader<CursorMoved>,
    //camera_entity: Entity, //Tag for the camera in case we use multiple cameras
    prev_cursor_pos: Option<Vec2>,
}

impl CameraDraggingState {
    pub fn new(camera_entity: Entity) -> Self {
        CameraDraggingState {
            cursor_event_reader: Default::default(),
            //camera_entity,
            prev_cursor_pos: None,
        }
    }
}

pub fn camera_dragging_system(
    mut state: ResMut<CameraDraggingState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_button_input: Res<Input<MouseButton>>,
    //windows: Res<Windows>,
    mut query: Query<&mut Transform>,
) {
    for mut transform in query.iter_mut() {
        if mouse_button_input.pressed(MouseButton::Right) {
            for event in state.cursor_event_reader.iter(&cursor_moved_events) {
                //let cursor_pos = screen_to_world(event.position, &transform, &windows);
                // let prev_cursor_pos = screen_to_world(
                //     state.prev_cursor_pos.unwrap_or(event.position),
                //     &transform,
                //     &windows,
                // );
                let cursor_pos_delta = event.position - state.prev_cursor_pos.unwrap_or(event.position);
                transform.translation -= cursor_pos_delta.extend(0.0);
/*              println!("Cursor position delta {:?}", cursor_pos_delta);
                println!("Previous Cursor position {:?}", cursor_pos_delta);
                println!("Event position {:?}", event.position); */
                //Translation(camera_translation.0 - cursor_pos_delta.extend(0.0));
                state.prev_cursor_pos = Some(event.position);
            }
        } else {
            state.prev_cursor_pos = None;
        }
    }
}

/*
    Needed later for handling the Zoomed coordinates
*/
pub fn screen_to_world(p: Vec2, camera_transform: &Transform, windows: &Windows) -> Vec2 {
    let w = windows.get_primary().unwrap();
    let resolution = Vec2::new(w.width() as f32, w.height() as f32);
    let p_ndc = p - resolution / 2.0;
    let p_world = camera_transform.translation * p_ndc.extend(1.0);

    p_world.truncate()
}
