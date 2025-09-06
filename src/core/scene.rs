use super::geometry::sphere::Sphere;
use super::geometry::aabb::Aabb;
use super::light::Light;
use super::skybox::Skybox;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub aabbs: Vec<Aabb>,
    pub lights: Vec<Light>,
    pub skybox: Option<Skybox>,
}