use bevy::prelude::*;
use crate::{ 
    attractors, 
    shapes, shapes::*
};

pub fn draw_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<shapes::line::Mat>>,
) {
    let lines = generate_lines(100000);

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(line::List {
            lines: lines,
        })),
        transform: Transform::from_xyz(-1.5, 0.0, 0.0),
        material: materials.add(line::Mat {
            color: Color::ORANGE_RED,
        }),
        ..default()
    });
}

fn generate_lines(amount: i32) -> Vec<(Vec3, Vec3)> {
    let mut lines = Vec::new();
    let mut current: Vec3 = attractors::three_scroll::start_point();

    for _ in 0..amount {
        let delta = attractors::three_scroll::gen_vec3(&current);
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