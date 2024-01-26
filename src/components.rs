use bevy::prelude::*;

#[derive(Component)]
pub struct IntroScreen {}

#[derive(Component)]
pub struct GameScreen {}

#[derive(Component)]
pub struct GameOverScreen {}

#[derive(Component)]
pub struct MenuMusic {}

#[derive(Component)]
pub struct GameStartMusic {}

#[derive(Component)]
pub struct GameOverMusic {}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
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

#[derive(Component)]
pub struct LivesCounter;

