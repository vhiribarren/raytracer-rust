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

  render() {
    return (
      <Form className="config__item" layout="vertical">
        {/*
          <Form.Item className="config__item" label="Progressive Rendering">
            <Switch />
          </Form.Item>
          <Form.Item className="config__item" label="Parallel Rendering">
            <Switch />
          </Form.Item>
        */}
        <Form.Item className="config__item" label="Width">
          <Input placeholder="input placeholder" />
        </Form.Item>
        <Form.Item className="config__item" label="Height">
          <Input placeholder="input placeholder" />
        </Form.Item>
        <Form.Item className="config__item" label="Rendering strategy" hasFeedback>
          <Select placeholder="Please select a strategy">
            <Option value="china">Normal</Option>
            <Option value="usa">Random</Option>
          </Select>
        </Form.Item>
        <Form.Item
          className="config__item"
          label="Number of rays">
          <InputNumber min={8} max={12} />
        </Form.Item>
      </Form>
    );
  }
}
