use crate::primitives::Vec3;

pub trait LightObject {}

pub struct LightPoint {
    pub source: Vec3,
}

impl LightObject for LightPoint {}
