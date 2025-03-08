pub mod components;
mod systems;

use bevy::prelude::*;
use super::laser::systems::*;

pub struct LaserPlugin;
impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, laser_movement);
        app.add_systems(Update, laser_collision);
        app.add_systems(Update, laser_lifetime);
    }
}