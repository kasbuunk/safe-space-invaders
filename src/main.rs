mod components;
mod events;
mod resources;
mod systems;

use crate::components::{Bullet, Enemy};
use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use bevy_xpbd_2d::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vector::ZERO))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Safe space invaders!".into(),
                resolution: (WINDOW_HEIGHT, WINDOW_WIDTH).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Game>()
        .init_resource::<Score>()
        .init_resource::<HighScore>()
        .init_resource::<Lives>()
        .init_resource::<EnemyInfo>()
        .init_resource::<EnemyCatalog>()
        .add_event::<GameOver>()
        .add_event::<StartGame>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_game_intro)
        .add_systems(Startup, start_menu_music)
        .add_systems(Update, spawn_game_background)
        .add_systems(Update, spawn_bullet)
        .add_systems(Update, bullet_hits_enemy)
        .add_systems(Update, move_bullet)
        .add_systems(Update, move_enemy_bullet)
        .add_systems(Update, setup_lives)
        .add_systems(Update, start_game)
        .add_systems(Update, spawn_player)
        .add_systems(Update, spawn_castles)
        .add_systems(Update, spawn_enemies)
        .add_systems(Update, player_movement)
        .add_systems(Update, enemy_movements)
        .add_systems(Update, enemy_shoot)
        .add_systems(Update, update_enemy_info)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, bullet_hits_castle)
        .add_systems(Update, enemy_bullet_hits_player)
        .add_systems(Update, enemy_bullet_hits_castle)
        .add_systems(Update, update_lives)
        .add_systems(Update, handle_game_start_music)
        .add_systems(Update, handle_game_over_music)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, new_game)
        .add_systems(Update, detect_game_won)
        .run();
}
