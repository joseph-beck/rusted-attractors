use bevy::prelude::Vec3;

const A: f32 = 40.;
const B: f32 = 0.833;
const C: f32 = 0.5;
const D: f32 = 0.65;
const E: f32 = 20.;
const DT: f32 = 0.001;

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = (A * (y - x) + C * x * z) * DT;
    let dy: f32 = (E * y - x * z) * DT;
    let dz: f32 = (B * z + x * y - D * x * x) * DT;

    return (dx, dy, dz);
}

pub fn start_point() -> Vec3 {
    return Vec3::new(0.1, 1., -0.1);
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}
