use super::terrain::TerrainNode;

pub fn generate_world_nodes(size: u32) -> Vec<Vec<TerrainNode>> {
    todo!()
}

fn normalize_grid_size(size: u32) -> u32 {
    let mut exp: u32 = 2;
    while 2_u32.pow(exp) + 1 < size {
        exp += 1;
    }
    2_u32.pow(exp) + 1
}

fn trim_grid(size: u32, grid: &mut Vec<Vec<TerrainNode>>) {
    grid.truncate(size as usize);
    for row in grid.iter_mut() {
        row.truncate(size as usize);
    }
}

#[cfg(test)]
mod test {
    use crate::worldgen::terrain::Terrain;

    use super::*;
    use test_case::test_case;

    #[test_case(10, 17)]
    #[test_case(16, 17)]
    #[test_case(20, 33)]
    #[test_case(34, 65)]
    fn test_normalize_grid_size(size: u32, expected: u32) {
        let actual = normalize_grid_size(size);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_trim_grid() {
        let example_node = TerrainNode {
            terrain: Terrain::Grass,
            x: 0,
            z: 0,
        };
        let mut grid = vec![
            vec![example_node.clone(); 5],
            vec![example_node.clone(); 5],
            vec![example_node.clone(); 5],
            vec![example_node.clone(); 5],
            vec![example_node.clone(); 5],
        ];

        trim_grid(3, &mut grid);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0].len(), 3);
    }
}