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
