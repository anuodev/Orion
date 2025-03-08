mod app;
mod config;
mod events;
mod game;
mod main_menu;

use bevy::{prelude::*, window::PresentMode};

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Orion".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(crate::app::AppPlugin)
        .add_plugins(crate::main_menu::MainMenuPlugin)
        .add_plugins(crate::game::GamePlugin)
        .run();
}
