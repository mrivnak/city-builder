use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    components::{OrthographicCameraHeading, PanOrbitCamera},
    worldgen,
};

// Square world
const WORLD_SIZE: u32 = 128;

// Control limits
const MIN_SCALE: f32 = 5.0;
const MAX_SCALE: f32 = 25.0;
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
            .add_systems(Update, control_camera_pan);
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

    if keys.pressed(KeyCode::W) {
        velocity.y -= PAN_SPEED;
        velocity.x -= PAN_SPEED;
    }
    if keys.pressed(KeyCode::S) {
        velocity.y += PAN_SPEED;
        velocity.x += PAN_SPEED;
    }
    if keys.pressed(KeyCode::A) {
        velocity.x -= PAN_SPEED;
        velocity.y += PAN_SPEED;
    }
    if keys.pressed(KeyCode::D) {
        velocity.x += PAN_SPEED;
        velocity.y -= PAN_SPEED;
    }

    let velocity = rotate(velocity, &camera.heading).normalize_or_zero();
    transform.translation += Vec3::new(velocity.x, 0.0, velocity.y) * 1.0;
}

fn rotate(velocity: Vec2, heading: &OrthographicCameraHeading) -> Vec2 {
    match heading {
        OrthographicCameraHeading::NorthWest => velocity,
        _ => todo!("Implement camera rotation")
    }
}
