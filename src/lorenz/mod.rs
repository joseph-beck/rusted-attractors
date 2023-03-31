mod attractor;

pub fn run() {
    let mut x: f64 = 0.01;
    let mut y: f64 = 0.;
    let mut z: f64 = 0.;

    loop {
        let result = attractor::generate(&x, &y, &z);

        x += result.0;
        y += result.1;
        z += result.2;

        println!("{}, {}, {}", x, y, z);
    }
}