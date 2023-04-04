use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
use crate::{
    AppState, 
    user_interface::main_menu::MainMenuPlugin,
    viewer::*,
    shapes::line,
};

mod setup;
mod transitions;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MaterialPlugin::<line::Mat>::default())  
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(MainMenuPlugin)
        .insert_resource(setup::SETTINGS)
        .add_startup_system(setup::camera)
        .add_startup_system(viewer::draw_lines)
        .add_system(transitions::to_viewer)
        .add_system(transitions::to_main_menu)
        .run();
}
