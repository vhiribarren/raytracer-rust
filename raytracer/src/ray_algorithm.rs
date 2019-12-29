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
use crate::lights::AnyLightObject;
use crate::primitives::Ray;
use crate::result::RaytracerError;
use crate::result::Result;
use crate::scene::{AnySceneObject, Scene};
use crate::vector::Vec3;
use crate::UnitInterval;
use rand::Rng;
use std::f64;

pub trait AnyPixelRenderStrategy: Send + Sync {
    fn render_pixel(
        &self,
        scene: &Scene,
        canvas_x: UnitInterval,
        canvas_y: UnitInterval,
        pixel_width: f64,
        pixel_height: f64,
    ) -> Result<Color>;
}

pub mod strategy {
    use super::*;

    pub struct StandardRenderStrategy;

    impl AnyPixelRenderStrategy for StandardRenderStrategy {
        fn render_pixel(
            &self,
            scene: &Scene,
            canvas_x: UnitInterval,
            canvas_y: UnitInterval,
            pixel_width: f64,
            pixel_height: f64,
        ) -> Result<Color> {
            let x_unit = pixel_width / 2.0 + canvas_x;
            let y_unit = pixel_height / 2.0 + canvas_y;
            let camera_ray = scene.camera.generate_ray(x_unit, y_unit);
            launch_ray(&camera_ray, scene, 0)
        }
    }

    pub struct RandomAntiAliasingRenderStrategy {
        pub rays_per_pixel: u32,
    }

    impl AnyPixelRenderStrategy for RandomAntiAliasingRenderStrategy {
        fn render_pixel(
            &self,
            scene: &Scene,
            canvas_x: UnitInterval,
            canvas_y: UnitInterval,
            pixel_width: f64,
            pixel_height: f64,
        ) -> Result<Color> {
            let mut rng = rand::thread_rng();
            let mut result_color = Color::BLACK;
            for _ in 0..self.rays_per_pixel {
                let x_unit: f64 = rng.gen::<f64>() * pixel_width + canvas_x;
                let y_unit: f64 = rng.gen::<f64>() * pixel_height + canvas_y;
                let camera_ray = scene.camera.generate_ray(x_unit, y_unit);
                result_color +=
                    (1.0 / (self.rays_per_pixel as f64)) * launch_ray(&camera_ray, scene, 0)?;
            }
            Ok(result_color)
        }
    }
}

fn launch_ray(camera_ray: &Ray, scene: &Scene, depth: u8) -> Result<Color> {
    if depth > scene.config.maximum_light_recursion {
        return Ok(Color::BLACK);
    }

    // Check if there is an object to process for this pixel
    let collision_context = match search_object_collision(&camera_ray, &scene.objects) {
        Some(collision_context) => collision_context,
        None => {
            return Ok(scene.config.world_color.clone());
        }
    };
    let CollisionContext {
        object: nearest_object,
        collision_point,
        array_index,
    } = collision_context;

    // After having found the nearest object, we launch a ray to the light
    let mut total_color = Color::BLACK;
    total_color += illumination_from_lights(
        &collision_context,
        &scene.lights,
        &scene.objects,
        &camera_ray,
    )?;

    // Refraction light
    if let Some(transparency) = &nearest_object.effects().transparency {
        let surface_normal = nearest_object
            .normal_at(collision_point)
            .ok_or(RaytracerError::NormalNotFound(array_index))?
            .normalize();
        let n_ratio = scene.config.world_refractive_index / transparency.refractive_index;
        let cos_refraction = camera_ray.direction.dot_product(surface_normal);
        let sin_square_refraction = n_ratio.powi(2) * (1.0 - cos_refraction.powi(2));
        let refraction_direction = n_ratio * camera_ray.direction
            - (n_ratio * cos_refraction + (1.0 - sin_square_refraction).sqrt()) * surface_normal;
        // Go up to object exterior
        let refraction_ray = Ray::new(collision_point, refraction_direction).shift_source();
        if let Some(collision_context) = search_object_collision(&refraction_ray, &scene.objects) {
            // TODO only the nearest_object is necessary
            // launch new ray
            let exit_point = collision_context.collision_point;
            let new_ray = Ray::new(exit_point, camera_ray.direction).shift_source();
            total_color += transparency.alpha * launch_ray(&new_ray, scene, depth + 1)?;
        }
    }

    // Reflexion
    if let Some(mirror) = &nearest_object.effects().mirror {
        let surface_normal = nearest_object
            .normal_at(collision_point)
            .ok_or(RaytracerError::NormalNotFound(array_index))?
            .normalize();
        let ray_reflexion = Ray::new(
            collision_point,
            camera_ray.direction.reflect(surface_normal).normalize(),
        )
        .shift_source();
        total_color += mirror.coeff * launch_ray(&ray_reflexion, scene, depth + 1)?;
    }

    // Ambient light
    if let Some(ambient_light) = &scene.config.ambient_light {
        total_color += ambient_light * &nearest_object.color_at(collision_point);
    }

    Ok(total_color)
}

