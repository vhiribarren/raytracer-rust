/*
MIT License

Copyright (c) 2020 Vincent Hiribarren

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

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

mod samples;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use raytracer::renderer::RenderConfiguration;
use raytracer::wasm::JsConfig;
use wasm_bindgen::JsValue;


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn smoke_wasm_rendering() {
    let scene_toml = include_str!("samples/ok_basic.toml");
    let config = JsValue::from_serde(&<JsConfig as Default>::default()).unwrap();
    let mut renderer = raytracer::wasm::Renderer::new(&scene_toml, config).unwrap();
    let expected_count = (renderer.width() * renderer.height()) as usize;
    let count = {
        let mut i = 0;
        loop {
            if ! renderer.next() {
                break;
            }
            i += 1;
        }
        i
    };
    assert_eq!(count, expected_count);
}