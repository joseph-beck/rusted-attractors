use bevy::prelude::Vec3;

use super::Attractor;

pub struct ThreeScroll {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32, 
    pub dt: f32
}

impl Default for ThreeScroll {
    fn default() -> Self {
        ThreeScroll {
            a: 40.,
            b: 0.833,
            c: 0.5,
            d: 0.65,
            e: 20.,
            dt: 0.001
        }
    }
}

impl Attractor for ThreeScroll {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32,
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = (self.a * (y - x) + self.c * x * z) * self.dt;
        let dy: f32 = (self.e * y - x * z) * self.dt;
        let dz: f32 = (self.b * z + x * y - self.d * x * x) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 1., -0.1)
    }
}
