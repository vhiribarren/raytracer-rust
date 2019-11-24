use crate::primitives::{Ray, Vec3};
use crate::scene::{Scene, SceneObject};
use crate::textures::Color;
use crate::utils;
use log::debug;
use std::f64;

pub trait DrawCanvas {
    fn draw(&mut self, x: u32, y: u32, color: &Color);
}

pub struct RenderOptions {
    pub canvas_width: u32,
    pub canvas_height: u32,
}

pub fn render(
    scene: &Scene,
    canvas: &mut impl DrawCanvas,
    options: &RenderOptions,
) -> Result<(), String> {
    debug!("render: {} objects to process", scene.objects.len());
    debug!("render: {} lights to process", scene.lights.len());
    let camera = &scene.camera;
    let light = match scene.lights.len() {
        0 => return Err(String::from("There is no light in the scene")),
        1 => &scene.lights[0],
        _ => unimplemented!("Only one light is implemented for now"),
    };
    // We scan the pixels of the canvas
    'pixel_loop: for (x, y, ray) in
        camera.generate_rays(options.canvas_width, options.canvas_height)
    {
        let mut shortest_distance: f64 = std::f64::MAX;
        let mut nearest_object: Option<&Box<dyn SceneObject>> = None;
        let mut collision_point: Vec3 = Default::default();
        // For each pixel, we search for collision with objects
        // We also take into account the nearest object, for now
        for object in &scene.objects {
            if let Some(vec) = object.check_collision(&ray) {
                let distance = vec.distance(ray.source);
                if distance < shortest_distance {
                    shortest_distance = distance;
                    nearest_object = Some(object);
                    collision_point = vec;
                }
            }
        }

        if let Some(obj) = nearest_object {
            // After having found the nearest object, we launch a ray to the light
            let light_ray = Ray::ray_from_to(collision_point, light.source());
            let light_direction = light_ray.direction;
            let light_distance = Vec3::between_points(collision_point, light.source()).norm();
            // Check of object obstruction between light and collision point
            for object in &scene.objects {
                if utils::cmp_ref(obj, object) {
                    continue;
                }
                if let Some(vec) = object.check_collision(&light_ray) {
                    let object_distance = Vec3::between_points(collision_point, vec).norm();
                    if object_distance > light_distance {
                        continue;
                    } else {
                        // Object is hiding an other
                        continue 'pixel_loop;
                    }
                }
            }

            // Try a first simple light model where intensity vary depending on angle with normal
            let surface_normal = obj
                .normal_at(collision_point)
                .ok_or(String::from("No normal found"))?;
            let cos_angle = light_direction.dot_product(surface_normal)
                / (light_direction.norm() * surface_normal.norm());
            let intensity: f64 = if cos_angle > 0.0 { cos_angle } else { 0.0 };
            canvas.draw(
                x,
                options.canvas_height - y,
                &(intensity * &obj.texture().color),
            );
        }
    }
    Ok(())
}
