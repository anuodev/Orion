pub mod components;
mod systems;

use super::player::systems::*;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_spawn);
        app.add_systems(Update, player_movement);
        app.add_systems(Update, player_movement_bound.after(player_movement));
        app.add_systems(Update, player_shoot_laser.after(player_movement_bound));
    }
}
