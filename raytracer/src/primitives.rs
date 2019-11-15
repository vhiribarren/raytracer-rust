pub trait Collision {
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
}

#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}


#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn from_to_point(source: Vec3, destination: Vec3) -> Vec3 {
        destination - source
    }

    pub fn dot_product(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let norm = self.norm();
        Vec3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Self> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
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
    pub center: Vec3,
    pub normal: Vec3,
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
