use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,

    #[bundle]
    pub object: MaterialMeshBundle<StandardMaterial>
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
