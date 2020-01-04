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

use raytracer::scene::Scene;
use std::fs;
use std::iter;
use std::path::PathBuf;
use std::str::FromStr;

const SAMPLES_ROOT_DIR: [&str; 2] = ["tests", "samples"];

fn load_sample<T: AsRef<str>>(sample_file: T) -> String {
    let sample_file = sample_file.as_ref();
    let scene_path: PathBuf = SAMPLES_ROOT_DIR
        .iter()
        .chain(iter::once(&sample_file))
        .collect();
    fs::read_to_string(scene_path).unwrap()
}

#[test]
fn load_basic_scene() {
    let scene_string = load_sample("ok_basic.toml");
    let scene_result = Scene::from_str(&scene_string);
    assert!(scene_result.is_ok());
}
