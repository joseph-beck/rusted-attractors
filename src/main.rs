use bevy::prelude::*;

mod attractors;
mod shapes;
mod window;
mod user_interface;
mod viewer;

fn main() {
    window::run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Viewer,
}
