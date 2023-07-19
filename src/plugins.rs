use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    components::{OrthographicCameraHeading, PanOrbitCamera},
    consts::*,
    worldgen,
};

// Square world
const WORLD_SIZE: u32 = 128;

// Control limits
const MIN_SCALE: f32 = 5.0;
const MAX_SCALE: f32 = 2500.0;
const ZOOM_SPEED: f32 = 1.0;
const PAN_SPEED: f32 = 0.8;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_world);
    }
}

fn generate_world(mut commands: Commands, assets: Res<AssetServer>) {
    let (tiles, props, resources) = worldgen::generate_world(WORLD_SIZE, assets);

    commands.spawn_batch(tiles);
    commands.spawn_batch(props);
    commands.spawn_batch(resources);
}

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_camera_zoom)
            .add_systems(Update, control_camera_pan)
            .add_systems(Update, control_camera_rotate);
    }
}

fn control_camera_zoom(
    mut camera_query: Query<&mut Projection, With<PanOrbitCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut camera = camera_query.single_mut();

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if let Projection::Orthographic(ref mut p) = *camera {
                    let new = p.scale - ev.y * ZOOM_SPEED;
                    p.scale = if new > MAX_SCALE {
                        MAX_SCALE
                    } else if new < MIN_SCALE {
                        MIN_SCALE
                    } else {
                        new
                    }
                }
            }
            MouseScrollUnit::Pixel => {
                // TODO: Idk what to do with this
            }
        }
    }
}

fn control_camera_pan(
    mut camera_query: Query<(&mut Transform, &PanOrbitCamera)>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut transform, camera) = camera_query.single_mut();
    let mut velocity = Vec2::ZERO;
    const VERTICAL_MULTIPLIER: f32 = 1.75;

    if keys.pressed(KeyCode::W) {
        velocity.y -= PAN_SPEED;
        velocity.x -= PAN_SPEED;
        velocity *= VERTICAL_MULTIPLIER;
    }
    if keys.pressed(KeyCode::S) {
        velocity.y += PAN_SPEED;
        velocity.x += PAN_SPEED;
        velocity *= VERTICAL_MULTIPLIER;
    }
    if keys.pressed(KeyCode::A) {
        velocity.x -= PAN_SPEED;
        velocity.y += PAN_SPEED;
    }
    if keys.pressed(KeyCode::D) {
        velocity.x += PAN_SPEED;
        velocity.y -= PAN_SPEED;
    }


    let velocity = rotate_velocity(velocity, &camera.heading);
    transform.translation += Vec3::new(velocity.x, 0.0, velocity.y) * 1.0;
}

fn rotate_velocity(velocity: Vec2, heading: &OrthographicCameraHeading) -> Vec2 {
    match heading {
        OrthographicCameraHeading::NorthWest => velocity,
        OrthographicCameraHeading::SouthWest => Vec2::new(velocity.y, -velocity.x),
        OrthographicCameraHeading::SouthEast => Vec2::new(-velocity.x, -velocity.y),
        OrthographicCameraHeading::NorthEast => Vec2::new(-velocity.y, velocity.x),
    }
}

fn control_camera_rotate(
    mut camera_query: Query<(&mut Transform, &mut PanOrbitCamera)>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut transform, mut camera) = camera_query.single_mut();

    // TODO: use current position to calculate looking_at
    // TODO: this doesn't work
    let center_offset = match camera.heading {
        OrthographicCameraHeading::NorthWest => Vec2::new(transform.translation.x - CAMERA_DISTANCE - WORLD_CENTER, transform.translation.z - CAMERA_DISTANCE - WORLD_CENTER),
        OrthographicCameraHeading::SouthWest => Vec2::new(transform.translation.x - CAMERA_DISTANCE - WORLD_CENTER, transform.translation.z + CAMERA_DISTANCE - WORLD_CENTER),
        OrthographicCameraHeading::SouthEast => Vec2::new(transform.translation.x + CAMERA_DISTANCE - WORLD_CENTER, transform.translation.z + CAMERA_DISTANCE - WORLD_CENTER),
        OrthographicCameraHeading::NorthEast => Vec2::new(transform.translation.x + CAMERA_DISTANCE - WORLD_CENTER, transform.translation.z - CAMERA_DISTANCE - WORLD_CENTER),
    };

    if keys.just_pressed(KeyCode::E) {
        match camera.heading {
            OrthographicCameraHeading::NorthWest => {
                camera.heading = OrthographicCameraHeading::SouthWest;
                *transform = Transform::from_xyz(
                    WORLD_CENTER + CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER - CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
            OrthographicCameraHeading::SouthWest => {
                camera.heading = OrthographicCameraHeading::SouthEast;
                *transform = Transform::from_xyz(
                    WORLD_CENTER - CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER - CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
            OrthographicCameraHeading::SouthEast => {
                camera.heading = OrthographicCameraHeading::NorthEast;
                *transform = Transform::from_xyz(
                    WORLD_CENTER - CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER + CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
            OrthographicCameraHeading::NorthEast => {
                camera.heading = OrthographicCameraHeading::NorthWest;
                *transform = Transform::from_xyz(
                    WORLD_CENTER + CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER + CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
        }
    }

    if keys.just_pressed(KeyCode::Q) {
        match camera.heading {
            OrthographicCameraHeading::SouthWest => {
                camera.heading = OrthographicCameraHeading::NorthWest;
                *transform = Transform::from_xyz(
                    WORLD_CENTER + CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER + CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
            OrthographicCameraHeading::SouthEast => {
                camera.heading = OrthographicCameraHeading::SouthWest;
                *transform = Transform::from_xyz(
                    WORLD_CENTER + CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER - CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
            OrthographicCameraHeading::NorthEast => {
                camera.heading = OrthographicCameraHeading::SouthEast;
                *transform = Transform::from_xyz(
                    WORLD_CENTER - CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER - CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
            OrthographicCameraHeading::NorthWest => {
                camera.heading = OrthographicCameraHeading::NorthEast;
                *transform = Transform::from_xyz(
                    WORLD_CENTER - CAMERA_DISTANCE,
                    CAMERA_HEIGHT,
                    WORLD_CENTER + CAMERA_DISTANCE,
                )
                .looking_at(
                    Vec3 {
                        x: WORLD_CENTER + center_offset.x,
                        y: WORLD_HEIGHT,
                        z: WORLD_CENTER + center_offset.y,
                    },
                    Vec3::Y,
                );
            }
        }
    }
}
