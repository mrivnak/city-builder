use bevy::prelude::*;

use crate::components::{Road, RoadType, Tile, TileBundle};
use crate::worldgen;

const WORLD_X_SIZE: u32 = 20;
const WORLD_Y_SIZE: u32 = 20;
const WORLD_Z_SIZE: u32 = 5;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(generate_world);
    }
}

fn generate_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let tiles = worldgen::generate_world(WORLD_X_SIZE, WORLD_Y_SIZE, WORLD_Z_SIZE)
        .into_iter()
        .map(|vec| TileBundle {
            tile: Tile {},
            object: MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    base_color: Color::Lcha { lightness: 0.5, chroma: 0.5, hue: 130.0, alpha: 1.0 },
                    ..Default::default()
                }),
                transform: Transform::from_xyz(vec.x, vec.y, vec.z),
                ..Default::default()
            },
         }).collect::<Vec<TileBundle>>();
    commands.spawn_batch(tiles);
}
