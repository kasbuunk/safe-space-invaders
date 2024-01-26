use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bullet {
    pub position: Vec2,
    pub speed: u8,
}

#[derive(Component)]
pub struct Castle {
    pub hitpoints: u32,
}
