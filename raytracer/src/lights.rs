use crate::vector::Vec3;

pub trait LightObject {
    fn source(&self) -> Vec3;
}

pub struct LightPoint {
    pub source: Vec3,
}

impl LightObject for LightPoint {
    fn source(&self) -> Vec3 {
        self.source
    }
}

pub struct AmbientLight {
    pub power: f64,
}
