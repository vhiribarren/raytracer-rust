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
import { Input, Form, Select, InputNumber } from 'antd';

export class Config extends React.Component {

  static get defaultProps() {
    return {
      onConfigChange: (config) => {}
    };
  }

  updateConfig() {
    const config = {
      canvas_width: this.state.canvas_width,
      strategy: this.state.strategy,
      ray_number: this.state.ray_number,
    }
    this.props.onConfigChange(config);
  }

  constructor(props) {
    super(props);

    this.state = {
      strategy: "normal",
      ray_number: 50,
      canvas_width: 1024,
    };

    this.onStrategySelect = (strategy) => {
      this.setState(state => ({
        strategy
      }),
      this.updateConfig);
    };

    this.onRayNumberChange = (ray_number) => {
      if (!ray_number) {
        return;
      }
      this.setState(state => ({
        ray_number
      }),
      this.updateConfig);
    };

    this.onCanvasWidthChange = (canvas_width) => {
      if (!canvas_width) {
        return;
      }
      this.setState(state => ({
        canvas_width
      }),
      this.updateConfig);
    };

  }

  render() {
    return (
      <Form className="config__item" layout="vertical">
        <Form.Item className="config__item" label="Width">
          <InputNumber min={1} value={this.state.canvas_width} onChange={this.onCanvasWidthChange} />
        </Form.Item>
        <Form.Item className="config__item" label="Ray generation strategy">
          <Select value={this.state.strategy} onSelect={this.onStrategySelect}>
            <Select.Option value="normal">Normal</Select.Option>
            <Select.Option value="random">Random</Select.Option>
          </Select>
        </Form.Item>
        {this.state.strategy === "random" &&
          <Form.Item
            className="config__item"
            label="Number of rays">
            <InputNumber min={1} value={this.state.ray_number} onChange={this.onRayNumberChange} />
          </Form.Item>
        }

      </Form>
    );
  }
}
