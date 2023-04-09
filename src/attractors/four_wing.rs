use bevy::prelude::Vec3;

use super::Attractor;

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
            dt: 0.005
        }
    }
}

impl Attractor for FourWing {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = ((self.a * x) + (y * z)) * self.dt;
        let dy: f32 = ((self.b * x) + (self.c * y) - (x * z)) * self.dt;
        let dz: f32 = (-z - (x * y)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}
