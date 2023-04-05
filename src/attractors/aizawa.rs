use bevy::prelude::Vec3;

use super::Attractor;

/// Aizawa attractor
pub struct Aizawa {
    pub a: f32,    
    pub b: f32,    
    pub c: f32,    
    pub d: f32,    
    pub e: f32,    
    pub f: f32,    
    pub dt: f32        
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
            dt: 0.001
        }
    }
}

impl Attractor for Aizawa {
    fn generate(
        &self, 
        x: &f32, 
        y: &f32, 
        z: &f32
    ) -> (f32, f32, f32) {
        let dx: f32 = ((z - self.b) * x - (self.d * y)) * self.dt;
        let dy: f32 = ((self.d * x) + ((z - self.b) * y)) * self.dt;
        let dz: f32 = (self.c + (self.a * z) - (z.powf(3.)) / 3.) - ((x.powf(2.)) + (y.powf(2.))) * ((1. + (self.e * z)) + (self.f * z * x.powf(3.))) * self.dt;

        (dx, dy, dz)
    }

    fn start_point(&self) -> Vec3 {
        Vec3::new(0.1, 0., 0.)
    }
}
