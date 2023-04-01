use bevy::prelude::Vec3;

const A: f32 = 28.; 
const B: f32 = 10.;
const C: f32 = 2.67;
const DT: f32 = 0.005;

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = (B * (y - x)) * DT;
    let dy: f32 = (x * (A - z) - y) * DT;
    let dz: f32 = ((x * y) - (C * z)) * DT;

    return (dx, dy, dz);
}

pub fn start_point() -> Vec3 {
    return Vec3::new(0.01, 0., 0.);
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}
