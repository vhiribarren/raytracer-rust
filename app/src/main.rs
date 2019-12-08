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

mod utils;

use raytracer::renderer::DrawCanvas;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use crate::utils::result::RaytracingResult;
use log::info;
use raytracer::cameras::PerspectiveCamera;
use raytracer::textures::CheckedPattern;
use std::f32::consts::PI;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 576;
const CANVAS_WIDTH: u32 = 1024;
const CANVAS_HEIGHT: u32 = 576;

struct WrapperCanvas<'a>(&'a mut Canvas<Window>);

impl DrawCanvas for WrapperCanvas<'_> {
    fn draw(
        &mut self,
        x: u32,
        y: u32,
        color: &raytracer::colors::Color,
    ) -> std::result::Result<(), String> {
        let draw_color = sdl2::pixels::Color::RGB(
            (255.0 * color.red()) as u8,
            (255.0 * color.green()) as u8,
            (255.0 * color.blue()) as u8,
        );
        self.0.set_draw_color(draw_color);
        self.0
            .draw_point(sdl2::rect::Point::new(x as i32, y as i32))?;
        Ok(())
    }
}

pub fn main() -> RaytracingResult {
    stderrlog::new().verbosity(4).init()?;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("RayTracer Test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    canvas.set_logical_size(CANVAS_WIDTH, CANVAS_HEIGHT)?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    let mut wrapper_canvas = WrapperCanvas(&mut canvas);

    draw_test_scene(&mut wrapper_canvas)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn draw_test_scene(canvas: &mut impl DrawCanvas) -> RaytracingResult {
    use raytracer::colors::Color;
    use raytracer::lights::LightPoint;
    use raytracer::primitives::{InfinitePlan, Sphere};
    use raytracer::renderer::{render, RenderOptions};
    use raytracer::scene::{Scene, SceneObject};
    use raytracer::textures::PlainColorTexture;
    use raytracer::vector::Vec3;

    // let camera_orth = OrthogonalCamera::new(
    //     Vec3::new(0.0, 10.0, -15.0),
    //     Vec3::new(0.0, 0.0, 15.0),
    //     16.0 * 3.0,
    //     9.0 * 3.0,
    //);
    let camera_perspective = PerspectiveCamera::new(
        Vec3::new(0.0, 30.0, -10.0),
        Vec3::new(0.0, 0.0, 15.0),
        16.0 * 2.0,
        9.0 * 2.0,
        (PI / 10.0) as f64,
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
    let object_1 = SceneObject { primitive, texture };
    let primitive: Sphere = Sphere {
        center: Vec3::new(-10.0, 3.0, 10.0),
        radius: 8.0,
    };
    let color = Color::RED;
    let texture = PlainColorTexture { color };
    let object_2 = SceneObject { primitive, texture };
    let primitive: Sphere = Sphere {
        center: Vec3::new(10.0, 3.0, 10.0),
        radius: 8.0,
    };
    let color = Color::GREEN;
    let texture = PlainColorTexture { color };
    let object_3 = SceneObject { primitive, texture };
    //let plane = SquarePlan::new(Vec3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 40.0);
    let plane = InfinitePlan::new(Vec3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let texture = <CheckedPattern as Default>::default();
    let object_4 = SceneObject {
        primitive: plane,
        texture,
    };

    let scene: Scene = Scene {
        camera: Box::new(camera),
        lights: vec![Box::new(light_1), Box::new(light_2)],
        objects: vec![
            Box::new(object_1),
            Box::new(object_2),
            Box::new(object_3),
            Box::new(object_4),
        ],
        options: Default::default(),
    };
    let render_options = RenderOptions {
        canvas_width: CANVAS_WIDTH,
        canvas_height: CANVAS_HEIGHT,
    };
    info!("Generating test scene...");
    render(&scene, canvas, &render_options)?;
    info!("Done!");

    Ok(())
}
