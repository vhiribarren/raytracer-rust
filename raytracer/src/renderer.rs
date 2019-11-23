use crate::primitives::{Ray, Vec3};
use crate::scene::{RayEmitter, Scene};
use crate::textures::Color;
use log::debug;

pub trait DrawCanvas {
    fn draw(&mut self, x: u32, y: u32, color: &Color);
}

pub struct RenderOptions {
    pub canvas_width: u32,
    pub canvas_height: u32,
}

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
            eye: Vec3::new(0.0, 0.0, -10.5),
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
        let renderer =
            CameraCoordinateMapping::new(self.width, self.height, screen_width, screen_height);
        let mut index: u32 = 0;
        let camera_axis_z = Vec3::from_to_point(self.eye, self.screen_center).normalize();
        let camera_axis_y = self.up.normalize();
        let camera_axis_x = camera_axis_y.cross_product(camera_axis_z);
        let iter = std::iter::from_fn(move || match renderer.to_screen_coords(index) {
            None => None,
            Some((screen_x, screen_y)) => {
                let (camera_x, camera_y) = renderer.to_camera_coords(index).unwrap();
                let ray_destination = self.screen_center
                    + (camera_x as f64 - self.width / 2.0) * camera_axis_x
                    + (camera_y as f64 - self.height / 2.0) * camera_axis_y;

                let screen_ray = (
                    screen_x,
                    screen_y,
                    Ray::ray_from_to(self.eye, ray_destination),
                );
                index += 1;
                Some(screen_ray)
            }
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
        let renderer =
            CameraCoordinateMapping::new(self.width, self.height, screen_width, screen_height);
        let mut index: u32 = 0;
        let camera_axis_z = self.eye_direction.normalize();
        let camera_axis_y = self.up.normalize();
        let camera_axis_x = camera_axis_y.cross_product(camera_axis_z);
        let iter = std::iter::from_fn(move || match renderer.to_screen_coords(index) {
            None => None,
            Some((screen_x, screen_y)) => {
                let (camera_x, camera_y) = renderer.to_camera_coords(index).unwrap();
                let ray_source = self.screen_center
                    + (camera_x as f64 - self.width / 2.0) * camera_axis_x
                    + (camera_y as f64 - self.height / 2.0) * camera_axis_y;

                let screen_ray = (
                    screen_x,
                    screen_y,
                    Ray {
                        source: ray_source,
                        direction: self.eye_direction,
                    },
                );
                index += 1;
                Some(screen_ray)
            }
        });
        Box::new(iter)
    }
}

pub struct CameraCoordinateMapping {
    camera_width: f64,
    screen_width: u32,
    width_step: f64,
    height_step: f64,
    max_index: u32,
}

impl CameraCoordinateMapping {
    pub fn new(
        camera_width: f64,
        camera_height: f64,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        let max_index = screen_width.checked_mul(screen_height).unwrap();
        let width_step = camera_width / (screen_width as f64);
        let height_step = camera_height / (screen_height as f64);
        CameraCoordinateMapping {
            camera_width,
            screen_width,
            width_step,
            height_step,
            max_index,
        }
    }

    fn to_screen_coords(&self, i: u32) -> Option<(u32, u32)> {
        if i < self.max_index {
            Some((i % self.screen_width, i / self.screen_width))
        } else {
            None
        }
    }

    fn to_camera_coords(&self, i: u32) -> Option<(f64, f64)> {
        let i_float = i as f64;
        if i < self.max_index {
            Some((
                self.width_step / 2.0 + ((i_float * self.width_step) % self.camera_width),
                self.height_step / 2.0
                    + (((i_float * self.width_step) / self.camera_width).trunc()
                        * self.height_step),
            ))
        } else {
            None
        }
    }
}

pub fn render(scene: &Scene, canvas: &mut impl DrawCanvas, options: &RenderOptions) {
    debug!("{} objects to process", scene.objects.len());
    let camera = &scene.camera;
    for (x, y, ray) in camera.generate_rays(options.canvas_width, options.canvas_height) {
        let mut color: &Color = &Default::default();
        let mut shortest_distance: f64 = std::f64::MAX;
        for object in &scene.objects {
            match object.check_collision(&ray) {
                Some(vec) => {
                    let distance = vec.distance(ray.source);
                    if distance < shortest_distance {
                        shortest_distance = distance;
                        color = &object.texture().color;
                    }
                }
                None => continue,
            };
        }
        canvas.draw(x, options.canvas_height - y, color);
    }
}
