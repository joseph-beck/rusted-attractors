use bevy::prelude::Vec3;

use super::Attractor;

pub struct Sprott {
    pub a: f32,
    pub b: f32,
    pub dt: f32
}

impl Default for Sprott {
    fn default() -> Self {
        Sprott { 
            a: 2.07, 
            b: 1.79, 
            dt: 0.005  
        }
    }
}

impl Attractor for Sprott {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = (y + (self.a * x * y) + (x * z)) * self.dt;
        let dy: f32 = (1. - (self.b * x * x) + (y * z)) * self.dt;
        let dz: f32 = (x - (x * x) - (y * y)) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.01, 0., 0.)
    }
}
