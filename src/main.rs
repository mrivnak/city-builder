use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use plugins::WorldGenPlugin;

mod components;
mod plugins;
mod tiles;
mod worldgen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "City Builder".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldGenPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_light)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-200., 200., 200.).looking_at(
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Vec3::Y,
        ),
        ..default()
    });
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 100., 0.).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
