use crate::resources::ResourceType;

#[derive(Clone)]
pub struct TerrainNode {
    pub terrain: Terrain,
    pub x: u32,
    pub z: u32,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Terrain {
    Grass,
    GrassWith(TerrainProp),
    Water,
    Dirt,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum TerrainProp {
    Tree,
    Rock,
    Resource(ResourceType),
}
