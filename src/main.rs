use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use components::{OrthographicCameraHeading, PanOrbitCamera};
use consts::*;
use plugins::{CameraControlPlugin, WorldGenPlugin};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod consts;
mod plugins;
mod resources;
mod worldgen;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    .add_plugins(WorldGenPlugin)
    .add_plugins(CameraControlPlugin)
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_light);

    #[cfg(feature = "inspector")]
    {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 20.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(
                WORLD_CENTER + CAMERA_DISTANCE,
                CAMERA_HEIGHT,
                WORLD_CENTER + CAMERA_DISTANCE,
            )
            .looking_at(
                Vec3 {
                    x: WORLD_CENTER,
                    y: WORLD_HEIGHT,
                    z: WORLD_CENTER,
                },
                Vec3::Y,
            ),
            ..default()
        },
        PanOrbitCamera {
            heading: OrthographicCameraHeading::NorthWest,
        },
    ));
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
