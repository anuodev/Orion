mod components;
mod systems;

use systems::*;

use crate::app::AppState;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), hud_spawn);
        app.add_systems(Update, hud_update.run_if(in_state(AppState::Game)));
        app.add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
