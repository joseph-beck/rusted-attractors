use bevy::prelude::*;
use bevy_flycam::{ NoCameraPlayerPlugin, FlyCam};

mod node;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_startup_system(setup_camera)
        .add_system(spawn_sprott_base)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}

fn spawn_sprott_base(
    mut commands: Commands, 
    mut mesh_assets: ResMut<Assets<Mesh>>
) {
    let sphere = mesh_assets.add(shape::UVSphere::default().into());

    commands.spawn(PbrBundle {
        mesh: sphere.clone(),
        transform: Transform::from_translation(Vec3::new(2., 0., 2.)),
        ..Default::default()
    });
}