pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::AsteroidSpawnTimer;
use systems::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>();
        app.add_systems(Update, world_asteroid_tick_spawn_timer);
        app.add_systems(Update, world_asteroid_spawn_over_time);
    }
}
