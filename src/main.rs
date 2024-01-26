mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use bevy::prelude::*;

use bevy_xpbd_2d::prelude::PhysicsPlugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, spawn_player)
        .run();
}
