use bevy::{
    prelude::*,
};

use crate::{
    resources::InputState,
    components::{Player, RigidBody},
};

const PLAYER_MOVE_FORCE: f32 = 500.0;

pub fn player(
    input_state: Res<InputState>,
    mut query: Query<(&Player, &mut RigidBody)>,
) {
    for (_, mut rigidbody) in &mut query.iter() {
        let x_force = input_state.x_axis * PLAYER_MOVE_FORCE;
        let y_force = input_state.y_axis * PLAYER_MOVE_FORCE;
        let force = Vec2::new(x_force, y_force);

        rigidbody.apply_force(force);
    }
}
