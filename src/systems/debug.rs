use bevy::prelude::*;

use crate::{
    resources::InputState,
    components::{Player, RigidBody},
};

pub fn debug(
    input_state: Res<InputState>,
    mut text_query: Query<&mut Text>,
    mut query: Query<(&Player, &RigidBody,  &Translation)>,
) {
    for mut text in &mut text_query.iter() {
        for (_, rigidbody, player_position) in &mut query.iter() {
            text.value = format!(
                "m: ({:.3}, {:.3}) -- p: ({:.3}, {:.3}) -- v: ({:.3}, {:.3}) -- a: ({:.3}, {:.3})",
                input_state.mouse_position.x(),
                input_state.mouse_position.y(),
                player_position.x(),
                player_position.y(),
                rigidbody.velocity.x(),
                rigidbody.velocity.y(),
                rigidbody.acceleration.x(),
                rigidbody.acceleration.y(),
            );
        }    
    }
}
