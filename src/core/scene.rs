use super::geometry::sphere::Sphere;
use super::geometry::cube::Cube;
use super::light::Light;
use super::skybox::Skybox;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
    pub skybox: Option<Skybox>,
}