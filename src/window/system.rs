use bevy::prelude::*;
use crate::lorenz;
use crate::window::line;

pub fn generate_nodes(
    mut commands: Commands, 
    mut mesh_assets: ResMut<Assets<Mesh>>
) {
    let mut current = Vec3::new(0.1, 0., 0.);
    let sphere = mesh_assets.add(shape::UVSphere::default().into());
    
    for i in 0..50 {
        commands.spawn(PbrBundle {
            mesh: sphere.clone(),
            transform: Transform::from_translation(Vec3::new(current.x * 150., current.y * 150., current.z * 150.)),
            ..Default::default()
        });

        let delta = lorenz::gen_vec3(current);

        current.x += delta.x;
        current.y += delta.y;
        current.z += delta.z;
        println!("{}, {}", i, current)
    }
}

pub fn generate_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<line::Mat>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(line::List {
            lines: vec![
                (Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            ],
        })),
        transform: Transform::from_xyz(-1.5, 0.0, 0.0),
        material: materials.add(line::Mat {
            color: Color::GREEN,
        }),
        ..default()
    });

    // Spawn a line strip that goes from point to point
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(line::Strip {
            points: vec![
                Vec3::ZERO,
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ],
        })),
        transform: Transform::from_xyz(0.5, 0.0, 0.0),
        material: materials.add(line::Mat { color: Color::BLUE }),
        ..default()
    });

    println!("Lines made");
}