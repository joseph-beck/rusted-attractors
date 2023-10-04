use bevy::prelude::Vec3;

/// Attractor
pub trait Attractor {
    /// Generates with param of last point
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32);

    /// Gets the start point of the attractor
    fn start_point(&self) -> Vec3;

    /// Generates a vec3 from last vec3
    fn gen_vec3(&self, vector: &Vec3) -> Vec3 {
        let result = self.generate(&vector.x, &vector.y, &vector.z);
        return Vec3::new(result.0, result.1, result.2);
    }
}

/// Aizawa attractor
pub struct Aizawa {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub dt: f32,
}

impl Default for Aizawa {
    fn default() -> Self {
        Aizawa {
            a: 0.95,
            b: 0.7,
            c: 0.6,
            d: 3.5,
            e: 0.25,
            f: 0.1,
            dt: 0.001,
        }
    }
}

impl Attractor for Aizawa {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = ((z - self.b) * x - (self.d * y)) * self.dt;
        let dy: f32 = ((self.d * x) + ((z - self.b) * y)) * self.dt;
        let dz: f32 = (self.c + (self.a * z) - (z.powf(3.)) / 3.)
            - ((x.powf(2.)) + (y.powf(2.)))
                * ((1. + (self.e * z)) + (self.f * z * x.powf(3.)))
                * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 0., 0.)
    }
}

/// Chen attractor
pub struct Chen {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub dt: f32,
}

impl Default for Chen {
    fn default() -> Self {
        Chen {
            a: 40.,
            b: 28.,
            c: 3.,
            dt: 0.01,
        }
    }
}

impl Attractor for Chen {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = (self.a * (y - x)) * self.dt;
        let dy: f32 = ((self.c - self.a) * x - (x * z) + (self.c * y)) * self.dt;
        let dz: f32 = ((x * y) - (self.b * z)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(-0.1, 0.5, -0.6)
    }
}

/// Dadras attractor
pub struct Dadras {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub dt: f32,
}

impl Default for Dadras {
    fn default() -> Self {
        Dadras {
            a: 3.,
            b: 2.7,
            c: 1.7,
            d: 2.,
            e: 9.,
            dt: 0.001,
        }
    }
}

impl Attractor for Dadras {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = (y - self.a * x + self.b * y * z) * self.dt;
        let dy: f32 = (self.c * y - x * z + z) * self.dt;
        let dz: f32 = (self.d * x * y - self.e * z) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 0.03, 0.)
    }
}

pub struct FourWing {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub dt: f32,
}

impl Default for FourWing {
    fn default() -> Self {
        FourWing {
            a: 0.2,
            b: 0.01,
            c: -0.4,
            dt: 0.005,
        }
    }
}

impl Attractor for FourWing {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = ((self.a * x) + (y * z)) * self.dt;
        let dy: f32 = ((self.b * x) + (self.c * y) - (x * z)) * self.dt;
        let dz: f32 = (-z - (x * y)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}

pub struct Halvorsen {
    pub a: f32,
    pub dt: f32,
}

impl Default for Halvorsen {
    fn default() -> Self {
        Halvorsen { a: 1.89, dt: 0.008 }
    }
}

impl Attractor for Halvorsen {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = ((-1. * self.a * x) - (4. * y) - (4. * z) - (y * y)) * self.dt;
        let dy: f32 = ((-1. * self.a * y) - (4. * z) - (4. * x) - (z * z)) * self.dt;
        let dz: f32 = ((-1. * self.a * z) - (4. * x) - (4. * y) - (x * x)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}

/// Lorenz attractor
pub struct Lorenz {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub dt: f32,
}

impl Default for Lorenz {
    fn default() -> Self {
        Lorenz {
            a: 28.,
            b: 10.,
            c: 2.67,
            dt: 0.005,
        }
    }
}

impl Attractor for Lorenz {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = (self.b * (y - x)) * self.dt;
        let dy: f32 = (x * (self.a - z) - y) * self.dt;
        let dz: f32 = ((x * y) - (self.c * z)) * self.dt;

        return (dx, dy, dz);
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}

/// Sprott attractor
pub struct Sprott {
    pub a: f32,
    pub b: f32,
    pub dt: f32,
}

impl Default for Sprott {
    fn default() -> Self {
        Sprott {
            a: 2.07,
            b: 1.79,
            dt: 0.005,
        }
    }
}

impl Attractor for Sprott {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = (y + (self.a * x * y) + (x * z)) * self.dt;
        let dy: f32 = (1. - (self.b * x * x) + (y * z)) * self.dt;
        let dz: f32 = (x - (x * x) - (y * y)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}

/// Thomas attractor
pub struct Thomas {
    pub a: f32,
    pub dt: f32,
}

impl Default for Thomas {
    fn default() -> Self {
        Thomas {
            a: 0.208186,
            dt: 0.01,
        }
    }
}

impl Attractor for Thomas {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = (y.sin() - self.a * x) * self.dt;
        let dy: f32 = (z.sin() - self.a * y) * self.dt;
        let dz: f32 = (x.sin() - self.a * z) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 0.2, 0.3)
    }
}

/// Three Scroll attractor
pub struct ThreeScroll {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub dt: f32,
}

impl Default for ThreeScroll {
    fn default() -> Self {
        ThreeScroll {
            a: 40.,
            b: 0.833,
            c: 0.5,
            d: 0.65,
            e: 20.,
            dt: 0.001,
        }
    }
}

impl Attractor for ThreeScroll {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32) {
        let dx: f32 = (self.a * (y - x) + self.c * x * z) * self.dt;
        let dy: f32 = (self.e * y - x * z) * self.dt;
        let dz: f32 = (self.b * z + x * y - self.d * x * x) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 1., -0.1)
    }
}
