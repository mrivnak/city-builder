use bevy::prelude::*;

use crate::components::{Road, RoadType, Tile};
use crate::worldgen;

const WORLD_X_SIZE: u32 = 20;
const WORLD_Y_SIZE: u32 = 20;
const WORLD_Z_SIZE: u32 = 5;
pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_world)
            .add_system(print_world);
    }
}

fn generate_world(mut commands: Commands) {
    let tiles = worldgen::generate_world(WORLD_X_SIZE, WORLD_Y_SIZE, WORLD_Z_SIZE)
        .into_iter()
        .map(|vec| Tile { position: vec });
    commands.spawn_batch(tiles);
}

fn print_world(query: Query<(&Tile, Option<&Road>)>) {
    for (tile, road) in query.iter() {
        match road {
            Some(road) => {
                println!(
                    "Tile at ({}, {}, {}) with road {:?}",
                    tile.position.x, tile.position.y, tile.position.z, road.variant
                );
            }
            None => {
                println!(
                    "Tile at ({}, {}, {})",
                    tile.position.x, tile.position.y, tile.position.z
                );
            }
        }
    }
}
