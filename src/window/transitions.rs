use bevy::prelude::*;
use crate::AppState;

pub fn to_viewer(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Viewer {
            next_app_state.set(AppState::Viewer);
        }
    }
}

pub fn to_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
        }
    }
}