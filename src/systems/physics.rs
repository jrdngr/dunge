use bevy::{
    prelude::*,
};

use crate::{
    components::{Velocity, Acceleration},
};

pub fn physics(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Acceleration, &mut Translation)>,
) {
    for (mut velocity, acceleration, mut translation) in &mut query.iter() {
        velocity.0 += time.delta_seconds * acceleration.0;
        *translation.0.x_mut() += time.delta_seconds * velocity.0.x();
        *translation.0.y_mut() += time.delta_seconds * velocity.0.y();
    }
}
