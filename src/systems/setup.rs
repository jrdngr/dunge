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
    configure_cameras(&mut commands);
    configure_player(&mut commands, &mut materials);
    // _configure_debug_test(&mut commands, &_asset_server);
}

fn configure_cameras(commands: &mut Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .with(PrimaryCamera)
        .spawn(UiCameraComponents::default());
}

fn configure_player(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands
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
        });
}

fn _configure_debug_test(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(TextComponents {
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
    });
}
