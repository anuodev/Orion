pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldData>();
        app.init_resource::<WorldAsteroidSpawnTimer>();
        app.add_systems(Update, world_asteroid_tick_spawn_timer);
        app.add_systems(Update, world_asteroid_spawn_over_time);
        app.add_systems(
            Update,
            world_asteroid_increase_spawn.after(world_asteroid_spawn_over_time),
        );
    }
}
