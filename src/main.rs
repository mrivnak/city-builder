use bevy::prelude::*;
use plugins::WorldGenPlugin;

mod components;
mod plugins;

fn main() {
    App::new().add_plugin(WorldGenPlugin).run();
}
