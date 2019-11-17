use raytracer::renderer::DrawCanvas;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 576;
const CANVAS_WIDTH: u32 = 640;
const CANVAS_HEIGHT: u32 = 360;

struct WrapperCanvas<'a>(&'a mut Canvas<Window>);

impl DrawCanvas for WrapperCanvas<'_> {
    fn draw(&mut self, x: u32, y: u32, color: &raytracer::textures::Color) {
        let draw_color = sdl2::pixels::Color::RGB(
            (255.0 * color.red) as u8,
            (255.0 * color.green) as u8,
            (255.0 * color.blue) as u8,
        );
        self.0.set_draw_color(draw_color);
        self.0
            .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
            .unwrap();
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("RayTracer Test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_logical_size(CANVAS_WIDTH, CANVAS_HEIGHT)
        .unwrap();
    let mut wrapper_canvas = WrapperCanvas(&mut canvas);
    draw_test_scene(&mut wrapper_canvas);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
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
    }
}

fn draw_test_scene(canvas: &mut impl DrawCanvas) {
    use raytracer::primitives::{Sphere, Vec3};
    use raytracer::scene::{Camera, Scene, SceneObject, SceneObjectStruct};
    use raytracer::textures::{Color, Texture};
    use raytracer::renderer::{render, RenderOptions};

    let camera: Camera = Default::default();
    let primitive: Sphere = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 8.0,
    };
    let color = Color {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };
    let texture = Texture { color };
    let object = SceneObjectStruct { primitive, texture };
    let mut objects: Vec<Box<dyn SceneObject>> = Vec::new();
    objects.push(Box::new(object));

    let scene: Scene = Scene { camera, objects };
    let render_options = RenderOptions {
        canvas_width: CANVAS_WIDTH,
        canvas_height: CANVAS_HEIGHT,
    };
    render(&scene, canvas, &render_options);
}
