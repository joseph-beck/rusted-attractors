use bevy::prelude::*;
use crate::attractors;
use crate::window::line;

pub fn draw_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<line::Mat>>,
) {
    let lines = generate_lines(100000);

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

fn generate_lines(amount: i32) -> Vec<(Vec3, Vec3)> {
    let mut lines = Vec::new();
    let mut current: Vec3 = attractors::thomas::start_point();

    for _ in 0..amount {
        let delta = attractors::thomas::gen_vec3(&current);
        let next = Vec3::new(
            current.x + delta.x,
            current.y + delta.y,
            current.z + delta.z
        );

        lines.push((current, next));
        current = next;
    }

    return lines;
}
