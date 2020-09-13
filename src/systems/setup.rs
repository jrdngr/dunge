use bevy::prelude::*;

use crate::components::{RigidBody, Player};

use crate::{
    WINDOW_SIZE,
};

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // Add player
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.2, 1.0).into()),
            translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(50.0, 50.0),
            },
            ..Default::default()
        })
        .with(Player::default())
        .with(RigidBody {
            mass: 1.0,
            drag_coefficient: 0.7,
            max_velocity: 500.0,
            ..Default::default()
        });

    // Add walls
    let wall_material = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    let wall_thickness = 5.0;
    let bounds = Vec2::new(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32);

    commands
        // left
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        // right
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        // bottom
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        // top
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        });
}
