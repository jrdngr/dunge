use bevy::prelude::Vec2;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RigidBody {
    pub mass: f32,
    pub drag_coefficient: f32,
    pub max_velocity: f32,
    pub min_velocity: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl RigidBody {
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn update(&mut self, delta_time: f32) {
        let drag = self.drag_coefficient * self.velocity;
        self.acceleration -= drag;

        let acceleration = delta_time * self.acceleration;
        self.velocity += acceleration;

        if self.is_under_min_velocity() {
            self.velocity = Vec2::new(0.0, 0.0);
        }

        self.acceleration = Vec2::new(0.0, 0.0);
    }

    pub fn is_over_max_velocity(&self) -> bool {
        self.velocity.length_squared() > self.max_velocity * self.max_velocity
    }

    pub fn is_under_min_velocity(&self) -> bool {
        self.velocity.length_squared() < self.min_velocity * self.min_velocity
    }
}
