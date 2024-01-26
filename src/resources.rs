use bevy::prelude::*;

#[derive(Resource)]
pub struct Game {
    pub started: bool,
}

impl Default for Game {
    fn default() -> Game {
        Game { started: false }
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct Lives {
    pub value: u32,
}

impl Default for Lives {
    fn default() -> Lives {
        Lives { value: 3 }
    }
}

#[derive(Resource)]
pub struct EnemyInfo {
    pub enemy_directions: Vec2,
}
