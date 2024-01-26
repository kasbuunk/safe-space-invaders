use bevy::prelude::*;

#[derive(Component)]
pub struct Intro {}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bullet {
    pub speed: u8,
}

#[derive(Component)]
pub struct Castle {
    pub hitpoints: u32,
}

#[derive(Component, Default)]
pub struct Enemy {
    pub level: u32,
    pub is_dead: bool,
}
