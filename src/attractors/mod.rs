use bevy::prelude::Vec3;

pub(crate) mod sprott;
pub(crate) mod lorenz;
pub(crate) mod aizawa;
pub(crate) mod chen;
pub(crate) mod thomas;
pub(crate) mod dadras;
pub(crate) mod three_scroll;

pub trait Attractor {
    fn generate(&self, x: &f32, y: &f32, z: &f32) -> (f32, f32, f32);

    fn start_point(&self) -> Vec3;

    fn gen_vec3(&self, vector: &Vec3) -> Vec3 {
        let result = self.generate(&vector.x, &vector.y, &vector.z);
        return Vec3::new(result.0, result.1, result.2);
    }
}
