use bevy::prelude::*;
use crate::components::{markers::Player, RigidBody};

// This doesn't work yet

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: SpriteComponents,
    player: Player,
    ridigbody: RigidBody,
}

impl PlayerBundle {
    pub fn new(materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            player: Player,
            sprite: SpriteComponents {
                material: materials.add(Color::rgb(0.0, 0.2, 1.0).into()),
                translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(50.0, 50.0),
                },
                ..Default::default()
            },
            ridigbody: RigidBody {
                mass: 1.0,
                drag_coefficient: 5.0,
                max_velocity: 500.0,
                min_velocity: 1.0,
                ..Default::default()
            },
        }
    }
}
