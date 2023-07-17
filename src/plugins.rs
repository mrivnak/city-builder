use bevy::{prelude::*, input::mouse::MouseWheel};

use crate::{worldgen, components::PanOrbitCamera};

// Square world
const WORLD_SIZE: u32 = 128;

// Control limits
const MIN_SCALE: f32 = 5.0;
const MAX_SCALE: f32 = 25.0;

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
        app.add_systems(Update, control_camera_zoom);
    }
}

fn control_camera_zoom(mut camera_query: Query<&mut Projection, With<PanOrbitCamera>>, mut scroll_evr: EventReader<MouseWheel>) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut camera = camera_query.single_mut();

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                if let Projection::Orthographic(ref mut p) = *camera {
                    let new = p.scale - ev.y;
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
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}

fn control_camera_pan(mut camera_query: Query<&mut Transform, With<PanOrbitCamera>>) {
    
}
