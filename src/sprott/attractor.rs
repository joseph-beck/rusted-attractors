const A: f64 = 2.07;
const B: f64 = 1.79;
const DT: f64 = 0.01;

pub fn generate(x: &f64, y: &f64, z: &f64) -> (f64, f64, f64) {
    let dx: f64 = (y + (A * x * y) + (x * z)) * DT;
    let dy: f64 = (1.0 - (B * x * x) + (y * z)) * DT;
    let dz: f64 = (x - (x * x) - (y * y)) * DT;

    return (dx, dy, dz);
}