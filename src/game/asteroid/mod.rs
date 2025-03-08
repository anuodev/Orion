pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use resources::AsteroidSpawnTimer;

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>();
        app.add_systems(Startup, asteroid_startup);
        app.add_systems(Update, asteroid_movement);
        app.add_systems(Update, asteroid_collision);
        app.add_systems(Update, asteroid_tick_spawn_timer);
        app.add_systems(Update, asteroid_spawn_over_time);
        app.add_systems(Update, asteroid_lifetime);
    }
}