use super::components::Asteroid;
use super::resources::AsteroidSpawnTimer;
use crate::config::*;
use crate::events::GameOver;
use crate::game::player::components::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

fn spawn_asteroid(commands: &mut Commands, window: &Window, asset_server: &AssetServer) {
    let mut rng = rand::thread_rng();
    commands.spawn((
        Transform {
            translation: Vec3::new(rng.gen_range(-1.0..=1.0) * window.width(), window.height(), 0.0),
            ..default()
        },
        Sprite {
            image: asset_server.load(ASTEROID_SPRITE),
            ..default()
        },
        Asteroid {
            direction: Vec3::new(0.0, -1.0, 0.0),
            ..default()
        },
    ));
}

pub fn asteroid_startup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_asteroid(
        &mut commands,
        window_query.get_single().unwrap(),
        &asset_server,
    );
}

pub fn asteroid_spawn_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    asteroid_spawn_timer: Res<AsteroidSpawnTimer>,
) {
    if asteroid_spawn_timer.timer.finished() {
        spawn_asteroid(
            &mut commands,
            window_query.get_single().unwrap(),
            &asset_server,
        );
    }
}

pub fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    for (mut transform, asteroid) in asteroid_query.iter_mut() {
        let direction = Vec3::new(asteroid.direction.x, asteroid.direction.y, 0.0);
        transform.translation += direction * ASTEROID_SPEED * time.delta_secs();
    }
}

pub fn asteroid_collision(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asteroid_query: Query<&Transform, With<Asteroid>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for asteroid_transform in asteroid_query.iter() {
            let distance = player_transform
                .translation
                .distance(asteroid_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let asteroid_radius = ASTEROID_SIZE / 2.0;

            if distance < player_radius + asteroid_radius {
                println!("Player hit !");
                commands.spawn((
                    AudioPlayer::new(asset_server.load(SFX_GAMEOVER)),
                    PlaybackSettings::ONCE,
                ));
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: 0 });
            }
        }
    }
}

pub fn asteroid_lifetime(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    camera_query: Query<&OrthographicProjection, With<Camera2d>>,
) {
    let camera = camera_query.single();

    for (asteroid_entity, asteroid_transform) in asteroid_query.iter() {
        if asteroid_transform.translation.y < camera.area.height() / 2.0 * -1.0 {
            commands.entity(asteroid_entity).despawn();
            println!("Asteroid {:?} despawned!", asteroid_entity);
        }
    }
}

pub fn asteroid_tick_spawn_timer(
    mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());
}
