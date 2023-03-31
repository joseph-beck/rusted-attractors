use bevy::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;

mod setup;
mod system;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(setup::SETTINGS)
        .add_startup_system(setup::camera)
        .add_system(system::generate_nodes)
        .run();
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