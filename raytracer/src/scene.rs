use crate::lights::LightObject;
use crate::primitives::{Collision, Ray, Vec3};
use crate::textures::{Color, Texture};

pub struct Scene {
    pub camera: Box<dyn RayEmitter>,
    pub lights: Vec<Box<dyn LightObject>>,
    pub objects: Vec<Box<dyn AnySceneObject>>,
}

pub trait AnySceneObject {
    fn color_at(&self, point: Vec3) -> Color;
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Option<Vec3>;
}

pub struct SceneObject<T: Texture, P: Collision> {
    pub texture: T,
    pub primitive: P,
}

impl<T: Texture, P: Collision> AnySceneObject for SceneObject<T, P> {
    fn color_at(&self, point: Vec3) -> Color {
        let (u, v) = self.primitive.surface_mapping_at(point).unwrap();
        self.texture.color_at(u, v)
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
