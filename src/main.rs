use bevy::prelude::*;
use plugins::WorldGenPlugin;

mod components;
mod plugins;
mod worldgen;

fn main() {
    App::new().add_plugin(WorldGenPlugin).run();
}
