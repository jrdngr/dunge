use bevy::{
    prelude::*,
    render::camera::{CameraProjection, OrthographicProjection},
    window::CursorMoved,
};

use crate::components::markers::PrimaryCamera;

#[derive(Default)]
pub struct InputState {
    pub x_axis: f32,
    pub y_axis: f32,
    pub left_mouse: bool,
    pub dash: bool,
    pub mouse_position: Vec2,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

impl InputState {
    pub fn update_mouse_position(
        &mut self,
        cursor_moved_events: &Res<Events<CursorMoved>>,
        window: &Res<WindowDescriptor>,
        mut camera_query: Query<(&OrthographicProjection, &Transform, &PrimaryCamera)>,
    ) {
        let event = self
            .cursor_moved_event_reader
            .iter(&cursor_moved_events)
            .next_back();
        if let Some(event) = event {
            let width = window.width as f32;
            let height = window.height as f32;

            let x = event.position.x() / width;
            let y = event.position.y() / height;

            let x = (2.0 * x) - 1.0;
            let y = (2.0 * y) - 1.0;

            let camera_iter = &mut camera_query.iter();

            if let Some((projection, transform, _)) = camera_iter.into_iter().next() {
                let transform = projection.get_projection_matrix() * transform.value;
                let position = Vec4::new(x, y, 0.0, 1.0);
                let world_position = transform.inverse() * position;

                self.mouse_position = Vec2::new(world_position.x(), world_position.y());
            }
        }
    }
}
