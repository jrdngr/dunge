use bevy::{
    prelude::*,
    window::CursorMoved,
};

use crate::{
    resources::InputState,
};

pub fn update_input_state(
    mut input_state: ResMut<InputState>,
    window: Res<WindowDescriptor>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    input_state.update_mouse_position(&cursor_moved_events, &window);
    
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        input_state.x_axis = -1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        input_state.x_axis = 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        input_state.x_axis = -1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        input_state.y_axis = 1.0;
    }

    input_state.left_mouse = mouse_input.pressed(MouseButton::Left);
    input_state.right_mouse = mouse_input.pressed(MouseButton::Right);
}
