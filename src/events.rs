use bevy::prelude::*;

#[derive(Event)]
pub struct GameStartRequested {
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
    pub won: bool,
}
