pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, asteroid_startup);
        app.add_systems(Update, asteroid_movement);
        app.add_systems(Update, asteroid_collision);
        app.add_systems(Update, asteroid_lifetime);
    }
}
