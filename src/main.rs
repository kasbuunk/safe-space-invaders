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
                resolution: (800., 600.).into(),
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
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: true,
                ..default()
            }),
            ..default()
        })))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_castles)
        .add_systems(Startup, spawn_bullet)
        .add_systems(Startup, move_bullet)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .run();
}
