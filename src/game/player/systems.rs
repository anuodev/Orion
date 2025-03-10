use super::components::Player;
use crate::config::*;
use crate::game::laser::systems::spawn_laser;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn player_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform {
            scale: Vec3::new(0.5, 0.5, 1.0), 
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
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };

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

pub fn player_movement_bound(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    let window = window_query.get_single().unwrap();
    let half_player_size = PLAYER_SIZE / 2.0;

    let min_vec = Vec3::new(
        window.width() / 2.0 * -1.0 + half_player_size,
        window.height() / 2.0 * -1.0 + half_player_size,
        0.0,
    );
    let max_vec = Vec3::new(
        window.width() / 2.0 - half_player_size,
        window.height() / 2.0 - half_player_size,
        0.0,
    );

    player_transform.translation = player_transform.translation.clamp(min_vec, max_vec);
}

pub fn player_shoot_laser(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(transform) = player_query.get_single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        spawn_laser(&mut commands, transform, &asset_server);
        commands.spawn((
            AudioPlayer::new(asset_server.load(SFX_PLAYER_SHOOT_LASER)),
            PlaybackSettings::ONCE,
        ));
    }
}
