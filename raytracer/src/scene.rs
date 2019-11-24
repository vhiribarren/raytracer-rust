use crate::lights::LightObject;
use crate::primitives::{Collision, Ray, Vec3};
use crate::textures::Texture;

pub struct Scene {
    pub camera: Box<dyn RayEmitter>,
    pub lights: Vec<Box<dyn LightObject>>,
    pub objects: Vec<Box<dyn SceneObject>>,
}

pub trait SceneObject {
    fn texture(&self) -> &Texture;
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Option<Vec3>;
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

    fn normal_at(&self, point: Vec3) -> Option<Vec3> {
        self.primitive.normal_at(point)
    }
}

pub trait RayEmitter {
    fn generate_rays<'a>(
        &'a self,
        screen_width: u32,
        screen_height: u32,
    ) -> Box<dyn Iterator<Item = (u32, u32, Ray)> + 'a>;
}
