fn main() {
    test_scene();
}

fn test_scene() {
    use raytracer::primitives::{Camera, Sphere, Vec3, Collision};
    use raytracer::renderer::{Scene, StdoutCanvas, render, RenderOptions};

    let camera: Camera = Default::default();
    let sphere: Sphere = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 8.0,
    };
    let mut objects: Vec<Box<dyn Collision>> = Vec::new();
    objects.push(Box::new(sphere));

    let scene: Scene = Scene { camera, objects };
    let canvas = StdoutCanvas;
    let render_options = RenderOptions { canvas_width: 16, canvas_height: 9 };
    render(&scene, &canvas, &render_options);
}
