mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use bevy::prelude::*;

use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .insert_resource(Gravity(Vec2::NEG_Y * 0.0))
        .run();
}
