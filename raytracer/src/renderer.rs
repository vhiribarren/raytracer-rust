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

use crate::colors::Color;
use crate::primitives::Ray;
use crate::scene::{AnySceneObject, Scene};
use crate::utils;
use crate::vector::Vec3;
use log::{debug, info};
use std::f64;
use std::time;

pub trait DrawCanvas {
    fn draw(&mut self, x: u32, y: u32, color: &Color) -> Result<(), String>;
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
    let start_render_instant = time::Instant::now();
    debug!("render: {} objects to process", scene.objects.len());
    debug!("render: {} lights to process", scene.lights.len());
    if scene.lights.is_empty() {
        return Err(String::from("There is no light in the scene"));
    }
    let camera = &scene.camera;
    // We scan the pixels of the canvas
    for (x, y, ray) in camera.generate_rays(options.canvas_width, options.canvas_height) {
        let mut shortest_distance: f64 = std::f64::MAX;
        let mut nearest_object_opt: Option<&Box<dyn AnySceneObject>> = None;
        let mut collision_point: Vec3 = Default::default();
        // For each pixel, we search for collision with objects
        // We also take into account the nearest object, for now
        for object_candidate in &scene.objects {
            if let Some(collision_point_candidate) = object_candidate.check_collision(&ray) {
                let distance = collision_point_candidate.distance(ray.source);
                if distance < shortest_distance {
                    shortest_distance = distance;
                    nearest_object_opt = Some(object_candidate);
                    collision_point = collision_point_candidate;
                }
            }
        }

        let nearest_object = match nearest_object_opt {
            Some(val) => val,
            _ => {
                canvas.draw(x, options.canvas_height - y, &(scene.options.world_color))?;
                continue;
            }
        };

        let collision_point = collision_point;

        // After having found the nearest object, we launch a ray to the light
        let mut total_color = Color::BLACK;
        'light_loop: for current_light in &scene.lights {
            let light_ray = Ray::ray_from_to(collision_point, current_light.source());
            let light_distance =
                Vec3::between_points(collision_point, current_light.source()).norm();
            // Check of object obstruction between light and collision point
            for candidate_object in &scene.objects {
                if utils::ref_equals(nearest_object, candidate_object) {
                    continue;
                }
                if let Some(obstruction_point) = candidate_object.check_collision(&light_ray) {
                    let object_distance =
                        Vec3::between_points(collision_point, obstruction_point).norm();
                    if object_distance > light_distance {
                        continue;
                    } else {
                        // Object is hiding an other
                        continue 'light_loop;
                    }
                }
            }

            // Build values needed for light computation
            let light_direction = light_ray.direction.normalize();
            let light_color = current_light.light_color_at(collision_point);
            let surface_normal = nearest_object
                .normal_at(collision_point)
                .ok_or_else(|| String::from("No normal found"))?;
            let ray_reflexion = ray.direction.reflect(surface_normal).normalize();

            // Diffuse reflection
            let reflection_angle = light_direction.dot_product(surface_normal);
            if reflection_angle > 0.0 {
                total_color += reflection_angle
                    * &(light_color.clone() * nearest_object.color_at(collision_point));
            }

            // Add specular / phong light
            if let Some(phong) = &nearest_object.effects().phong {
                let specular_angle = light_direction.dot_product(ray_reflexion);
                if specular_angle > 0.0 {
                    total_color += light_color.clone()
                        * (specular_angle).powi(phong.size as i32)
                        * phong.lum_coeff;
                }
            }
        }
        // Ambien light
        if let Some(ambient_light) = &scene.options.ambient_light {
            total_color += ambient_light * &nearest_object.color_at(collision_point);
        }

        canvas.draw(x, options.canvas_height - y, &(total_color))?;
    }
    info!(
        "Rendering duration: {:.3} seconds",
        start_render_instant.elapsed().as_secs_f32()
    );
    Ok(())
}
