use bevy::prelude::Vec3;

use super::Attractor;

/// Thomas attractor
pub struct Thomas {
    pub a: f32,
    pub dt: f32
}

impl Default for Thomas {
    fn default() -> Self {
        Thomas {
            a: 0.208186,
            dt: 0.01
        }
    }
}

impl Attractor for Thomas {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = (y.sin() - self.a * x) * self.dt;
        let dy: f32 = (z.sin() - self.a * y) * self.dt;
        let dz: f32 = (x.sin() - self.a * z) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 0.2, 0.3)
    }
}
