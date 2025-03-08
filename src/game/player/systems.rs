use super::components::Player;
use crate::config::*;
use crate::game::laser::systems::spawn_laser;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Transform {
            translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Sprite {
            image: asset_server.load(PLAYER_SPRITE),
            ..default()
        },
        Player { ..default() },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        for key in keyboard_input.get_pressed() {
            match key {
                KeyCode::KeyA => direction.x -= 1.0,
                KeyCode::KeyD => direction.x += 1.0,
                KeyCode::KeyW => direction.y += 1.0,
                KeyCode::KeyS => direction.y -= 1.0,
                _ => {}
            }
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn player_movement_bound(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;

        let min_vec = Vec3::new(half_player_size, half_player_size, 0.0);
        let max_vec = Vec3::new(
            window.width() - half_player_size,
            window.height() - half_player_size,
            0.0,
        );

        player_transform.translation = player_transform.translation.clamp(min_vec, max_vec);
    }
}

pub fn player_shooting(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(transform) = player_query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            spawn_laser(&mut commands, transform, &asset_server);
        }
    }
}