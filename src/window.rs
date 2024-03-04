#![allow(dead_code)]

use crate::{
    attractors::{self, Attractor},
    camera::{self, MovementSettings, PanOrbitCamera},
    shapes,
    ui::MainMenuPlugin,
    AppState,
};
use bevy::prelude::*;

pub fn spawn_orbit_camera(mut commands: Commands) {
    let translation = Vec3::new(-5., 7., 7.);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}

pub fn to_viewer(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.0 != AppState::Viewer {
            next_app_state.set(AppState::Viewer);
        }
    }
}

pub fn to_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
        }
    }
}

pub fn draw_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<shapes::Mat>>,
) {
    let lines = generate_lines(1_000_000);

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shapes::List { lines: lines })),
        transform: Transform::from_xyz(-1.5, 0.0, 0.0),
        material: materials.add(shapes::Mat {
            color: Color::ORANGE_RED,
        }),
        ..default()
    });
}

fn generate_lines(amount: i32) -> Vec<(Vec3, Vec3)> {
    let attractor = attractors::Rossler {
        ..Default::default()
    };
    let mut lines = Vec::new();
    let mut current: Vec3 = attractor.start_point();

    for _ in 0..amount {
        let delta = attractor.gen_vec3(&current);
        let next = Vec3::new(
            current.x + delta.x,
            current.y + delta.y,
            current.z + delta.z,
        );

        lines.push((current, next));
        current = next;
    }

    lines
}

pub fn run() {
    let settings: MovementSettings = MovementSettings {
        ..Default::default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugin(MaterialPlugin::<shapes::Mat>::default())
        //.add_plugin(NoCameraPlayerPlugin) // when using fly cam.
        .add_plugin(MainMenuPlugin)
        .insert_resource(settings)
        .add_startup_system(spawn_orbit_camera)
        .add_startup_system(draw_lines)
        .add_system(to_viewer)
        .add_system(to_main_menu)
        .add_system(camera::pan_orbit_camera)
        .run();
}
