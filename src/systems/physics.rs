use bevy::prelude::*;

use crate::components::RigidBody;

pub fn physics(time: Res<Time>, mut query: Query<(&mut RigidBody, &mut Translation)>) {
    for (mut rigidbody, mut translation) in &mut query.iter() {
        rigidbody.update(time.delta_seconds);

        *translation.0.x_mut() += time.delta_seconds * rigidbody.velocity.x();
        *translation.0.y_mut() += time.delta_seconds * rigidbody.velocity.y();
    }
}
