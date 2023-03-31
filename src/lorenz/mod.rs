use bevy::prelude::Vec3;

mod attractor;

pub fn run() {
    let mut x: f32 = 0.01;
    let mut y: f32 = 0.;
    let mut z: f32 = 0.;

    loop {
        let result = attractor::generate(&x, &y, &z);

        x += result.0;
        y += result.1;
        z += result.2;

        println!("{}, {}, {}", x, y, z);
    }
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = attractor::generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}