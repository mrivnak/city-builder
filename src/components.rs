use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,

    #[bundle]
    pub gltf_object: SceneBundle
}
