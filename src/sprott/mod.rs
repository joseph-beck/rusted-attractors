mod sprott;

pub fn generate(x: &f64, y: &f64, z: &f64) -> Vec<f64> {
    return sprott::sprott(&x, &y, &z);
}

pub fn run() {
    let mut x: f64 = 0.01;
    let mut y: f64 = 0f64;
    let mut z: f64 = 0f64;

    loop {
        let result = generate(&x, &y, &z);

        x += result[0];
        y += result[1];
        z += result[2];

        println!("{}, {}, {}", x, y, z);
    }
}