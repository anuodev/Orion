mod game;
mod events;
mod config;
mod app;
mod main_menu;

use bevy::prelude::*;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Orion".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(crate::app::AppPlugin)
        .add_plugins(crate::main_menu::MainMenuPlugin)
        .add_plugins(crate::game::GamePlugin)
        .run();
}