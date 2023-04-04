use bevy::prelude::*;
use bevy_flycam::{
    FlyCam, 
    MovementSettings
};

pub const SETTINGS: MovementSettings = MovementSettings {
    sensitivity: 0.00015,
    speed: 10.,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-20., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default() }, 
        FlyCam
    ));
}
