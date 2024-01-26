mod components;
mod events;
mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
