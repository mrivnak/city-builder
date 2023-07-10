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
    Dirt,
    Water,
}

impl Terrain {
    /// Get terrains that cannot be adjacent to this terrain
    pub fn get_invalid(&self) -> Vec<Terrain> {
        match self {
            Terrain::Grass => vec![],
            Terrain::Dirt => vec![Terrain::Water],
            Terrain::Water => vec![Terrain::Dirt],
        }
    }
}
