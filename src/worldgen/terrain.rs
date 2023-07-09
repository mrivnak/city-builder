pub struct TerrainNode {
    terrain: Terrain,
    x: u32,
    z: u32,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
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
