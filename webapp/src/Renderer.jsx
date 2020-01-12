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

import React from 'react';
import ReactDOM from 'react-dom';
import raytracer from '../../raytracer/Cargo.toml';

export class Renderer extends React.Component {

  constructor(props) {
    super(props);
    raytracer.wasm_init();
  }

  static get defaultProps() {
    return {
      shouldRender: false,
      progressBarPercent: 50,
      onError: (msg) => {},
    };
  }

  renderScene(sceneDescription) {
    let renderer;
    try {
      renderer = raytracer.Renderer.new(sceneDescription);
    }
    catch(err) {
      console.log(err);
      this.props.onError(err);
      return;
    }
    const canvas_width = renderer.width();
    const canvas_height = renderer.height();
    const video_buffer_size = canvas_width * canvas_height * 4;

    const canvas = document.getElementById("canvas");
    canvas.width = canvas_width;
    canvas.height = canvas_height;

    const ctx = canvas.getContext('2d');

    const drawScreen = () => {
      const video_data = new Uint8ClampedArray(raytracer.wasm.memory.buffer, renderer.buffer_ptr(), video_buffer_size);
      const img = new ImageData(video_data, canvas_width, canvas_height);
      ctx.putImageData(img, 0, 0);
    }

    const renderLoop = () => {
      const loop_start = Date.now()
      while (renderer.next()) {
        if (Date.now() - loop_start > 20) {
          requestAnimationFrame(renderLoop);
          break;
        }
      }
      drawScreen();
    };

    renderLoop();
  }

  render() {
    return (
      <div className="renderer">
        <canvas className="renderer__canvas" id="canvas" />
      </div>
    );
  }

}