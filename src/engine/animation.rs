use bevy::prelude::*;

use super::player::*;

pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn animate_sprite(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    let keys = [
        KeyCode::W,
        KeyCode::Up,
        KeyCode::A,
        KeyCode::Left,
        KeyCode::S,
        KeyCode::Down,
        KeyCode::D,
        KeyCode::Right,
    ];
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if keyboard_input.any_pressed(keys) {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
                if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
                    sprite.flip_x = true;
                    sprite.flip_y = false;
                } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
                    sprite.flip_x = false;
                    sprite.flip_y = false
                }
            } else {
                sprite.index = indices.first;
            }
        }
    }
}
