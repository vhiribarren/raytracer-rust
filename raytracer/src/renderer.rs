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
use crate::scene::{AnySceneObject, Scene};
use crate::vector::Vec3;
use crate::UnitInterval;
use log::{debug, info, warn};
use rand::Rng;
use std::f64;
use std::time;

pub use iterator::*;

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Pixel {
        Pixel { x, y, color }
    }
}

pub trait DrawCanvas {
    fn draw(&mut self, pixel: Pixel) -> Result<(), String>;
}

pub trait AnyPixelRenderStrategy {
    fn render_pixel(
        &self,
        scene: &Scene,
        canvas_x: UnitInterval,
        canvas_y: UnitInterval,
        pixel_width: f64,
        pixel_height: f64,
    ) -> Result<Color, String>;
}

pub struct RenderConfiguration {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub render_strategy: Box<dyn AnyPixelRenderStrategy>,
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
        ) -> Result<Color, String> {
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
        ) -> Result<Color, String> {
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

pub struct ProgressiveRenderer<'a> {
    scene: &'a Scene,
    config: &'a RenderConfiguration,
}

impl ProgressiveRenderer<'_> {
    pub fn new<'a>(scene: &'a Scene, config: &'a RenderConfiguration) -> ProgressiveRenderer<'a> {
        ProgressiveRenderer { scene, config }
    }

    pub fn total_pixels(&self) -> u32 {
        self.config.canvas_width * self.config.canvas_height
    }

    pub fn render<E, F>(&self, mut callback: E, mut finally: F) -> Result<(), String>
    where
        E: FnMut(Pixel) -> Result<(), String>,
        F: FnMut(),
    {
        if cfg!(debug_assertions) {
            warn!("Debug compiled binary is used, performance will be low!");
        }

        debug!("render: {} objects to process", self.scene.objects.len());
        debug!("render: {} lights to process", self.scene.lights.len());
        if self.scene.lights.is_empty() {
            return Err(String::from("There is no light in the scene"));
        }

        let area_iterator = AreaRenderIterator::with_full_area(self.scene, self.config);

        {
            info!("render: start process...");
            let start_render_instant = time::Instant::now();
            for result in area_iterator {
                match result {
                    Err(val) => return Err(val),
                    Ok(pixel) => {
                        callback(pixel)?;
                    }
                }
            }
            finally();
            info!("render: done!");
            info!(
                "render: duration: {:.3} seconds",
                start_render_instant.elapsed().as_secs_f32()
            );
        }

        Ok(())
    }
}

pub fn simple_render_to_canvas(
    scene: &Scene,
    canvas: &mut impl DrawCanvas,
    config: &RenderConfiguration,
) -> Result<(), String> {
    ProgressiveRenderer::new(scene, config).render(|pixel| canvas.draw(pixel), || {})
}

pub mod iterator {
    use super::*;

    pub struct AreaRenderIterator<'a> {
        scene: &'a Scene,
        config: &'a RenderConfiguration,
        area_x_origin: u32,
        #[allow(dead_code)]
        area_y_origin: u32,
        area_width: u32,
        area_height: u32,
        area_x_current: u32,
        area_y_current: u32,
        pixel_width: f64,
        pixel_height: f64,
    }

    impl AreaRenderIterator<'_> {
        pub fn new<'a>(
            scene: &'a Scene,
            config: &'a RenderConfiguration,
            area_x: u32,
            area_y: u32,
            area_width: u32,
            area_height: u32,
        ) -> AreaRenderIterator<'a> {
            AreaRenderIterator {
                scene,
                config,
                area_x_origin: area_x,
                area_y_origin: area_y,
                area_width,
                area_height,
                area_x_current: area_x,
                area_y_current: area_y,
                pixel_width: 1.0 / config.canvas_width as f64,
                pixel_height: 1.0 / config.canvas_height as f64,
            }
        }

        pub fn with_full_area<'a>(
            scene: &'a Scene,
            config: &'a RenderConfiguration,
        ) -> AreaRenderIterator<'a> {
            Self::new(
                scene,
                config,
                0,
                0,
                config.canvas_width,
                config.canvas_height,
            )
        }

        pub fn total_pixels(&self) -> usize {
            (self.area_width * self.area_height) as usize
        }
    }

    impl Iterator for AreaRenderIterator<'_> {
        type Item = Result<Pixel, String>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.area_y_current >= self.area_height {
                return None;
            }
            let x = self.area_x_current;
            let y = self.area_y_current;
            let canvas_x = (self.area_x_current as f64) / (self.config.canvas_width as f64);
            let canvas_y = (self.area_y_current as f64) / (self.config.canvas_height as f64);
            let render_strategy = &*self.config.render_strategy;
            let result_color = render_strategy.render_pixel(
                self.scene,
                canvas_x,
                canvas_y,
                self.pixel_width,
                self.pixel_height,
            );
            let color = match result_color {
                Ok(val) => val,
                Err(val) => return Some(Err(val)),
            };
            self.area_x_current += 1;
            if self.area_x_current >= self.area_width {
                self.area_x_current = self.area_x_origin;
                self.area_y_current += 1;
            }
            Some(Ok(Pixel::new(x, y, color)))
        }
    }
}

