use bevy::prelude::*;

use crate::{
    components::TileBundle,
    tiles::CornerType,
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
