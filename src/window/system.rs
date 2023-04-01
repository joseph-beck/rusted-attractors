use bevy::prelude::*;
use crate::attractors;
use crate::window::line;

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
        let delta = attractors::sprott::gen_vec3(&current);
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