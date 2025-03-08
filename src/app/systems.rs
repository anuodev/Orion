use crate::config::APP_BACKGROUND;
use crate::events::GameOver;
use crate::game::score::resources::Score;
use bevy::app::AppExit;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn set_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Transform {
            translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, -1.0),
            ..default()
        },
        Sprite {
            image: asset_server.load(APP_BACKGROUND),
            custom_size: Option::from(window.size()),
            ..default()
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Transform {
            translation: vec3(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Camera2d { ..default() },
    ));
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Game Over: {}", event.score.to_string());
    }
}

pub fn display_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}
