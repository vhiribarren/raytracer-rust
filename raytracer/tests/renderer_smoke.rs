/*
MIT License

Copyright (c) 2019, 2020 Vincent Hiribarren

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

use raytracer::renderer::{render_scene, RenderConfiguration};
use raytracer::scene::Scene;

mod samples;

#[test]
#[should_panic]
fn scene_without_lights_is_error() {
    let scene = Scene {
        lights: Vec::new(),
        ..samples::generate_test_scene()
    };
    let config = <RenderConfiguration as Default>::default();
    render_scene(scene, config, false).unwrap().count();
}

#[test]
fn smoke_sequential_rendering() {
    let scene = samples::generate_test_scene();
    let config = <RenderConfiguration as Default>::default();
    let expected_count = (config.canvas_height * config.canvas_width) as usize;
    let render_iter = render_scene(scene, config, false).unwrap();
    let count = render_iter.count();
    assert_eq!(count, expected_count);
}

#[test]
fn smoke_parallel_rendering() {
    let scene = samples::generate_test_scene();
    let config = <RenderConfiguration as Default>::default();
    let expected_count = (config.canvas_height * config.canvas_width) as usize;
    let render_iter = render_scene(scene, config, true).unwrap();
    let count = render_iter.count();
    assert_eq!(count, expected_count);
}
