mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn greet() {
    alert("Hello, raytracer for wasm!");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn greet() {
    println!("Hello, raytracer without wasm!");
}
