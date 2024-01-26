mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use bevy::prelude::*;
use bevy::ui::AlignSelf::Start;
use bevy::window::{PresentMode, WindowTheme};

use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(Gravity(Vec2::NEG_Y * 0.0))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
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
        })))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_castles)
        .add_systems(Update, spawn_bullet)
        .add_systems(Update, move_bullet)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_hit_player)
        .run();
}
