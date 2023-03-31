use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;

mod setup;
mod system;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(setup::SETTINGS)
        .add_startup_system(setup::camera)
        .add_system(system::generate_nodes)
        .run();
}

