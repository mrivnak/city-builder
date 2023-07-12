use bevy::prelude::*;

use crate::resources::ResourceType;

#[derive(Component, Default)]
pub struct Tile {}

#[derive(Component, Default)]
pub struct Prop {}

#[derive(Component)]
pub struct Resource {
    pub resource_type: ResourceType,
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    pub tile: Tile,

    #[bundle]
    pub model: SceneBundle,
}

#[derive(Bundle, Default)]
pub struct PropBundle {
    pub prop: Prop,

    #[bundle]
    pub model: SceneBundle,
}

#[derive(Bundle)]
pub struct ResourceBundle {
    pub resource: Resource,

    #[bundle]
    pub model: SceneBundle,
}