use bevy::prelude::Vec3;

use super::Attractor;

pub struct Halvorsen {
    pub a: f32,
    pub dt: f32,
}

impl Default for Halvorsen {
    fn default() -> Self {
        Halvorsen {
            a: 1.89,
            dt: 0.008
        }
    }
}

impl Attractor for Halvorsen {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = ((-1. * self.a * x) - (4. * y) - (4. * z) - (y * y)) * self.dt;
        let dy: f32 = ((-1. * self.a * y) - (4. * z) - (4. * x) - (z * z)) * self.dt;
        let dz: f32 = ((-1. * self.a * z) - (4. * x) - (4. * y) - (x * x)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}
