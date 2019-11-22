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
        let max_index: u32 = screen_width.checked_mul(screen_height).unwrap();
        let width_step = self.width / (screen_width as f64);
        let height_step = self.height / (screen_height as f64);
        let to_screen_coords = move |i: u32| {
            if i < max_index {
                Some((i % screen_width, i / screen_width))
            } else {
                None
            }
        };
        let to_camera_coords = move |i: u32| {
            let i_float = i as f64;
            if i < max_index {
                Some((
                    width_step / 2.0 + ((i_float * width_step) % self.width),
                    height_step / 2.0
                        + (((i_float * width_step) / self.width).trunc() * height_step),
                ))
            } else {
                None
            }
        };
        let mut index: u32 = 0;
        let camera_axis_z = Vec3::from_to_point(self.eye, self.screen_center).normalize();
        let camera_axis_y = self.up.normalize();
        let camera_axis_x = camera_axis_y.cross_product(camera_axis_z);
        let iter = std::iter::from_fn(move || match to_screen_coords(index) {
            None => None,
            Some((screen_x, screen_y)) => {
                let (camera_x, camera_y) = to_camera_coords(index).unwrap();
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
