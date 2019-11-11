use raytracer::primitives::Camera;
use raytracer::renderer::generate_rays;

fn main() {
    let camera: Camera = Default::default();
    for a in generate_rays(&camera, 2, 2) {
        println!("{:?}", a);
    }
}
