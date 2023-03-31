use bevy::prelude::*;
use crate::lorenz;

pub fn generate_nodes(
    mut commands: Commands, 
    mut mesh_assets: ResMut<Assets<Mesh>>
) {
    let mut current = Vec3::new(0.01, 0., 0.);
    let sphere = mesh_assets.add(shape::UVSphere::default().into());
    
    for i in 0..50 {
        commands.spawn(PbrBundle {
            mesh: sphere.clone(),
            transform: Transform::from_translation(current),
            ..Default::default()
        });

        let delta = lorenz::gen_vec3(current);

        current.x += delta.x;
        current.y += delta.y;
        current.z += delta.z;
        println!("{}, {}", i, current)
    }
}