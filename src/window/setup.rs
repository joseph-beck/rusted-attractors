use bevy::prelude::*;
use bevy_flycam::{
    FlyCam, 
    MovementSettings
};
use crate::camera::orbit_camera::PanOrbitCamera;

pub const SETTINGS: MovementSettings = MovementSettings {
    sensitivity: 0.00015,
    speed: 10.,
};

pub fn spawn_flycam_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default() }, 
        FlyCam
    ));
}

pub fn spawn_orbit_camera(mut commands: Commands) {
    let translation = Vec3::new(-2., 2.5, 5.);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}
