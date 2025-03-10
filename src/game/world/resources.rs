use crate::config::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldData {
    pub asteroid_per_second: usize,
    pub asteroid_speed: f32,
}

impl Default for WorldData {
    fn default() -> WorldData {
        WorldData {
            asteroid_per_second: 1,
            asteroid_speed: ASTEROID_SPEED,
        }
    }
}

#[derive(Resource)]
pub struct WorldAsteroidSpawnTimer {
    pub timer: Timer,
    pub increase_timer: Timer,
}

impl Default for WorldAsteroidSpawnTimer {
    fn default() -> WorldAsteroidSpawnTimer {
        WorldAsteroidSpawnTimer {
            timer: Timer::from_seconds(WORLD_ASTEROID_SPAWNTIME, TimerMode::Repeating),
            increase_timer: Timer::from_seconds(
                WORLD_ASTEROID_INCREASE_SPAWNTIME,
                TimerMode::Repeating,
            ),
        }
    }
}
