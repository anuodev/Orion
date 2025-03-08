use crate::config::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> AsteroidSpawnTimer {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(WORLD_ASTEROID_SPAWNTIME, TimerMode::Repeating),
        }
    }
}
