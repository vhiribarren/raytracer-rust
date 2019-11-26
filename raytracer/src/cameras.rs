use crate::primitives::{Ray, Vec3};
use crate::scene::RayEmitter;

#[derive(Debug)]
pub struct PerspectiveCamera {
    pub eye: Vec3,
    pub screen_center: Vec3,
    pub up: Vec3,
    pub width: f64,
    pub height: f64,
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        PerspectiveCamera {
            eye: Vec3::new(0.0, 0.0, -100.0),
            screen_center: Vec3::new(0.0, 0.0, -10.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            width: 16.0,
            height: 9.0,
        }
    }
}

impl RayEmitter for PerspectiveCamera {
    fn generate_rays<'a>(
        &'a self,
        screen_width: u32,
        screen_height: u32,
    ) -> Box<dyn Iterator<Item = (u32, u32, Ray)> + 'a> {
        let surface_iterator = CameraRectangleSurfaceIterator::new(
            self.width,
            self.height,
            screen_width,
            screen_height,
        );
        let camera_axis_z = Vec3::between_points(self.eye, self.screen_center).normalize();
        let camera_axis_y = self.up.normalize();
        let camera_axis_x = camera_axis_y.cross_product(camera_axis_z);
        let iter = surface_iterator.map(move |(screen_x, screen_y, camera_x, camera_y)| {
            let ray_destination = self.screen_center
                + (camera_x as f64 - self.width / 2.0) * camera_axis_x
                + (camera_y as f64 - self.height / 2.0) * camera_axis_y;
            (
                screen_x,
                screen_y,
                Ray::ray_from_to(self.eye, ray_destination),
            )
        });
        Box::new(iter)
    }
}

#[derive(Debug)]
pub struct OrthogonalCamera {
    pub screen_center: Vec3,
    pub up: Vec3,
    pub eye_direction: Vec3,
    pub width: f64,
    pub height: f64,
}

impl Default for OrthogonalCamera {
    fn default() -> Self {
        OrthogonalCamera {
            screen_center: Vec3::new(0.0, 0.0, -10.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            eye_direction: Vec3::new(0.0, 0.0, 1.0),
            width: 16.0,
            height: 9.0,
        }
    }
}

impl RayEmitter for OrthogonalCamera {
    fn generate_rays<'a>(
        &'a self,
        screen_width: u32,
        screen_height: u32,
    ) -> Box<dyn Iterator<Item = (u32, u32, Ray)> + 'a> {
        let surface_iterator = CameraRectangleSurfaceIterator::new(
            self.width,
            self.height,
            screen_width,
            screen_height,
        );
        let camera_axis_z = self.eye_direction.normalize();
        let camera_axis_y = self.up.normalize();
        let camera_axis_x = camera_axis_y.cross_product(camera_axis_z);
        let iter = surface_iterator.map(move |(screen_x, screen_y, camera_x, camera_y)| {
            let ray_source = self.screen_center
                + (camera_x as f64 - self.width / 2.0) * camera_axis_x
                + (camera_y as f64 - self.height / 2.0) * camera_axis_y;

            (
                screen_x,
                screen_y,
                Ray {
                    source: ray_source,
                    direction: self.eye_direction,
                },
            )
        });
        Box::new(iter)
    }
}

pub struct CameraRectangleSurfaceIterator {
    camera_width: f64,
    screen_width: u32,
    width_step: f64,
    height_step: f64,
    max_index: u32,
    current_index: u32,
}

impl CameraRectangleSurfaceIterator {
    pub fn new(
        camera_width: f64,
        camera_height: f64,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        let max_index = screen_width
            .checked_mul(screen_height)
            .expect("Screen dimensions should be less high.");
        let width_step = camera_width / (screen_width as f64);
        let height_step = camera_height / (screen_height as f64);
        let current_index = 0;
        CameraRectangleSurfaceIterator {
            camera_width,
            screen_width,
            width_step,
            height_step,
            max_index,
            current_index,
        }
    }

    fn to_screen_coords(&self, i: u32) -> (u32, u32) {
        (i % self.screen_width, i / self.screen_width)
    }

    fn to_camera_coords(&self, i: u32) -> (f64, f64) {
        let i_float = i as f64;
        (
            self.width_step / 2.0 + ((i_float * self.width_step) % self.camera_width),
            self.height_step / 2.0
                + (((i_float * self.width_step) / self.camera_width).trunc() * self.height_step),
        )
    }
}

impl Iterator for CameraRectangleSurfaceIterator {
    type Item = (u32, u32, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.max_index {
            let (camera_x, camera_y) = self.to_camera_coords(self.current_index);
            let (screen_x, screen_y) = self.to_screen_coords(self.current_index);
            self.current_index += 1;
            Some((screen_x, screen_y, camera_x, camera_y))
        } else {
            None
        }
    }
}
