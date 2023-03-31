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

