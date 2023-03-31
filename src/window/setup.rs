use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings};

pub const SETTINGS: MovementSettings = MovementSettings {
    sensitivity: 0.00015,
    speed: 100.,
};

pub fn camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}