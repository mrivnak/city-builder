use bevy::prelude::*;
use rand::Rng;

mod diamond_square;
mod terrain;

use crate::components::{PropBundle, ResourceBundle, TileBundle};
use crate::resources::ResourceType;
use crate::worldgen::terrain::*;

use self::diamond_square::generate_world_nodes;

const NUM_COAL_VEINS: u32 = 5;
const NUM_IRON_VEINS: u32 = 5;
const NUM_OIL_VEINS: u32 = 5;

pub fn generate_world(
    size: u32,
    asset_server: Res<AssetServer>,
) -> (Vec<TileBundle>, Vec<PropBundle>, Vec<ResourceBundle>) {
    // World generation
    let start = std::time::Instant::now();
    let mut nodes = generate_world_nodes(size);

    add_resources(&mut nodes);

    println!("World generation took {:?}", start.elapsed());

    let nodes = nodes.into_iter().flatten();

    // Entity bundling
    let mut tiles = Vec::new();
    let mut props = Vec::new();
    let mut resources = Vec::new();

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
            }
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
                    TerrainProp::Resource(resource) => {
                        use crate::components::Resource;

                        match resource {
                            ResourceType::Coal => {
                                resources.push(ResourceBundle {
                                    resource: Resource {
                                        resource_type: ResourceType::Coal,
                                    },
                                    model: SceneBundle {
                                        scene: asset_server.load("models/stone_largeA.glb#Scene0"), // TODO: pick a model
                                        transform: Transform::from_xyz(
                                            node.x as f32,
                                            0.0,
                                            node.z as f32,
                                        ),
                                        ..default()
                                    },
                                });
                            }
                            ResourceType::Iron => {
                                resources.push(ResourceBundle {
                                    resource: Resource {
                                        resource_type: ResourceType::Iron,
                                    },
                                    model: SceneBundle {
                                        scene: asset_server.load("models/rock_largeA.glb#Scene0"), // TODO: pick a model
                                        transform: Transform::from_xyz(
                                            node.x as f32,
                                            0.0,
                                            node.z as f32,
                                        ),
                                        ..default()
                                    },
                                });
                            }
                            ResourceType::Oil => {
                                resources.push(ResourceBundle {
                                    resource: Resource {
                                        resource_type: ResourceType::Iron,
                                    },
                                    model: SceneBundle {
                                        scene: asset_server.load("models/stump_square.glb#Scene0"), // TODO: pick a model
                                        transform: Transform::from_xyz(
                                            node.x as f32,
                                            0.0,
                                            node.z as f32,
                                        ),
                                        ..default()
                                    },
                                });
                            }
                        }
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
    }

    (tiles, props, resources)
}

fn add_resources(map: &mut Vec<Vec<TerrainNode>>) {
    for _ in 0..NUM_COAL_VEINS {
        add_single_resource(map, ResourceType::Coal);
    }

    for _ in 0..NUM_IRON_VEINS {
        add_single_resource(map, ResourceType::Iron);
    }

    for _ in 0..NUM_OIL_VEINS {
        add_single_resource(map, ResourceType::Oil);
    }
}

fn add_single_resource(map: &mut Vec<Vec<TerrainNode>>, resource: ResourceType) {
    let mut rng = rand::thread_rng();

    let mut success = false;

    loop {
        let x = rng.gen_range(0..map.len());
        let z = rng.gen_range(0..map.len());

        for i in -2..=2 {
            for j in -2..=2 {
                // skip sometimes
                if rng.gen_range(0..=5) != 0 && (i != 0 && j != 0) {
                    continue;
                }

                if x as i32 + i < 0
                    || x as i32 + i >= map.len() as i32
                    || z as i32 + j < 0
                    || z as i32 + j >= map.len() as i32
                {
                    continue;
                }

                let x = (x as i32 + i) as usize;
                let z = (z as i32 + j) as usize;

                match map[x][z].terrain {
                    Terrain::GrassWith(_) => {
                        map[x][z].terrain =
                            Terrain::GrassWith(TerrainProp::Resource(resource.clone()));
                        success = true;
                    }
                    Terrain::Grass => {
                        map[x][z].terrain =
                            Terrain::GrassWith(TerrainProp::Resource(resource.clone()));
                        success = true;
                    }
                    Terrain::Water => continue,
                    Terrain::Dirt => continue,
                }
            }
        }
        if success {
            break;
        }
    }
}
