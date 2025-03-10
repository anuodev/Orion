use super::resources::{WorldAsteroidSpawnTimer, WorldData};
use crate::config::*;
use crate::game::asteroid::systems::spawn_asteroid;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn world_asteroid_tick_spawn_timer(
    mut asteroid_spawn_timer: ResMut<WorldAsteroidSpawnTimer>,
    time: Res<Time>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());
    asteroid_spawn_timer.increase_timer.tick(time.delta());
}

pub fn world_asteroid_spawn_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    asteroid_spawn_timer: Res<WorldAsteroidSpawnTimer>,
    world_data: Res<WorldData>,
) {
    if asteroid_spawn_timer.timer.finished() {
        for _ in 0..world_data.asteroid_per_second {
            spawn_asteroid(
                &mut commands,
                window_query.get_single().unwrap(),
                &asset_server,
            );
        }
    }
}

pub fn world_asteroid_increase_spawn(
    asteroid_spawn_timer: Res<WorldAsteroidSpawnTimer>,
    mut world_data: ResMut<WorldData>,
) {
    if asteroid_spawn_timer.increase_timer.finished() {
        world_data.asteroid_per_second += WORLD_ASTEROID_SPAWN_INCREASE_STEP;
        world_data.asteroid_speed += WORLD_ASTEROID_SPEED_INCREASE_STEP;
        info!(
            "Asteroid spawn rate increased: {}/s at {}speed",
            world_data.asteroid_per_second, world_data.asteroid_speed
        );
    }
}
