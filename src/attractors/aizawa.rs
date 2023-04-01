use bevy::prelude::Vec3;

const A: f32 = 0.95; 
const B: f32 = 0.7;
const C: f32 = 0.6;
const D: f32 = 3.5;
const E: f32 = 0.25;
const F: f32 = 0.1;
const DT: f32 = 0.001;

// something here is wrong

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = ((z - B) * x - (D * y)) * DT;
    let dy: f32 = ((D * x) + ((z - B) * y)) * DT;
    let dz: f32 = (C + (A * z) - (z.powf(3.)) / 3.) - ((x.powf(2.)) + (y.powf(2.))) * ((1. + (E * z)) + (F * z * x.powf(3.))) * DT;

    return (dx, dy, dz);
}

pub fn start_point() -> Vec3 {
    return Vec3::new(0.1, 0., 0.);
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}