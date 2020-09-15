use bevy::prelude::*;

use crate::components::{
    RigidBody, 
    markers::{
        Player,
        PrimaryCamera,
    },
};

use crate::{
    WINDOW_SIZE,
};

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
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
        // Add debug text
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                value: "Physics".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.2, 0.2, 0.8),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        ;

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
