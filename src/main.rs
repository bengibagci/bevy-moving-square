mod movable_square;

use bevy::prelude::*;
use crate::movable_square::MovableSquare;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d::default());

    // Movable square
    commands
        .spawn(Sprite {
            color: Color::srgb(0.3, 0.7, 0.3),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))  // Square starting position
        .insert(MovableSquare);
}

fn movement_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<MovableSquare>>, time: Res<Time>) {
    let speed = 200.0;
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    // The amount of movement is scaled by frame time
    let movement = direction.normalize_or_zero() * speed * time.delta_secs();

    // All frames are accessed to apply the motion
    for mut transform in query.iter_mut() {
        transform.translation += movement;
    }
}