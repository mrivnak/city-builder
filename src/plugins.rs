use bevy::prelude::*;

use crate::worldgen;

const WORLD_X_SIZE: u32 = 20;
const WORLD_Y_SIZE: u32 = 20;
const WORLD_Z_SIZE: u32 = 5;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_world);
    }
}

fn generate_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let tiles = worldgen::generate_world(
        WORLD_X_SIZE,
        WORLD_Y_SIZE,
        WORLD_Z_SIZE,
        assets,
    );
    commands.spawn_batch(tiles);
}
