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
            a: 40.,
            b: 28.,
            c: 3.,
            dt: 0.01
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
        let dx: f32 = (self.a * (y - x)) * self.dt;
        let dy: f32 = ((self.c - self.a) * x - (x * z) + (self.c * y)) * self.dt;
        let dz: f32 = ((x * y) - (self.b * z)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(-0.1, 0.5, -0.6)
    }
}
