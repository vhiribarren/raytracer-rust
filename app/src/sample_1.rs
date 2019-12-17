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

use raytracer::cameras::PerspectiveCamera;
use raytracer::colors::Color;
use raytracer::lights::LightPoint;
use raytracer::primitives::{InfinitePlan, Sphere};
use raytracer::scene::{Scene, SceneConfiguration, SceneObject};
use raytracer::textures::{
    CheckedPattern, Mirror, PlainColorTexture, TextureEffects, Transparency,
};
use raytracer::vector::Vec3;
use std::f64::consts::PI;

pub(crate) fn generate_test_scene() -> Scene {
    // let camera_orth = OrthogonalCamera::new(
    //     Vec3::new(0.0, 10.0, -15.0),
    //     Vec3::new(0.0, 0.0, 15.0),
    //     16.0 * 3.0,
    //     9.0 * 3.0,
    //);
    let camera_perspective = PerspectiveCamera::new(
        Vec3::new(0.0, 10.0, -10.0),
        Vec3::new(0.0, 0.0, 30.0),
        16.0 * 2.0,
        9.0 * 2.0,
        (PI / 8.0) as f64,
    );
    //let camera = camera_orth;
    let camera = camera_perspective;
    let light_1 = LightPoint::with_color(Vec3::new(50.0, 100.0, -50.0), Color::new(0.8, 0.8, 0.8));
    let light_2 = LightPoint::with_color(Vec3::new(-50.0, 20.0, -20.0), Color::new(0.8, 0.0, 0.0));
    let primitive: Sphere = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5.0,
    };
    let texture = <CheckedPattern as Default>::default();
    let object_1 = SceneObject {
        primitive,
        texture,
        effects: TextureEffects {
            phong: Some(Default::default()),
            ..Default::default()
        },
    };
    let primitive: Sphere = Sphere {
        center: Vec3::new(-10.0, 3.0, 10.0),
        radius: 8.0,
    };
    let color = Color::RED;
    let texture = PlainColorTexture { color };
    let object_2 = SceneObject {
        primitive,
        texture,
        effects: TextureEffects {
            phong: Some(Default::default()),
            mirror: Some(Mirror { coeff: 1.0 }),
            ..Default::default()
        },
    };
    let primitive: Sphere = Sphere {
        center: Vec3::new(10.0, 3.0, 10.0),
        radius: 8.0,
    };
    let color = Color::GREEN;
    let texture = PlainColorTexture { color };
    let object_3 = SceneObject {
        primitive,
        texture,
        effects: TextureEffects {
            phong: Some(Default::default()),
            transparency: Some(Transparency {
                refractive_index: 1.3,
                ..Default::default()
            }),
            ..Default::default()
        },
    };
    let primitive: Sphere = Sphere {
        center: Vec3::new(0.0, 10.0, 35.0),
        radius: 15.0,
    };
    let color = Color::YELLOW;
    let texture = PlainColorTexture { color };
    let object_4 = SceneObject {
        primitive,
        texture,
        effects: TextureEffects {
            phong: Some(Default::default()),
            transparency: Some(Transparency {
                refractive_index: 1.3,
                ..Default::default()
            }),
            ..Default::default()
        },
    };
    //let plane = SquarePlan::new(Vec3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 40.0);
    let plane = InfinitePlan::new(Vec3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let texture = <CheckedPattern as Default>::default();
    let object_5 = SceneObject {
        primitive: plane,
        texture,
        effects: TextureEffects {
            mirror: Some(Mirror { coeff: 0.8 }),
            ..Default::default()
        },
    };

    Scene {
        camera: Box::new(camera),
        lights: vec![Box::new(light_1), Box::new(light_2)],
        objects: vec![
            Box::new(object_1),
            Box::new(object_2),
            Box::new(object_3),
            Box::new(object_4),
            Box::new(object_5),
        ],
        config: SceneConfiguration {
            ambient_light: Some(Color::new(0.0, 0.0, 0.2)),
            maximum_light_recursion: 2,
            ..Default::default()
        },
    }
}
