use bevy::prelude::*;
use crate::{lorenz, sprott};
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

        let delta = lorenz::gen_vec3(&current);

        current.x += delta.x;
        current.y += delta.y;
        current.z += delta.z;
        println!("{}, {}", i, current)
    }
}

pub fn draw_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<line::Mat>>,
) {
    let lines = generate_lines();

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(line::List {
            lines: lines,
        })),
        transform: Transform::from_xyz(-1.5, 0.0, 0.0),
        material: materials.add(line::Mat {
            color: Color::GREEN,
        }),
        ..default()
    });
}

fn generate_lines() -> Vec<(Vec3, Vec3)> {
    let mut lines = Vec::new();
    let mut current: Vec3 = Vec3::new(0.01, 0., 0.);

    for _ in 0..10000 {
        let delta = sprott::gen_vec3(&current);
        let next = add_vec(&current, &delta);

        lines.push((current, next));
        current = next;
    }

    return lines;
}

fn add_vec(a: &Vec3, b: &Vec3) -> Vec3 {
    return Vec3::new(
        a.x + b.x,
        a.y + b.y,
        a.z + b.z
    );
}