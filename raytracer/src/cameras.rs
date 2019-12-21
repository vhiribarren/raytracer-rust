/*
MIT License

Copyright (c) 2019 Vincent Hiribarren

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

use crate::primitives::Ray;
use crate::scene::RayEmitter;
use crate::utils::{f64_gt, f64_lt};
use crate::vector::{Mat3, Vec3};
use crate::UnitInterval;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct PerspectiveCamera {
    eye: Vec3,
    screen_center: Vec3,
    width: f64,
    height: f64,
    axis_x: Vec3,
    axis_y: Vec3,
    axis_z: Vec3,
}

impl PerspectiveCamera {
    pub fn new(
        screen_center: Vec3,
        look_at: Vec3,
        width: f64,
        height: f64,
        angle: f64,
    ) -> PerspectiveCamera {
        let eye_direction = Vec3::between_points(screen_center, look_at).normalize();
        let transform = Mat3::transformation_between(Vec3::new(0.0, 0.0, 1.0), eye_direction);
        let distance_eye_center = height / (2.0 * angle.tan());
        let eye = screen_center - distance_eye_center * eye_direction;
        let axis_z = Vec3::between_points(eye, screen_center).normalize();
        let axis_y = (transform * Vec3::new(0.0, 1.0, 0.0)).normalize();
        let axis_x = axis_y.cross_product(axis_z);
        PerspectiveCamera {
            screen_center,
            eye,
            width,
            height,
            axis_x,
            axis_y,
            axis_z,
        }
    }
}

impl Default for PerspectiveCamera {
    fn default() -> PerspectiveCamera {
        PerspectiveCamera::new(
            Vec3::new(0.0, 0.0, -50.0),
            Vec3::new(0.0, 0.0, 50.0),
            16.0,
            9.0,
            PI / 4.0,
        )
    }
}

impl RayEmitter for PerspectiveCamera {
    fn generate_ray(&self, canvas_x: UnitInterval, canvas_y: UnitInterval) -> Ray {
        assert!(
            f64_lt(canvas_x, 1.0) && f64_gt(canvas_x, 0.0),
            "canvas_x is: {}",
            canvas_x
        );
        assert!(
            f64_lt(canvas_y, 1.0) && f64_gt(canvas_y, 0.0),
            "canvas_y is: {}",
            canvas_y
        );
        let ray_destination = self.screen_center - (self.width / 2.0) * self.axis_x
            + (self.height / 2.0) * self.axis_y
            + canvas_x * self.width * self.axis_x
            - canvas_y * self.height * self.axis_y;
        Ray::ray_from_to(self.eye, ray_destination)
    }
}

#[derive(Debug)]
pub struct OrthogonalCamera {
    screen_center: Vec3,
    width: f64,
    height: f64,
    axis_x: Vec3,
    axis_y: Vec3,
    axis_z: Vec3,
}

impl OrthogonalCamera {
    pub fn new(eye: Vec3, look_at: Vec3, width: f64, height: f64) -> Self {
        let axis_z = Vec3::between_points(eye, look_at).normalize();
        let transform = Mat3::transformation_between(Vec3::new(0.0, 0.0, 1.0), axis_z);
        let axis_y = transform * Vec3::new(0.0, 1.0, 0.0);
        let axis_x = axis_y.cross_product(axis_z);
        OrthogonalCamera {
            screen_center: eye,
            axis_x,
            axis_y,
            axis_z,
            width,
            height,
        }
    }
}

impl Default for OrthogonalCamera {
    fn default() -> Self {
        OrthogonalCamera::new(
            Vec3::new(0.0, 0.0, -10.0),
            Vec3::new(0.0, 0.0, 0.0),
            16.0,
            9.0,
        )
    }
}

impl RayEmitter for OrthogonalCamera {
    fn generate_ray(&self, canvas_x: UnitInterval, canvas_y: UnitInterval) -> Ray {
        assert!(
            f64_lt(canvas_x, 1.0) && f64_gt(canvas_x, 0.0),
            "canvas_x is: {}",
            canvas_x
        );
        assert!(
            f64_lt(canvas_y, 1.0) && f64_gt(canvas_y, 0.0),
            "canvas_y is: {}",
            canvas_y
        );
        let ray_source = self.screen_center - (self.width / 2.0) * self.axis_x
            + (self.height / 2.0) * self.axis_y
            + canvas_x * self.width * self.axis_x
            - canvas_y * self.height * self.axis_y;
        Ray::new(ray_source, self.axis_z)
    }
}
