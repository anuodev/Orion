pub mod asteroid;
pub mod laser;
pub mod player;
pub mod score;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugin;
impl PluginGroup for GamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::game::player::PlayerPlugin)
            .add(crate::game::asteroid::AsteroidPlugin)
            .add(crate::game::laser::LaserPlugin)
            .add(crate::game::score::ScorePlugin)
    }
}
