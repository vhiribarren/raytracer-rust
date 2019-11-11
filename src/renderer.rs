use crate::primitives::{Camera, Ray, Sphere, Vec3};

pub fn generate_rays<'a>(
    camera: &'a Camera,
    screen_width: u32,
    screen_height: u32,
) -> impl Iterator<Item = ScreenRay> + 'a {
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

            let screen_ray = ScreenRay {
                screen_x,
                screen_y,
                ray: Ray::ray_from_to(camera.eye, ray_destination),
            };
            index += 1;
            Some(screen_ray)
        }
    })
}

#[derive(Debug)]
pub struct ScreenRay {
    screen_x: u32,
    screen_y: u32,
    ray: Ray,
}
