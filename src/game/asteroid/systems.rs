use super::components::Asteroid;
use crate::config::*;
use crate::events::GameOver;
use crate::game::player::components::Player;
use crate::game::world::resources::WorldData;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub fn spawn_asteroid(commands: &mut Commands, window: &Window, asset_server: &AssetServer) {
    let mut rng = rand::thread_rng();
    commands.spawn((
        Transform {
            translation: Vec3::new(
                rng.gen_range(-0.9..=0.9) * window.width() * 0.9,
                window.height(),
                LAYER_WORLD,
            ),
            rotation: Quat::from_rotation_z(rng.gen_range(0.0..=10.0)),
            ..default()
        },
        Sprite {
            image: asset_server.load(ASTEROID_SPRITE),
            ..default()
        },
        Asteroid {
            direction: Vec3::new(0.0, -1.0, LAYER_WORLD),
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

pub fn asteroid_movement(
    mut asteroid_query: Query<(&mut Transform, &Asteroid)>,
    time: Res<Time>,
    world_data: Res<WorldData>,
) {
    for (mut transform, asteroid) in asteroid_query.iter_mut() {
        let direction = Vec3::new(asteroid.direction.x, asteroid.direction.y, asteroid.direction.z);
        transform.translation += direction * world_data.asteroid_speed * time.delta_secs();
    }
}

pub fn asteroid_collision(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asteroid_query: Query<&Transform, With<Asteroid>>,
    asset_server: Res<AssetServer>,
) {
    let Ok((player_entity, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    for asteroid_transform in asteroid_query.iter() {
        let distance = player_transform
            .translation
            .distance(asteroid_transform.translation);
        let player_radius = PLAYER_SIZE / 2.0;
        let asteroid_radius = ASTEROID_SIZE / 2.0;

        if distance < player_radius + asteroid_radius {
            info!("Player hit !");
            commands.spawn((
                AudioPlayer::new(asset_server.load(SFX_GAMEOVER)),
                PlaybackSettings::ONCE,
            ));
            commands.entity(player_entity).despawn();
            game_over_event_writer.send(GameOver { score: 0 });
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
            debug!("Asteroid {:?} despawned!", asteroid_entity);
        }
    }
}
