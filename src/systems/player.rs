use bevy::{
    prelude::*,
};

use crate::{
    resources::InputState,
    components::{Player, Velocity},
};

const PLAYER_VELOCITY: f32 = 500.0;

pub fn player(
    input_state: Res<InputState>,
    mut query: Query<(&Player, &mut Velocity)>,
) {
    for (_, mut velocity) in &mut query.iter() {
        *velocity.0.x_mut() = input_state.x_axis * PLAYER_VELOCITY;
        *velocity.0.y_mut() = input_state.y_axis * PLAYER_VELOCITY;
    }
}
