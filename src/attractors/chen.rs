use bevy::prelude::Vec3;

const A: f32 = 5.; 
const B: f32 = -10.;
const C: f32 = -0.38;
const DT: f32 = 0.01;

// doesn't work for some reason

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = ((A * x) - (y * z)) * DT;
    let dy: f32 = ((B * y) + (x * z)) * DT;
    let dz: f32 = ((C * z) + ((x * y) / 3.)) * DT;

    return (dx, dy, dz);
}

pub fn start_point() -> Vec3 {
    return Vec3::new(0.01, 0.01, 0.01);
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}
