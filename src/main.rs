mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use bevy::prelude::*;
use bevy::ui::AlignSelf::Start;

use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Gravity(Vec2::NEG_Y * 0.0))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_castles)
        .add_systems(Startup, spawn_bullet)
        .add_systems(Startup, move_bullet)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .run();
}
