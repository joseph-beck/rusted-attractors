use bevy::prelude::Vec3;

use super::Attractor;

pub struct Lorenz {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub dt: f32
}

impl Default for Lorenz {
    fn default() -> Self {
        Lorenz {
            a: 28.,
            b: 10.,
            c: 2.67,
            dt: 0.005
        }
    }
}

impl Attractor for Lorenz {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = (self.b * (y - x)) * self.dt;
        let dy: f32 = (x * (self.a - z) - y) * self.dt;
        let dz: f32 = ((x * y) - (self.c * z)) * self.dt;

        return (dx, dy, dz);
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}