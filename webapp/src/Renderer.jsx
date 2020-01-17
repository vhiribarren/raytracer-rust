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
    this.shouldStop = false;
    this.progressPercent = 0;
    this.progressCurrent = 0;
    this.progressMax = 0;
  }

  static get defaultProps() {
    return {
      shouldRender: false,
      progressBarPercent: 0,
      onError: (msg) => {},
      onSuccess: (time) => {},
      onChange: (isRendering) => {},
      onPercentProgression: (percent) => {},
    };
  }

  stop() {
    this.shouldStop = true;
  }

  updateProgression() {
    this.progressCurrent += 1;
    const percentProgress = Math.trunc((100*(this.progressCurrent/this.progressMax)));
    if (percentProgress != this.progressPercent) {
      this.progressPercent = percentProgress;
      this.props.onPercentProgression(percentProgress);
    }
  }

  renderScene(sceneDescription, config) {
    console.log("Config for rendering: ", config);
    this.shouldStop = false;
    let renderer;
    try {
      renderer = raytracer.Renderer.new(sceneDescription, config ? config : {});
    }
    catch(err) {
      console.log(err);
      this.props.onError(err);
      return;
    }
    this.progressMax = renderer.width()*renderer.height();
    this.progressPercent = 0;
    this.progressCurrent = 0;
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
      let hasNext;
      while (hasNext = renderer.next()) {
        this.updateProgression();
        if (Date.now() - loop_start > 20) {
          if (this.shouldStop) {
            hasNext = false;
          }
          else {
            requestAnimationFrame(renderLoop);
          }
          break;
        }
      }
      if (!hasNext) {
        this.props.onSuccess(Date.now() - startDate);
        this.props.onChange(false);
      }
      drawScreen();
    };

    this.props.onChange(true);
    const startDate = Date.now();
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