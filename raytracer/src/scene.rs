use crate::primitives::{Collision, Ray, Vec3};
use crate::textures::Texture;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn SceneObject>>,
}

pub trait SceneObject {
    fn texture(&self) -> &Texture;
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
}

pub struct SceneObjectStruct<P: Collision> {
    pub texture: Texture,
    pub primitive: P,
}

impl<P: Collision> SceneObject for SceneObjectStruct<P> {
    fn texture(&self) -> &Texture {
        &self.texture
    }

    fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        self.primitive.check_collision(ray)
    }
}

#[derive(Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub screen_center: Vec3,
    pub up: Vec3,
    pub width: f64,
    pub height: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            eye: Vec3::new(0.0, 0.0, -15.0),
            screen_center: Vec3::new(0.0, 0.0, -10.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            width: 16.0,
            height: 9.0,
        }
    }
}
