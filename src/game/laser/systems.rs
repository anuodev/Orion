use std::ops::Add;

use super::components::Laser;
use crate::config::*;
use crate::game::asteroid::components::Asteroid;
use crate::game::score::resources::Score;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_laser(
    commands: &mut Commands,
    player_transform: &Transform,
    asset_server: &AssetServer,
) {
    commands.spawn((
        Transform {
            translation: player_transform.translation,
            ..default()
        },
        Sprite {
            image: asset_server.load(LASER_SPRITE),
            ..default()
        },
        Laser {
            speed: LASER_SPEED,
            direction: Vec3::new(0.0, 1.0, 0.0),
            ..default()
        },
    ));
}

pub fn laser_movement(mut laser_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut transform, laser) in laser_query.iter_mut() {
        let direction = Vec3::new(laser.direction.x, laser.direction.y, 0.0);
        transform.translation += direction * laser.speed * time.delta_secs();
    }
}

pub fn laser_collision(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    for (laser_entity, laser_transform) in laser_query.iter() {
        for (asteroid_entity, asteroid_transform) in asteroid_query.iter() {
            let distance = laser_transform
                .translation
                .distance(asteroid_transform.translation);
            let laser_radius = LASER_SIZE / 2.0;
            let asteroid_radius = ASTEROID_SIZE / 2.0;

            if distance < laser_radius + asteroid_radius {
                debug!("Asteroid destroyed by laser !");
                commands.spawn((
                    AudioPlayer::new(asset_server.load(SFX_ASTEROID_DESTROYED)),
                    PlaybackSettings::ONCE,
                ));
                commands.entity(laser_entity).despawn();
                commands.entity(asteroid_entity).despawn();

                score.as_mut().value += 1;
            }
        }
    }
}

pub fn laser_lifetime(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
) {
    let window = window_query.get_single().unwrap();

    for (laser_entity, asteroid_transform) in laser_query.iter() {
        if asteroid_transform.translation.y > window.height().add(10.0) {
            commands.entity(laser_entity).despawn();
            debug!("Laser {:?} despawned!", laser_entity);
        }
    }
}
