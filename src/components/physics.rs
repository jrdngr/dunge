use bevy::prelude::Vec2;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Acceleration(pub Vec2);
