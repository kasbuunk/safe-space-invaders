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

#[derive(Debug)]
pub enum EnemyStage {
    RIGHT,
    LEFT,
    DOWN(usize, bool),
}

#[derive(Resource)]
pub struct EnemyInfo {
    pub stage: EnemyStage,
}

impl Default for EnemyInfo {
    fn default() -> EnemyInfo {
        EnemyInfo {
            stage: EnemyStage::RIGHT,
        }
    }
}
