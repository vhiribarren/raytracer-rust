/*
MIT License

Copyright (c) 2019, 2020 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::vector::{Mat3, Vec3};
use crate::UnitInterval;
use std::f64::consts::PI;
use std::fmt::Debug;

pub trait Shape: Sync + Send {
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Option<Vec3>;
    fn surface_mapping_at(&self, point: Vec3) -> Option<(UnitInterval, UnitInterval)>;
}

#[derive(Debug)]
pub struct Ray {
    pub source: Vec3,
    /// The direction is normalized
    pub direction: Vec3,
    _use_constructor: (),
}

impl Ray {
    pub fn new(source: Vec3, direction: Vec3) -> Ray {
        Ray {
            source,
            direction: direction.normalize(),
            _use_constructor: (),
        }
    }

    pub fn ray_from_to(source: Vec3, destination: Vec3) -> Ray {
        Ray {
            source,
            direction: (destination - source).normalize(),
            _use_constructor: (),
        }
    }

    pub fn shift_source(&self) -> Ray {
        Ray {
            source: self.source + 1e-12 * self.direction,
            direction: self.direction,
            _use_constructor: (),
        }
    }
}

#[derive(Debug)]
pub struct InfinitePlan {
    center: Vec3,
    normal_normalized: Vec3,
    u_vec: Vec3,
    v_vec: Vec3,
    uv_mapping_width: f64,
}

impl InfinitePlan {
    pub fn new(center: Vec3, normal: Vec3) -> Self {
        let transform = Mat3::transformation_between(Vec3::new(0.0, 1.0, 0.0), normal);
        InfinitePlan {
            center,
            normal_normalized: normal.normalize(),
            u_vec: transform * Vec3::new(1.0, 0.0, 0.0),
            v_vec: transform * Vec3::new(0.0, 0.0, 1.0),
            uv_mapping_width: 50.0,
        }
    }
}

impl Shape for InfinitePlan {
    fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        let denom = self.normal_normalized.dot_product(ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }
        let p_l = self.center - ray.source;
        let t = p_l.dot_product(self.normal_normalized) / denom;
        if t > 0.0 {
            Some(ray.source + t * ray.direction)
        } else {
            None
        }
    }

    fn normal_at(&self, _point: Vec3) -> Option<Vec3> {
        Some(self.normal_normalized)
    }

    fn surface_mapping_at(&self, point: Vec3) -> Option<(UnitInterval, UnitInterval)> {
        let positive_space = |x| if x >= 0.0 { x } else { 1.0 + x };
        let plane_coords = Vec3::between_points(self.center, point);
        let u =
            (plane_coords.dot_product(self.u_vec) % self.uv_mapping_width) / self.uv_mapping_width;
        let u = positive_space(u);
        let v =
            (plane_coords.dot_product(self.v_vec) % self.uv_mapping_width) / self.uv_mapping_width;
        let v = positive_space(v);
        Some((u, v))
    }
}

#[derive(Debug)]
pub struct SquarePlan {
    center: Vec3,
    normal_normalized: Vec3,
    u_vec: Vec3,
    v_vec: Vec3,
    width: f64,
}

impl SquarePlan {
    pub fn new(center: Vec3, normal: Vec3, width: f64) -> Self {
        let transform = Mat3::transformation_between(Vec3::new(0.0, 1.0, 0.0), normal);
        SquarePlan {
            center,
            normal_normalized: normal.normalize(),
            u_vec: transform * Vec3::new(1.0, 0.0, 0.0),
            v_vec: transform * Vec3::new(0.0, 0.0, 1.0),
            width,
        }
    }

    fn to_plan_coords(&self, point: Vec3) -> Option<(f64, f64)> {
        // TODO return None if point not in plan
        let local_coords = Vec3::between_points(self.center, point);
        let local_x = local_coords.dot_product(self.u_vec);
        let local_y = local_coords.dot_product(self.v_vec);
        let radius = self.width / 2.0;
        if local_x < -radius || local_x > radius || local_y < -radius || local_y > radius {
            None
        } else {
            Some((local_x, local_y))
        }
    }
}

impl Shape for SquarePlan {
    fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        let denom = self
            .normal_normalized
            .dot_product(ray.direction.normalize());
        if denom.abs() < 1e-6 {
            return None;
        }
        let p_l = self.center - ray.source;
        let t = p_l.dot_product(self.normal_normalized) / denom;
        if t > 0.0 {
            let collision_point = ray.source + t * ray.direction.normalize();
            if self.to_plan_coords(collision_point).is_some() {
                return Some(collision_point);
            }
        }
        None
    }

    fn normal_at(&self, _point: Vec3) -> Option<Vec3> {
        Some(self.normal_normalized)
    }

    fn surface_mapping_at(&self, point: Vec3) -> Option<(UnitInterval, UnitInterval)> {
        let (local_x, local_y) = self.to_plan_coords(point).unwrap();
        let radius = self.width / 2.0;
        let u = (local_x + radius) / self.width;
        let v = (local_y + radius) / self.width;
        Some((u, v))
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        }
    }
}

impl Shape for Sphere {
    #[allow(non_snake_case)]
    fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        // http://mathinfo.univ-reims.fr/image/siRendu/Documents/2004-Chap6-RayTracing.pdf
        let r_square = self.radius.powi(2);
        let u = ray.direction.normalize();
        let C = self.center;
        let A = ray.source;
        let L = C - A;
        let d = L.dot_product(u);
        let l_square = L.dot_product(L);
        if d < 0.0 && l_square > r_square {
            return None;
        }
        let m_square = l_square - d.powi(2);
        if m_square > r_square {
            return None;
        }
        let q = (r_square - m_square).sqrt();
        let t: f64 = if l_square > r_square { d - q } else { d + q };
        Some(A + t * u)
    }

    fn normal_at(&self, point: Vec3) -> Option<Vec3> {
        Some(Vec3::between_points(self.center, point).normalize())
    }

    fn surface_mapping_at(&self, point: Vec3) -> Option<(UnitInterval, UnitInterval)> {
        let unit_point = Vec3::between_points(self.center, point).normalize();
        let u = 0.5 + unit_point.z.atan2(unit_point.x) / (2.0 * PI);
        let v = 0.5 - unit_point.y.asin() / PI;
        Some((u, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::f64_eq;

    #[test]
    fn from_to_ray_has_normalized_direction() {
        let ray = Ray::ray_from_to(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
        assert!(f64_eq(ray.direction.norm(), 1.0));
    }

    #[test]
    fn new_ray_has_normalized_direction() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
        assert!(f64_eq(ray.direction.norm(), 1.0));
    }

    #[test]
    fn unit_sphere_values() {
        let sphere: Sphere = Default::default(); // Given a unit sphere
        assert!(f64_eq(sphere.radius, 1.0)); // Then it has a radius of 1
        assert!(f64_eq(sphere.center.x, 0.0)); // And a x value of 0
        assert!(f64_eq(sphere.center.y, 0.0)); // And a y value of 0
        assert!(f64_eq(sphere.center.z, 0.0)); // And a z value of 0
    }

    #[test]
    fn ray_sphere_collision() {
        // Given a unit sphere
        let sphere: Sphere = Default::default();
        // If we launch a ray in front of it
        let ray: Ray = Ray::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );
        let result = sphere.check_collision(&ray);
        // There is a collision
        assert!(result.is_some());
    }

    #[test]
    fn ray_sphere_no_collision() {
        let sphere: Sphere = Default::default(); // Given a unit sphere
        let ray: Ray = Ray::new(
            // If we launch a ray next to it and orthogonally
            Vec3 {
                x: 2.0,
                y: 0.0,
                z: -2.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );
        assert!(sphere.check_collision(&ray).is_none()); // There is no collision
    }
}
