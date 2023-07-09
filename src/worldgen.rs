use bevy::prelude::*;

mod terrain;
mod tiles;
mod wave_fn_collapse;

use crate::components::TileBundle;

use self::{tiles::get_tile_from_terrain_node, wave_fn_collapse::generate_world_nodes};

pub fn generate_world(xsize: u32, zsize: u32, assets: Res<AssetServer>) -> Vec<TileBundle> {
    generate_world_nodes(xsize, zsize)
        .into_iter()
        .flatten()
        .map(get_tile_from_terrain_node)
        .collect::<Vec<TileBundle>>()
}
