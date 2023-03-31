use bevy::prelude::*;
use bevy_flycam::{ NoCameraPlayerPlugin, FlyCam};

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
    let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.).into());

    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * 2., 0., z as f32 * 2.)),
                ..Default::default()
            });
        }
    }
}