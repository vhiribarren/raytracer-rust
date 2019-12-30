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

import { wasm_init, Renderer } from '../raytracer/src/lib.rs';
//import { memory } from "../raytracer/pkg/raytracer";

wasm_init();

//console.log(memory);

//class ScreenSurface {
//
//    init(memory_buffer, memory_ptr, surface_width, surface_height) {
//        this.video_buffer = new Uint8Array(memory.buffer, renderer.buffer_ptr(), surface_width * surface_height * 3);
//        this.width = surface_width;
//        this.height = surface_height;
//    }
//
//    color_at(x, y) {
//        const index = (y * this.width + x) * 3;
//        const r = this.video_buffer[index];
//        const g = this.video_buffer[index + 1];
//        const b = this.video_buffer[index + 2];
//        return `rgb(${r},${g},${b})`;
//    }
//
//}

const renderer = Renderer.new();
//const screenSurface = new ScreenSurface(memory.buffer, renderer.buffer_ptr(), renderer.width(), renderer.height());

const canvas = document.getElementById("canvas");
canvas.height = renderer.height();
canvas.width = renderer.width();

const ctx = canvas.getContext('2d');

const instant_start = Date.now();
const renderLoop = () => {

    const loop_start = Date.now()
    while(true) {
        const pixel = renderer.next();
        if (pixel == undefined) {
            const duration = Date.now() - instant_start;
            console.log(`Rendering duration: ${duration/1000}s`)
            return;
        }
        ctx.fillStyle = `rgb(${pixel.r},${pixel.g},${pixel.b})`;
        ctx.fillRect( pixel.x, pixel.y, 1, 1 );
        if (Date.now() - loop_start > 50) {
            requestAnimationFrame(renderLoop);
            return;
        }
    }
};

renderLoop();


/*
  for(let y=0; y<renderer.height(); y++) {
    for(let x=0; x<renderer.width(); x++) {
        ctx.fillStyle = screenSurface.color_at(x, y);
        ctx.fillRect( x, y, 1, 1 );
    }
  }
*/

  //requestAnimationFrame(renderLoop);
