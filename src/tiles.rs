use bevy::prelude::*;

use crate::worldgen::{Terrain, TerrainContour};

pub trait Tile {
    fn get_scene(&self) -> Handle<Scene>;
    // x and z are coordinates relative to the grid, not world coordinates
    fn get_transform(&self, x: u32, z: u32) -> Transform;
}

// terrain form should already be generated, this just produces a tile to render
pub fn get_tile_from_Terrain(terrain: Terrain) -> Box<dyn Tile> {
    match terrain.contour {
        TerrainContour::Flat => Box::new(FlatTile {
            height: terrain.height,
        }),
        TerrainContour::North => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::North,
        }),
        TerrainContour::NorthEast(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::NorthEast,
            corner_type,
        }),
        TerrainContour::East => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::East,
        }),
        TerrainContour::SouthEast(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::SouthEast,
            corner_type,
        }),
        TerrainContour::South => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::South,
        }),
        TerrainContour::SouthWest(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::SouthWest,
            corner_type,
        }),
        TerrainContour::West => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::West,
        }),
        TerrainContour::NorthWest(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::NorthWest,
            corner_type,
        }),
    }
}

enum CornerOrientation {
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

pub enum CornerType {
    Inner,
    Outer,
}

enum EdgeOrientation {
    North,
    East,
    South,
    West,
}

struct CornerTile {
    height: u8,
    orientation: CornerOrientation,
    corner_type: CornerType,
}

impl Tile for CornerTile {
    fn get_scene(&self) -> Handle<Scene> {
        unimplemented!()
    }

    fn get_transform(&self, x: u32, z: u32) -> Transform {
        unimplemented!()
    }
}

struct EdgeTile {
    height: u8,
    orientation: EdgeOrientation,
}

impl Tile for EdgeTile {
    fn get_scene(&self) -> Handle<Scene> {
        unimplemented!()
    }

    fn get_transform(&self, x: u32, z: u32) -> Transform {
        unimplemented!()
    }
}

struct FlatTile {
    height: u8,
}

impl Tile for FlatTile {
    fn get_scene(&self) -> Handle<Scene> {
        unimplemented!()
    }

    fn get_transform(&self, x: u32, z: u32) -> Transform {
        unimplemented!()
    }
}
