use bevy::prelude::*;
use crate::score::*;

pub fn display_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}