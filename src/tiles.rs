use std::f32::consts::*;

use bevy::prelude::*;

use crate::{
    components::TileBundle,
    worldgen::{Terrain, TerrainContour},
};

pub trait Tile {
    fn get_scene(&self, assets: &Res<AssetServer>) -> Handle<Scene>;
    // x and z are coordinates relative to the grid, not world coordinates
    fn get_transform(&self, x: u32, z: u32) -> Transform;
    fn get_bundle(&self, x: u32, z: u32, assets: &Res<AssetServer>) -> TileBundle;
}

// terrain form should already be generated, this just produces a tile to render
pub fn get_tile_from_Terrain(terrain: Terrain) -> Box<dyn Tile> {
    match terrain.contour {
        TerrainContour::Flat => Box::new(FlatTile {
            height: terrain.height,
        }),
        TerrainContour::North => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::North,
        }),
        TerrainContour::NorthEast(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::NorthEast,
            corner_type,
        }),
        TerrainContour::East => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::East,
        }),
        TerrainContour::SouthEast(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::SouthEast,
            corner_type,
        }),
        TerrainContour::South => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::South,
        }),
        TerrainContour::SouthWest(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::SouthWest,
            corner_type,
        }),
        TerrainContour::West => Box::new(EdgeTile {
            height: terrain.height,
            orientation: EdgeOrientation::West,
        }),
        TerrainContour::NorthWest(corner_type) => Box::new(CornerTile {
            height: terrain.height,
            orientation: CornerOrientation::NorthWest,
            corner_type,
        }),
    }
}

enum CornerOrientation {
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

pub enum CornerType {
    Inner,
    Outer,
}

enum EdgeOrientation {
    North,
    East,
    South,
    West,
}

struct CornerTile {
    height: u8,
    orientation: CornerOrientation,
    corner_type: CornerType,
}

impl Tile for CornerTile {
    fn get_scene(&self, assets: &Res<AssetServer>) -> Handle<Scene> {
        match self.corner_type {
            CornerType::Inner => assets.load("models/cliff_stepsCornerInner_rock.glb#Scene0"),
            CornerType::Outer => assets.load("models/cliff_stepsCorner_rock.glb#Scene0"),
        }
    }

    fn get_transform(&self, x: u32, z: u32) -> Transform {
        let rotation = match self.orientation {
            CornerOrientation::NorthWest => Quat::from_rotation_y(PI),
            CornerOrientation::NorthEast => Quat::from_rotation_y(FRAC_PI_2),
            CornerOrientation::SouthEast => Quat::from_rotation_y(0.),
            CornerOrientation::SouthWest => Quat::from_rotation_y(-FRAC_PI_2),
        };

        Transform::from_xyz(x as f32, self.height as f32, z as f32).with_rotation(rotation)
    }

    fn get_bundle(&self, x: u32, z: u32, assets: &Res<AssetServer>) -> TileBundle {
        let scene = self.get_scene(assets);
        let transform = self.get_transform(x, z);
        TileBundle {
            model: SceneBundle {
                scene,
                transform,
                ..default()
            },
            ..default()
        }
    }
}

struct EdgeTile {
    height: u8,
    orientation: EdgeOrientation,
}

impl Tile for EdgeTile {
    fn get_scene(&self, assets: &Res<AssetServer>) -> Handle<Scene> {
        assets.load("models/cliff_steps_rock.glb#Scene0")
    }

    fn get_transform(&self, x: u32, z: u32) -> Transform {
        let rotation = match self.orientation {
            EdgeOrientation::North => Quat::from_rotation_y(FRAC_PI_2),
            EdgeOrientation::East => Quat::from_rotation_y(0.),
            EdgeOrientation::South => Quat::from_rotation_y(-FRAC_PI_2),
            EdgeOrientation::West => Quat::from_rotation_y(PI),
        };

        Transform::from_xyz(x as f32, self.height as f32, z as f32).with_rotation(rotation)
    }

    fn get_bundle(&self, x: u32, z: u32, assets: &Res<AssetServer>) -> TileBundle {
        println!("edge tile");
        let scene = self.get_scene(assets);
        println!("got scene: {:?}", scene.id());
        let transform = self.get_transform(x, z);
        println!("got transform: {:?}", transform);
        TileBundle {
            model: SceneBundle {
                scene,
                transform,
                ..default()
            },
            ..default()
        }
    }
}

struct FlatTile {
    height: u8,
}

impl Tile for FlatTile {
    fn get_scene(&self, assets: &Res<AssetServer>) -> Handle<Scene> {
        assets.load("models/ground_grass.glb#Scene0")
    }

    fn get_transform(&self, x: u32, z: u32) -> Transform {
        Transform::from_xyz(x as f32, self.height as f32, z as f32)
    }

    fn get_bundle(&self, x: u32, z: u32, assets: &Res<AssetServer>) -> TileBundle {
        let scene = self.get_scene(assets);
        let transform = self.get_transform(x, z);
        TileBundle {
            model: SceneBundle {
                scene,
                transform,
                ..default()
            },
            ..default()
        }
    }
}
