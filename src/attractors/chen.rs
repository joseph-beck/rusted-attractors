use bevy::prelude::Vec3;

use super::Attractor;

// doesn't work for some reason

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
            a: 5.,
            b: -10.,
            c: -0.38,
            dt: 0.001
        }
    }
}

impl Attractor for Chen {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = ((self.a * x) - (y * z)) * self.dt;
        let dy: f32 = ((self.b * y) + (x * z)) * self.dt;
        let dz: f32 = ((self.c * z) + ((x * y) / 3.)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0.01, 0.01)
    }
}
