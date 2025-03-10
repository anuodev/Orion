pub mod asteroid;
pub mod laser;
pub mod player;
pub mod score;
pub mod ui;
pub mod world;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugin;
impl PluginGroup for GamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::game::ui::UIPlugin)
            .add(crate::game::world::WorldPlugin)
            .add(crate::game::player::PlayerPlugin)
            .add(crate::game::asteroid::AsteroidPlugin)
            .add(crate::game::laser::LaserPlugin)
            .add(crate::game::score::ScorePlugin)
    }
}
