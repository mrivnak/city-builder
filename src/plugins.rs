use bevy::prelude::*;

use crate::worldgen;

// Square world
const WORLD_SIZE: u32 = 128;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_world);
    }
}

fn generate_world(mut commands: Commands, assets: Res<AssetServer>) {
    let (tiles, props, resources) = worldgen::generate_world(WORLD_SIZE, assets);
    
    commands.spawn_batch(tiles);
    commands.spawn_batch(props);
    commands.spawn_batch(resources);
}
