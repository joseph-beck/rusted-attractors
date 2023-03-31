const A: f64 = 28.0; 
const B: f64 = 10.0;
const C: f64 = 2.67;
const DT: f64 = 0.01;

pub fn lorenz(x: &f64, y: &f64, z: &f64) -> Vec<f64> {
    let dx: f64 = (B * (y - x)) * DT;
    let dy: f64 = (x * (A - z) - y) * DT;
    let dz: f64 = ((x * y) - (C * z)) * DT;

    return vec![dx, dy, dz];
}

