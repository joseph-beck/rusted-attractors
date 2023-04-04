use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
use crate::{
    AppState, 
    user_interface::main_menu::MainMenuPlugin,
    viewer::*,
    shapes::line,
    camera::orbit_camera_controls
};

mod setup;
mod transitions;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    window_level: bevy::window::WindowLevel::AlwaysOnTop,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_state::<AppState>()
        .add_plugin(MaterialPlugin::<line::Mat>::default())  
        //.add_plugin(NoCameraPlayerPlugin)
        .add_plugin(MainMenuPlugin)
        .insert_resource(setup::SETTINGS)
        .add_startup_system(setup::spawn_orbit_camera)
        .add_startup_system(viewer::draw_lines)
        .add_system(transitions::to_viewer)
        .add_system(transitions::to_main_menu)
        .add_system(orbit_camera_controls::pan_orbit_camera)
        .run();
}
