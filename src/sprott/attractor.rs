const A: f32 = 2.07;
const B: f32 = 1.79;
const DT: f32 = 0.01;

pub fn generate(x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
    let dx: f32 = (y + (A * x * y) + (x * z)) * DT;
    let dy: f32 = (1.0 - (B * x * x) + (y * z)) * DT;
    let dz: f32 = (x - (x * x) - (y * y)) * DT;

    return (dx, dy, dz);
}