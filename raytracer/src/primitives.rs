use crate::vector::Vec3;
use std::f64::consts::PI;

pub trait Collision {
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Option<Vec3>;
    fn surface_mapping_at(&self, point: Vec3) -> Option<(f64, f64)>;
}

#[derive(Debug)]
pub struct Ray {
    pub source: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn ray_from_to(source: Vec3, destination: Vec3) -> Ray {
        Ray {
            source,
            direction: destination - source,
        }
    }
}

#[derive(Debug)]
pub struct Plane {
    center: Vec3,
    normal_normalized: Vec3,
}

impl Plane {
    pub fn new(center: Vec3, normal: Vec3) -> Self {
        Plane {
            center,
            normal_normalized: normal.normalize(),
        }
    }
}

impl Collision for Plane {
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
            Some(ray.source + t * ray.direction)
        } else {
            None
        }
    }

    fn normal_at(&self, _point: Vec3) -> Option<Vec3> {
        Some(self.normal_normalized)
    }

    fn surface_mapping_at(&self, point: Vec3) -> Option<(f64, f64)> {
        unimplemented!()
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

impl Collision for Sphere {
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

    fn surface_mapping_at(&self, point: Vec3) -> Option<(f64, f64)> {
        let unit_point = Vec3::between_points(self.center, point).normalize();
        let u = 0.5 + unit_point.z.atan2(unit_point.x) / (2.0 * PI);
        let v = 0.5 - unit_point.y.asin() / PI;
        Some((u, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn unit_sphere_values() {
        let sphere: Sphere = Default::default(); // Given a unit sphere
        assert!((sphere.radius - 1.0).abs() < EPSILON); // Then it has a radius of 1
        assert!((sphere.center.x - 0.0).abs() < EPSILON); // And a x value of 0
        assert!((sphere.center.y - 0.0).abs() < EPSILON); // And a y value of 0
        assert!((sphere.center.z - 0.0).abs() < EPSILON); // And a z value of 0
    }

    #[test]
    fn ray_sphere_collision() {
        // Given a unit sphere
        let sphere: Sphere = Default::default();
        // If we launch a ray in front of it
        let ray: Ray = Ray {
            source: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = sphere.check_collision(&ray);
        // There is a collision
        assert!(result.is_some());
        println!("{:?}", result);
    }

    #[test]
    fn ray_sphere_no_collision() {
        let sphere: Sphere = Default::default(); // Given a unit sphere
        let ray: Ray = Ray {
            // If we launch a ray next to it and orthogonally
            source: Vec3 {
                x: 2.0,
                y: 0.0,
                z: -2.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        assert!(sphere.check_collision(&ray).is_none()); // There is no collision
    }
}
