use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

use super::terrain::{Terrain, TerrainNode};

const MAX_HEIGHT: u32 = 100;
const MIN_HEIGHT: u32 = 0;
const INITIAL_ROUGHNESS: u8 = 110;
const PERCENT_WATER: u32 = 20;

pub fn generate_world_nodes(size: u32) -> Vec<Vec<TerrainNode>> {
    let start = Instant::now();
    let mut height_map = generate_height_map(normalize_grid_size(size));

    trim_grid(size, &mut height_map);

    let set_node = |x: u32, z: u32, height: u32| -> TerrainNode {
        let terrain = match height {
            x if x < PERCENT_WATER => Terrain::Water,
            _ => Terrain::Grass, // TODO: add forest, mountain, etc.
        };
        TerrainNode { x, z, terrain }
    };

    let terrain_map = height_map
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(z, height)| set_node(x as u32, z as u32, *height))
                .collect()
        })
        .collect();

    println!("World generated in {:?}", start.elapsed());
    terrain_map
}

fn check(map: &Vec<Vec<u32>>) -> bool {
    const THRESHOLD: u32 = 5;
    let mut count = 0;
    for row in map {
        for height in row {
            if *height < PERCENT_WATER {
                count += 1;
            }
        }
    }
    let actual_percent = (count * 100) / (map.len() * map.len()) as u32;
    let diff = actual_percent.abs_diff(PERCENT_WATER);
    println!("count: {}", count);
    println!("size: {}", map.len() * map.len());
    println!("Actual percent: {}", actual_percent);
    diff < THRESHOLD
}

fn generate_height_map(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;

    let mut height_rng = StdRng::from_entropy();
    let mut rough_rng = StdRng::from_entropy();
    let mut get_rand_height = || height_rng.gen_range(MIN_HEIGHT..MAX_HEIGHT);

    let mut map = vec![vec![0_u32; size]; size];
    loop {
        let mut roughness = INITIAL_ROUGHNESS as i32;
        let mut chunk_size = size - 1;

        // Set corners
        map[0][0] = get_rand_height();
        map[0][size - 1] = get_rand_height();
        map[size - 1][0] = get_rand_height();
        map[size - 1][size - 1] = get_rand_height();

        while chunk_size > 1 {
            square_step(&mut map, &chunk_size, &roughness, &mut rough_rng);
            diamond_step(&mut map, &chunk_size, &roughness, &mut rough_rng);

            chunk_size /= 2;
            roughness /= 2;
        }

        if check(&map) {
            println!("World generation complete");
            break;
        }
        println!("World generation failed, retrying");
    }

    map
}

fn square_step(map: &mut Vec<Vec<u32>>, chunk_size: &usize, roughness: &i32, rng: &mut StdRng) {
    let half = chunk_size / 2;
    for i in (0..map.len() - 1).step_by(*chunk_size) {
        for j in (0..map.len() - 1).step_by(*chunk_size) {
            let sum = map[i][j]
                + map[i + chunk_size][j]
                + map[i][j + chunk_size]
                + map[i + chunk_size][j + chunk_size];
            let avg = sum / 4;
            let random = match roughness {
                0 => 0,
                _ => rng.gen_range(-*roughness..*roughness),
            };
            map[i + half][j + half] = avg.saturating_add_signed(random);
        }
    }
}

fn diamond_step(map: &mut Vec<Vec<u32>>, chunk_size: &usize, roughness: &i32, rng: &mut StdRng) {
    let half = chunk_size / 2;

    for i in (0..map.len()).step_by(half) {
        for j in ((i + half) % chunk_size..map.len()).step_by(*chunk_size) {
            let mut sum: u32 = 0;
            let mut count: u32 = 0;

            if half <= i {
                sum += map[i - half][j];
                count += 1;
            }
            if half <= j {
                sum += map[i][j - half];
                count += 1;
            }
            if j + half < map.len() {
                sum += map[i][j + half];
                count += 1;
            }
            if i + half < map.len() {
                sum += map[i + half][j];
                count += 1;
            }

            let avg = sum / count;
            let random = match roughness {
                0 => 0,
                _ => rng.gen_range(-*roughness..*roughness),
            };
            map[i][j] = avg.saturating_add_signed(random);
        }
    }
}

/// diamond square algorithm requires a grid of size 2^n + 1
fn normalize_grid_size(size: u32) -> u32 {
    let mut exp: u32 = 2;
    while 2_u32.pow(exp) + 1 < size {
        exp += 1;
    }
    2_u32.pow(exp) + 1
}

fn trim_grid(size: u32, grid: &mut Vec<Vec<u32>>) {
    grid.truncate(size as usize);
    for row in grid.iter_mut() {
        row.truncate(size as usize);
    }
}

#[cfg(test)]
mod test {
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
        let mut grid = vec![vec![1; 5]; 5];

        trim_grid(3, &mut grid);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0].len(), 3);
    }
}
