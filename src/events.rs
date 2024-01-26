use bevy::prelude::*;

#[derive(Event)]
pub struct StartGame {
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
    pub won: bool,
}
