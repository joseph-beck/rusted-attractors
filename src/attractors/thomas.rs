use bevy::prelude::Vec3;

const B: f32 = 0.208186;
const DT: f32 = 0.01;

// something here is wrong

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = (y.sin() - B * x) * DT;
    let dy: f32 = (z.sin() - B * y) * DT;
    let dz: f32 = (x.sin() - B * z) * DT;

    return (dx, dy, dz);
}

pub fn start_point() -> Vec3 {
    return Vec3::new(0.1, 0.2, 0.3);
}

pub fn gen_vec3(vector: &Vec3) -> Vec3 {
    let result = generate(&vector.x, &vector.y, &vector.z);
    return Vec3::new(result.0, result.1, result.2);
}
