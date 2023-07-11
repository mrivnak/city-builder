use bevy::prelude::*;

mod diamond_square;
mod terrain;

use crate::components::{PropBundle, TileBundle};
use crate::worldgen::terrain::*;

use self::diamond_square::generate_world_nodes;

pub fn generate_world(size: u32, asset_server: Res<AssetServer>) -> (Vec<TileBundle>, Vec<PropBundle>) {
    let nodes = generate_world_nodes(size).into_iter().flatten();

    let mut tiles = Vec::new();
    let mut props = Vec::new();

    // TODO: add random tranlate/rotate to props
    // TODO: take into account neighboring terrain for tile assets, e.g. river banks
    for node in nodes {
        match node.terrain {
            Terrain::Grass => {
                tiles.push(TileBundle {
                    model: SceneBundle {
                        scene: asset_server.load("models/ground_grass.glb#Scene0"),
                        transform: Transform::from_xyz(node.x as f32, 0.0, node.z as f32),
                        ..default()
                    },
                    ..default()
                });
            },
            Terrain::GrassWith(prop) => {
                tiles.push(TileBundle {
                    model: SceneBundle {
                        scene: asset_server.load("models/ground_grass.glb#Scene0"),
                        transform: Transform::from_xyz(node.x as f32, 0.0, node.z as f32),
                        ..default()
                    },
                    ..default()
                });
                match prop {
                    TerrainProp::Tree => {
                        props.push(PropBundle {
                            model: SceneBundle {
                                scene: asset_server.load("models/tree_default.glb#Scene0"),
                                transform: Transform::from_xyz(node.x as f32, 0.0, node.z as f32),
                                ..default()
                            },
                            ..default()
                        });
                    }
                    TerrainProp::Rock => {
                        props.push(PropBundle {
                            model: SceneBundle {
                                scene: asset_server.load("models/stone_smallA.glb#Scene0"), // TODO: pick a model
                                transform: Transform::from_xyz(node.x as f32, 1.0, node.z as f32),
                                ..default()
                            },
                            ..default()
                        });
                    }
                }
            }
            Terrain::Water => {
                tiles.push(TileBundle {
                    model: SceneBundle {
                        scene: asset_server.load("models/ground_riverOpen.glb#Scene0"),
                        transform: Transform::from_xyz(node.x as f32, 0.0, node.z as f32),
                        ..default()
                    },
                    ..default()
                });
            }
            Terrain::Dirt => {
                tiles.push(TileBundle {
                    model: SceneBundle {
                        scene: asset_server.load("models/ground_grass.glb#Scene0"),
                        transform: Transform::from_xyz(node.x as f32, 0.0, node.z as f32),
                        ..default()
                    },
                    ..default()
                });
            }
        }
        
        if let Some(resource) = node.resource {
            match resource {
                terrain::Resource::Coal => {
                    props.push(PropBundle {
                        model: SceneBundle {
                            scene: asset_server.load("models/stone_smallA.glb#Scene0"), // TODO: pick a model
                            transform: Transform::from_xyz(node.x as f32, 1.0, node.z as f32),
                            ..default()
                        },
                        ..default()
                    });
                }
                terrain::Resource::Iron => {
                    props.push(PropBundle {
                        model: SceneBundle {
                            scene: asset_server.load("models/stone_smallA.glb#Scene0"), // TODO: pick a model
                            transform: Transform::from_xyz(node.x as f32, 1.0, node.z as f32),
                            ..default()
                        },
                        ..default()
                    });
                }
            }
        }
    }

    (tiles, props)
}
