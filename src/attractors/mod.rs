use bevy::prelude::Vec3;

pub(crate) mod sprott;
pub(crate) mod lorenz;
pub(crate) mod aizawa;
pub(crate) mod chen;
pub(crate) mod thomas;
pub(crate) mod dadras;
pub(crate) mod three_scroll;
pub(crate) mod halvorsen;
pub(crate) mod four_wing;

/// Attractor
pub trait Attractor {
    /// Generates with param of last point
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32);

    /// Gets the start point of the attractor
    fn start_point(&self) -> Vec3;

    /// Generates a vec3 from last vec3
    fn gen_vec3(&self, vector: &Vec3) -> Vec3 {
        let result = self.generate(&vector.x, &vector.y, &vector.z);
        return Vec3::new(result.0, result.1, result.2);
    }
}
