use crate::scene::Scene;
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
