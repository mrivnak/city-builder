use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Tile {}

#[derive(Bundle, Default)]
pub struct Prop {}

#[derive(Bundle, Default)]
pub struct TileBundle {
    pub tile: Tile,

    #[bundle]
    pub model: SceneBundle,
}

#[derive(Bundle, Default)]
pub struct PropBundle {
    pub tile: Tile,

    #[bundle]
    pub model: SceneBundle,
}