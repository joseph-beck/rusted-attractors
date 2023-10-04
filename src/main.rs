use bevy::prelude::*;

mod attractors;
mod camera;
mod shapes;
mod ui;
mod window;

fn main() {
    window::run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Viewer,
}
