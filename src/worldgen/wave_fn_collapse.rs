use super::terrain::{Terrain, TerrainNode};

const SIMILAR_TILE_WEIGHT: u8 = 20;
const DISSIMILAR_TILE_WEIGHT: u8 = 4;

const STEP_TILE_WEIGHT: u8 = 1;
const CORNER_TILE_WEIGHT: u8 = 1;

/// Generate a world using the wave function collapse algorithm
pub fn generate_world_nodes(xsize: u32, zsize: u32) -> Vec<Vec<TerrainNode>> {
    let mut tile_map = populate_initial_domain(xsize, zsize);
    let mut resolved = false;
    while !resolved {
        let (x, z) = find_lowest_entropy(&tile_map);
        collapse_tile(&mut tile_map, x, z);
        resolved = true;
    }
    todo!()
}

/// Populate the initial tile domain map
fn populate_initial_domain(xsize: u32, zsize: u32) -> Vec<Vec<TileDomain>> {
    // TODO: set tile weights
    todo!()
}

/// Find the (first) tile with the lowest entropy
fn find_lowest_entropy(tile_map: &Vec<Vec<TileDomain>>) -> (usize, usize) {
    todo!()
}

/// Collapse a tile to a single terrain and update the tile domain map
fn collapse_tile(tile_map: &mut Vec<Vec<TileDomain>>, x: usize, z: usize) {
    let options = &tile_map[x][z].options;
    todo!()
}

/// Check if all tiles have been resolved
fn check_complete(tile_map: &Vec<Vec<TileDomain>>) -> bool {
    todo!()
}

struct TileOption {
    tile: Terrain,
    weight: u8,
}

struct TileDomain {
    options: Vec<TileOption>,
    resolved: bool,
}

impl TileDomain {
    fn remove(invalid: &Vec<Terrain>) {
        todo!()
    }

    fn flatten(&self) -> Vec<Terrain> {
        let mut result = Vec::new();
        for option in &self.options {
            for _ in 0..option.weight {
                result.push(option.tile.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_lowest_entropy() {
        todo!()
    }

    #[test]
    fn test_remove_invalid() {
        todo!()
    }

    #[test]
    fn test_flatten() {
        let domain = TileDomain {
            options: vec![
                TileOption {
                    tile: Terrain::Grass,
                    weight: 1,
                },
                TileOption {
                    tile: Terrain::Dirt,
                    weight: 2,
                },
            ],
            resolved: false,
        };

        let expected = vec![Terrain::Grass, Terrain::Dirt, Terrain::Dirt];
        let actual = domain.flatten();

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], actual[i]);
        }
    }
}