fn launch_ray(camera_ray: &Ray, scene: &Scene, depth: u8) -> Result<Color, String> {
    if depth > scene.config.maximum_light_recursion {
        return Ok(Color::BLACK);
    }

    // Check if there is an object to process for this pixel
    let (nearest_object, collision_point) =
        match search_object_collision(&camera_ray, &scene.objects) {
            Some((object, point)) => (object, point),
            None => {
                return Ok(scene.config.world_color.clone());
            }
        };

    // After having found the nearest object, we launch a ray to the light
    let mut total_color = Color::BLACK;
    total_color += illumination_from_lights(
        nearest_object,
        collision_point,
        &scene.lights,
        &scene.objects,
        &camera_ray,
    )?;

    // Refraction light
    if let Some(transparency) = &nearest_object.effects().transparency {
        let surface_normal = nearest_object
            .normal_at(collision_point)
            .ok_or_else(|| String::from("No normal found"))?
            .normalize();
        let n_ratio = scene.config.world_refractive_index / transparency.refractive_index;
        let cos_refraction = camera_ray.direction.dot_product(surface_normal);
        let sin_square_refraction = n_ratio.powi(2) * (1.0 - cos_refraction.powi(2));
        let refraction_direction = n_ratio * camera_ray.direction
            - (n_ratio * cos_refraction + (1.0 - sin_square_refraction).sqrt()) * surface_normal;
        // Go up to object exterior
        let refraction_ray = Ray::new(collision_point, refraction_direction).shift_source();
        if let Some((_, exit_point)) = search_object_collision(&refraction_ray, &scene.objects) {
            // TODO only the nearest_object is necessary
            // launch new ray
            let new_ray = Ray::new(exit_point, camera_ray.direction).shift_source();
            total_color += transparency.alpha * launch_ray(&new_ray, scene, depth + 1)?;
        }
    }

    // Reflexion
    if let Some(mirror) = &nearest_object.effects().mirror {
        let surface_normal = nearest_object
            .normal_at(collision_point)
            .ok_or_else(|| String::from("No normal found"))?
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

fn search_object_collision<'a>(
    ray: &Ray,
    objects: &'a [Box<dyn AnySceneObject>],
) -> Option<(&'a dyn AnySceneObject, Vec3)> {
    let mut shortest_distance: f64 = f64::MAX;
    let mut nearest_object_opt: Option<&Box<dyn AnySceneObject>> = None;
    let mut collision_point: Vec3 = Default::default();
    // For each pixel, we search for collision with objects
    // We also take into account the nearest object, for now
    for object_candidate in objects {
        if let Some(collision_point_candidate) = object_candidate.check_collision(&ray) {
            let distance = collision_point_candidate.distance(ray.source);
            if distance <= 1e-12 {
                continue;
            } else if distance < shortest_distance {
                shortest_distance = distance;
                nearest_object_opt = Some(object_candidate);
                collision_point = collision_point_candidate;
            }
        }
    }
    match nearest_object_opt {
        Some(nearest_object) => Some((&**nearest_object, collision_point)),
        _ => None,
    }
}

fn illumination_from_lights(
    object: &dyn AnySceneObject,
    surface_point: Vec3,
    lights: &[Box<dyn AnyLightObject>],
    objects: &[Box<dyn AnySceneObject>],
    camera_ray: &Ray,
) -> Result<Color, String> {
    let mut total_color = Color::BLACK;
    for current_light in lights {
        let light_ray = Ray::ray_from_to(surface_point, current_light.source());

        // Generate shadow, by skipping process if there is an obstacle between object and light
        if ray_encounter_obstacle(&light_ray, &current_light.source(), objects) {
            continue;
        }

        // Build values needed for light computation
        let light_direction = light_ray.direction;
        let light_color = current_light.light_color_at(surface_point);
        let surface_normal = object
            .normal_at(surface_point)
            .ok_or_else(|| String::from("No normal found"))?;
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
