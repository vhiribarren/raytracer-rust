use crate::primitives::{Ray, Vec3};
use crate::scene::{Camera, Scene};
use crate::textures::Color;
use log::debug;

pub trait DrawCanvas {
    fn draw(&mut self, x: u32, y: u32, color: &Color);
}

pub struct RenderOptions {
    pub canvas_width: u32,
    pub canvas_height: u32,
}

pub fn render(scene: &Scene, canvas: &mut impl DrawCanvas, options: &RenderOptions) {
    debug!("{} objects to process", scene.objects.len());
    for (x, y, ray) in generate_rays(&scene.camera, options.canvas_width, options.canvas_height) {
        let mut color: &Color = &Default::default();
        let mut shortest_distance: f64 = std::f64::MAX;
        for object in &scene.objects {
            match object.check_collision(&ray) {
                Some(vec) => {
                    let distance = vec.distance(scene.camera.eye);
                    if distance < shortest_distance {
                        shortest_distance = distance;
                        color = &object.texture().color;
                    }
                },
                None => continue,
            };
        }
        canvas.draw(x, options.canvas_height - y, color);
    }
}

fn generate_rays<'a>(
    camera: &'a Camera,
    screen_width: u32,
    screen_height: u32,
) -> impl Iterator<Item = (u32, u32, Ray)> + 'a {
    let max_index: u32 = screen_width.checked_mul(screen_height).unwrap();
    let width_step = camera.width / (screen_width as f64);
    let height_step = camera.height / (screen_height as f64);
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
                width_step / 2.0 + ((i_float * width_step) % camera.width),
                height_step / 2.0 + (((i_float * width_step) / camera.width).trunc() * height_step),
            ))
        } else {
            None
        }
    };
    let mut index: u32 = 0;
    let camera_axis_z = Vec3::from_to_point(camera.eye, camera.screen_center).normalize();
    let camera_axis_y = camera.up.normalize();
    let camera_axis_x = camera_axis_y.cross_product(camera_axis_z);
    std::iter::from_fn(move || match to_screen_coords(index) {
        None => None,
        Some((screen_x, screen_y)) => {
            let (camera_x, camera_y) = to_camera_coords(index).unwrap();
            let ray_destination = camera.screen_center
                + (camera_x as f64 - camera.width / 2.0) * camera_axis_x
                + (camera_y as f64 - camera.height / 2.0) * camera_axis_y;

            let screen_ray = (
                screen_x,
                screen_y,
                Ray::ray_from_to(camera.eye, ray_destination),
            );
            index += 1;
            Some(screen_ray)
        }
    })
}
