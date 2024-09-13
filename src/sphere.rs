use crate::ray_intersect::RayIntersect;
use nalgebra_glm::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool {
        return true;
    }
}
