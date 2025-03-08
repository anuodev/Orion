use bevy::prelude::*;

use crate::config::APP_FONT;
use crate::game::ui::hud::components::*;

use crate::game::score::resources::Score;
use crate::game::ui::hud::components::ScoreText;

pub fn hud_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(200.0),
                    height: Val::Percent(80.0),
                    margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("0"),
                        TextLayout::default(),
                        TextFont {
                            font: asset_server.load(APP_FONT),
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        ScoreText {},
                    ));
                });
        })
        .id();

    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn hud_update(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.0 = score.value.to_string();
        }
    }
}
