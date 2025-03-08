pub mod components;
mod systems;

use bevy::prelude::*;
use super::player::systems::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
        app.add_systems(Update, player_movement_bound.after(player_movement));
        app.add_systems(Update, player_shooting.after(player_movement_bound));
    }
}