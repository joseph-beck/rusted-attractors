use bevy::prelude::Vec3;

const A: f32 = 3.; 
const B: f32 = 2.7;
const C: f32 = 1.7;
const D: f32 = 2.;
const E: f32 = 9.;
const DT: f32 = 0.001;

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = (y - A * x + B * y * z) * DT;
    let dy: f32 = (C * y - x * z + z) * DT;
    let dz: f32 = (D * x * y - E * z) * DT;

    return (dx, dy, dz);
}

pub fn start_point() -> Vec3 {
    return Vec3::new(0.1, 0.03, 0.);
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}
