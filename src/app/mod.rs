mod components;
pub mod resources;
mod systems;

use super::app::systems::*;
use crate::events::GameOver;
use bevy::prelude::*;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.add_event::<GameOver>();
        //app.add_systems(Startup, spawn_perfui);
        app.add_systems(Startup, load_icon);
        app.add_systems(Startup, set_background);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, set_window_icon.after(load_icon));
        app.add_systems(Update, update_background_on_resize);
        app.add_systems(Update, exit_game);
        app.add_systems(Update, handle_game_over);
        app.add_systems(Update, display_score);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Game,
}
