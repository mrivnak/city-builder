use bevy::prelude::*;

mod diamond_square;
mod terrain;
mod tiles;

use crate::components::TileBundle;

use self::{diamond_square::generate_world_nodes, tiles::get_tile_from_terrain_node};

pub fn generate_world(size: u32, assets: Res<AssetServer>) -> Vec<TileBundle> {
    generate_world_nodes(size)
        .into_iter()
        .flatten()
        .map(|node| get_tile_from_terrain_node(node, &assets))
        .collect::<Vec<TileBundle>>()
}
