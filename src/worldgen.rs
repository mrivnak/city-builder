use bevy::prelude::*;
use rand::Rng;

pub fn generate_world(xsize: u32, ysize: u32, zsize: u32) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut world: Vec<Vec3> = Vec::new();
    for x in 0..xsize {
        for y in 0..ysize {
            // TODO: use a noise generator rather than random
            let z: u32 = rng.gen_range(0..zsize);
            world.push(Vec3 { x: x as f32, y: y as f32, z: (z as f32) / 10. });
        }
    }
    world
}