pub struct CollisionContext<'a> {
    pub object: &'a dyn AnySceneObject,
    pub collision_point: Vec3,
    pub array_index: usize,
}

fn search_object_collision<'a>(
    ray: &Ray,
    objects: &'a [Box<dyn AnySceneObject>],
) -> Option<CollisionContext<'a>> {
    let mut shortest_distance: f64 = f64::MAX;
    let mut nearest_object_opt: Option<&Box<dyn AnySceneObject>> = None;
    let mut collision_point: Vec3 = Default::default();
    let mut array_index = std::usize::MAX;
    // For each pixel, we search for collision with objects
    // We also take into account the nearest object, for now
    for (index, object_candidate) in objects.iter().enumerate() {
        if let Some(collision_point_candidate) = object_candidate.check_collision(&ray) {
            let distance = collision_point_candidate.distance(ray.source);
            if distance <= 1e-12 {
                continue;
            } else if distance < shortest_distance {
                shortest_distance = distance;
                nearest_object_opt = Some(object_candidate);
                collision_point = collision_point_candidate;
                array_index = index;
            }
        }
    }
    nearest_object_opt.map(|n| CollisionContext {
        object: &**n,
        collision_point,
        array_index,
    })
}

fn illumination_from_lights(
    collision_context: &CollisionContext,
    lights: &[Box<dyn AnyLightObject>],
    objects: &[Box<dyn AnySceneObject>],
    camera_ray: &Ray,
) -> Result<Color> {
    let mut total_color = Color::BLACK;
    let surface_point = collision_context.collision_point;
    let object = collision_context.object;
    for current_light in lights {
        let light_ray = Ray::ray_from_to(surface_point, current_light.source());

        // Generate shadow, by skipping process if there is an obstacle between object and light
        if ray_encounter_obstacle(&light_ray, &current_light.source(), objects) {
            continue;
        }

        // Build values needed for light computation
        let light_direction = light_ray.direction;
        let light_color = current_light.light_color_at(surface_point);
        let surface_normal =
            object
                .normal_at(surface_point)
                .ok_or(RaytracerError::NormalNotFound(
                    collision_context.array_index,
                ))?;
        let ray_reflexion = camera_ray.direction.reflect(surface_normal).normalize();

        // Diffuse reflection
        let reflection_angle = light_direction.dot_product(surface_normal);
        if reflection_angle > 0.0 {
            total_color +=
                reflection_angle * &(light_color.clone() * object.color_at(surface_point));
        }

        // Add specular / phong light
        if let Some(phong) = &object.effects().phong {
            let specular_angle = light_direction.dot_product(ray_reflexion);
            if specular_angle > 0.0 {
                total_color += light_color.clone()
                    * (specular_angle).powi(phong.size as i32)
                    * phong.lum_coeff;
            }
        }
    }
    Ok(total_color)
}

#[allow(clippy::if_same_then_else)]
fn ray_encounter_obstacle(
    ray: &Ray,
    destination: &Vec3,
    objects: &[Box<dyn AnySceneObject>],
) -> bool {
    let source = ray.source;
    let light_distance = Vec3::between_points(source, *destination).norm();
    // Check of object obstruction between light and collision point
    for candidate_object in objects {
        if let Some(obstruction_point) = candidate_object.check_collision(ray) {
            let object_distance = Vec3::between_points(source, obstruction_point).norm();
            if object_distance > light_distance {
                // Not between the object and the light
                continue;
            } else if object_distance <= 1e-12 {
                // TODO Check why this value is so high, it was f64::EPSILON before
                // Float comparison error, source is probably also the candidate object
                continue;
            } else {
                // Object is hiding an other
                return true;
            }
        }
    }
    false
}
