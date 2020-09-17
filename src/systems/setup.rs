use bevy::prelude::*;

use crate::components::{
    markers::{Player, PrimaryCamera},
    RigidBody,
};

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    commands
        // Add cameras
        .spawn(Camera2dComponents::default())
        .with(PrimaryCamera)
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
        .with(Player)
        .with(RigidBody {
            mass: 1.0,
            drag_coefficient: 5.0,
            max_velocity: 500.0,
            min_velocity: 1.0,
            ..Default::default()
        })
        // This doesn't work yet
        // .spawn(PlayerBundle::new(&mut materials));
        ;
}
