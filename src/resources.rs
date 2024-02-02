use bevy::prelude::*;

pub const NUMBER_OF_LIVES: u32 = 4;

#[derive(Resource, PartialEq, Eq)]
pub enum Game {
    INTRO,
    LOADING,
    STARTED,
    ENDED,
}

impl Default for Game {
    fn default() -> Game {
        Game::INTRO
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct LoadingFlags {
    pub player: bool,
    pub enemies: bool,
    pub castles: bool,
}

impl Default for LoadingFlags {
    fn default() -> LoadingFlags {
        LoadingFlags {
            castles: false,
            enemies: false,
            player: false,
        }
    }
}


impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct HighScore {
    pub value: u32,
}

impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { value: 0 }
    }
}

#[derive(Resource)]
pub struct Lives {
    pub value: u32,
}

impl Default for Lives {
    fn default() -> Lives {
        Lives {
            value: NUMBER_OF_LIVES,
        }
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

#[derive(Resource)]
pub struct EnemyCatalog {
    enemy_sprites: Vec<String>,
}

impl Default for EnemyCatalog {
    fn default() -> EnemyCatalog {
        EnemyCatalog {
            enemy_sprites: vec![
                "daan".to_string(),
                "erhan".to_string(),
                "frank".to_string(),
                "hus".to_string(),
                "jeroen".to_string(),
                "kas".to_string(),
                "ryan".to_string(),
                "storm".to_string(),
            ],
        }
    }
}

impl EnemyCatalog {
    pub fn get_random_enemy(&self) -> &str {
        let idx = rand::random::<usize>() % self.enemy_sprites.len();

        &self.enemy_sprites[idx]
    }
}
