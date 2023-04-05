use bevy::{
    prelude::*,
    app::AppExit
};
use crate::{
    user_interface::{
        button::components::{
            ViewButton,
            QuitButton
        },
        button::styles::{
            NORMAL_BUTTON_COLOR,
            HOVERED_BUTTON_COLOR,
            CLICKED_BUTTON_COLOR
        }
    },
    AppState
};

pub fn with_view_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<ViewButton>)
    >,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Viewer);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<QuitButton>)
    >,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
