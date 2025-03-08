pub mod components;
pub mod systems;

use super::laser::systems::*;
use bevy::prelude::*;

pub struct LaserPlugin;
impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, laser_movement);
        app.add_systems(Update, laser_collision);
        app.add_systems(Update, laser_lifetime);
    }
}