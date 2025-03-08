mod app;
mod config;
mod events;
mod game;
mod main_menu;

use bevy::{prelude::*, window::PresentMode};
use iyes_perf_ui::prelude::*;

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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(crate::app::AppPlugin)
        .add_plugins(crate::main_menu::MainMenuPlugin)
        .add_plugins(crate::game::GamePlugin)
        .run();
}
