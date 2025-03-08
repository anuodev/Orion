pub mod resources;

use bevy::prelude::*;
use resources::Score;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}