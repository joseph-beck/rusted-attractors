use bevy::prelude::{
    *
};
use crate::{
    user_interface::components::MainMenu,
    user_interface::main_menu::style::{
        NORMAL_BUTTON_COLOR,
        BUTTON_STYLE,
        MAIN_MENU_STYLE,
        get_text_style
    },
    user_interface::components::ViewButton,
    user_interface::components::QuitButton
};

pub fn spawn_main_menu(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let main_menu_ent = build_main_menu(&mut commands, &assets);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_ent) = main_menu_query.get_single() {
        commands.entity(main_menu_ent).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    assets: &Res<AssetServer>
) -> Entity {
    let main_menu_ent = commands.spawn((
        NodeBundle {
            style: MAIN_MENU_STYLE,
            //background_color: Color::RED.into(),
            ..default()
        },
        MainMenu { },
    )).with_children(|parent| {
        // title

        // view button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            ViewButton { },
        )).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "View",
                                get_text_style(&assets)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        // quit button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton { },
        )).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                get_text_style(&assets)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
    })
    .id();

    main_menu_ent
}
