use bevy::prelude::*;

use crate::{
    resources::InputState,
    components::{Player, RigidBody},
};

pub fn debug(
    input_state: Res<InputState>,
    mut text_query: Query<&mut Text>,
    mut query: Query<(&Player, &RigidBody)>,
) {
    for mut text in &mut text_query.iter() {
        for (_, rigidbody) in &mut query.iter() {
            text.value = format!(
                "m: ({:.3}, {:.3}) -- v: ({:.3}, {:.3}) -- a: ({:.3}, {:.3})",
                input_state.mouse_position.x(),
                input_state.mouse_position.y(),
                rigidbody.velocity.x(),
                rigidbody.velocity.y(),
                rigidbody.acceleration.x(),
                rigidbody.acceleration.y(),
            );
        }    
    }
}
