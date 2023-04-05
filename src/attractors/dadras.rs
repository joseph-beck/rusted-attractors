use bevy::prelude::Vec3;

use super::Attractor;

pub struct Dadras {
    pub a: f32, 
    pub b: f32, 
    pub c: f32, 
    pub d: f32, 
    pub e: f32, 
    pub dt: f32
}

impl Default for Dadras {
    fn default() -> Self {
        Dadras {
            a: 3.,
            b: 2.7,
            c: 1.7,
            d: 2.,
            e: 9.,
            dt: 0.001
        }
    }
}

impl Attractor for Dadras {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = (y - self.a * x + self.b * y * z) * self.dt;
        let dy: f32 = (self.c * y - x * z + z) * self.dt;
        let dz: f32 = (self.d * x * y - self.e * z) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 0.03, 0.)
    }
}
