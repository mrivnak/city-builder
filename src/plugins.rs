use bevy::prelude::*;

use crate::components::{Road, RoadType, Tile};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_world)
            .add_system(print_world);
    }
}

fn generate_world(mut commands: Commands) {
    commands.spawn(Tile { x: 0, y: 0, z: 0 });
    commands.spawn((
        Tile { x: 1, y: 0, z: 0 },
        Road {
            variant: RoadType::Dirt,
            bidirectional: true,
        },
    ));
}

fn print_world(query: Query<(&Tile, Option<&Road>)>) {
    for (tile, road) in query.iter() {
        match road {
            Some(road) => {
                println!(
                    "Tile at ({}, {}, {}) with road {:?}",
                    tile.x, tile.y, tile.z, road.variant
                );
            }
            None => {
                println!("Tile at ({}, {}, {})", tile.x, tile.y, tile.z);
            }
        }
    }
}
