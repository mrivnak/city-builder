use bevy::prelude::*;

use crate::components::{Road, RoadType, Tile};
use crate::worldgen;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_world)
            .add_system(print_world);
    }
}

fn generate_world(mut commands: Commands) {
    let tiles = worldgen::generate_world(10, 10, 10)
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
