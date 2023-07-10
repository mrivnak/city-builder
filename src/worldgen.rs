use bevy::prelude::*;

mod diamond_square;
mod terrain;
mod tiles;

use crate::components::TileBundle;

use self::{tiles::get_tile_from_terrain_node, diamond_square::generate_world_nodes};

pub fn generate_world(size: u32, assets: Res<AssetServer>) -> Vec<TileBundle> {
    generate_world_nodes(size)
        .into_iter()
        .flatten()
        .map(get_tile_from_terrain_node)
        .collect::<Vec<TileBundle>>()
}
