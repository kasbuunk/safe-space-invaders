use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Castle {
    pub hitpoints: u32,
}
