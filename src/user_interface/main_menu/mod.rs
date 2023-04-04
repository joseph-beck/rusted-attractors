use bevy::prelude::*;

use crate::{
    AppState
};

mod interactions;
mod layout;
mod style;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(layout::spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(layout::despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}