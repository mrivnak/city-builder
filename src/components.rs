use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug)]
pub enum RoadType {
    Dirt,
    Cobblestone,
}

#[derive(Component)]
pub struct Road {
    pub variant: RoadType,
    pub bidirectional: bool,
}
