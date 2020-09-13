use bevy::prelude::Vec2;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RigidBody {
    pub mass: f32,
    pub drag_coefficient: f32,
    pub max_velocity: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub sum_of_forces: Vec2,
}

impl RigidBody {
    pub fn apply_force(&mut self, force: Vec2) {
        self.sum_of_forces += force;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.acceleration += self.sum_of_forces;
        self.sum_of_forces = Vec2::new(0.0, 0.0);

        let acceleration = delta_time * self.acceleration;
        self.velocity += acceleration;

        if self.velocity.length_squared() > self.max_velocity * self.max_velocity {
            self.velocity = self.velocity.normalize() * self.max_velocity;
        }

        let drag = self.drag_coefficient * self.velocity;
        self.velocity -= drag;
    }
}
