use super::resources::AsteroidSpawnTimer;
use crate::game::asteroid::systems::spawn_asteroid;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn world_asteroid_spawn_over_time(
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

pub fn world_asteroid_tick_spawn_timer(
    mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());
}
