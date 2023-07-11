use bevy::prelude::*;

use crate::components::{Tile, TileBundle};

use super::terrain::{TerrainNode, Terrain};

// terrain form should already be generated, this just produces a tile to render
pub fn get_tile_from_terrain_node(node: TerrainNode, assets: &Res<AssetServer>) -> TileBundle {
    TileBundle {
        tile: Tile {},
        model: SceneBundle {
            scene: match node.terrain {
                Terrain::Water => assets.load("models/ground_riverOpen.glb#Scene0"),
                Terrain::Grass => assets.load("models/ground_grass.glb#Scene0"),
                Terrain::Dirt => assets.load("models/ground_pathOpen.glb#Scene0"),
            },
            transform: Transform::from_xyz(node.x as f32, 0., node.z as f32),
            ..default()
        },
    }
}
