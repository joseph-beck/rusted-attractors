use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;

mod setup;
mod system;
mod line;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<line::Mat>::default())  
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(setup::SETTINGS)
        .add_startup_system(system::draw_lines)
        .add_startup_system(setup::camera)
        .run();
}
