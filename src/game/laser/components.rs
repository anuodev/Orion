use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Laser {
    pub speed: f32,
    pub direction: Vec3,
}
