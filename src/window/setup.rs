use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings};

pub const SETTINGS: MovementSettings = MovementSettings {
    sensitivity: 0.00015,
    speed: 10.,
};

pub fn camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default() }, 
        FlyCam
    ));
}