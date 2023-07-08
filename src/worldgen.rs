use bevy::prelude::*;

use crate::{
    components::TileBundle,
    tiles::{get_tile_from_Terrain, CornerType},
};

fn generate_world_nodes(xsize: u32, ysize: u32, zsize: u32) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut world: Vec<Vec3> = Vec::new();
    world
}

pub fn generate_world(
    xsize: u32,
    ysize: u32,
    zsize: u32,
    assets: Res<AssetServer>,
) -> Vec<TileBundle> {
    let mut tiles = Vec::new();
    tiles.push(
        get_tile_from_Terrain(Terrain {
            contour: TerrainContour::North,
            height: 0,
        })
        .get_bundle(0, 0, &assets),
    );
    tiles.push(
        get_tile_from_Terrain(Terrain {
            contour: TerrainContour::NorthWest(CornerType::Outer),
            height: 0,
        })
        .get_bundle(0, 1, &assets),
    );
    tiles.push(
        get_tile_from_Terrain(Terrain {
            contour: TerrainContour::West,
            height: 0,
        })
        .get_bundle(1, 1, &assets),
    );
    tiles.push(
        get_tile_from_Terrain(Terrain {
            contour: TerrainContour::Flat,
            height: 1,
        })
        .get_bundle(1, 0, &assets),
    );
    tiles
}

pub struct Terrain {
    pub contour: TerrainContour,
    pub height: u8,
}

pub enum TerrainContour {
    Flat,
    North,
    NorthEast(CornerType),
    East,
    SouthEast(CornerType),
    South,
    SouthWest(CornerType),
    West,
    NorthWest(CornerType),
}

#[derive(Clone, Debug)]
enum TerrainType {
    Grass,
    Trees,
    Dirt,
    Water,
    Sand,
}
