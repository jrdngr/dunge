use bevy::{
    prelude::*,
};

use crate::{
    resources::InputState,
    components::{Player, RigidBody},
};

const PLAYER_MOVE_FORCE: f32 = 5_000.0;
const DASH_FORCE: f32 = 100_000.0;

pub fn player(
    mut input_state: ResMut<InputState>,
    mut query: Query<(&Player, &mut RigidBody, &Translation)>,
) {
    for (_, mut rigidbody, translation) in &mut query.iter() {
        if input_state.dash {
            input_state.dash = false;

            let player_position = Vec2::new(translation.0.x(), translation.0.y());
            let dash_direction = (input_state.mouse_position - player_position).normalize();

            let dash_force = dash_direction * DASH_FORCE;
            rigidbody.apply_force(dash_force);
        }

        if !rigidbody.is_over_max_velocity() {
            let x_force = input_state.x_axis * PLAYER_MOVE_FORCE;
            let y_force = input_state.y_axis * PLAYER_MOVE_FORCE;
            let force = Vec2::new(x_force, y_force);

            rigidbody.apply_force(force);    
        }
    }
}
