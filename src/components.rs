use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub position: Vec3,
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
