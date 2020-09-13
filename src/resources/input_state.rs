use bevy::{
    prelude::*,
    window::CursorMoved,
};

use crate::components::Position;

#[derive(Default)]
pub struct InputState {
    pub x_axis: f32,
    pub y_axis: f32,
    pub left_mouse: bool,
    pub right_mouse: bool,
    pub mouse_position: Position,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

impl InputState {
    pub fn update_mouse_position(&mut self,
        cursor_moved_events: &Res<Events<CursorMoved>>,
        window: &Res<WindowDescriptor>,
    ) {
        let event = self.cursor_moved_event_reader.iter(&cursor_moved_events).next_back();
        if let Some(event) = event {
            let width = window.width as f32;
            let height = window.height as f32;

            let x = event.position.x() / width;
            let y = event.position.y() / height;

            let x = (2.0 * x) - 1.0;
            let y = (2.0 * y) - 1.0;

            self.mouse_position = Position { x, y };
        }
    }
}
