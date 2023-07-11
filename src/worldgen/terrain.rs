#[derive(Clone)]
pub struct TerrainNode {
    pub terrain: Terrain,
    pub resource: Option<Resource>,
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
}

#[derive(Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Resource {
    Coal,
    Iron,
}