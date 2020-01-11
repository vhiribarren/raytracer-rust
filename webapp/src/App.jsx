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

import 'antd/dist/antd.css';

import React from 'react';
import ReactDOM from 'react-dom';
import SplitPane from 'react-split-pane'
import { Layout, PageHeader, Button, Icon, Input, Form, Radio, Switch, Select, Progress, InputNumber } from 'antd';

const { Header, Footer, Sider, Content } = Layout;
const { TextArea } = Input;

export class App extends React.Component {

  constructor(props) {
    super(props);
    this.state = {
      collapsed: false,
    };
  }

  render() {
    return (
      <div>
        <Layout>
          <Header
            style={{ background: '#fff', position: 'fixed', zIndex: 1, width: "100%", boxShadow: "0 2px 8px #f0f1f2", padding: "0 5px" }}>
            <PageHeader
              title="Rust Ray Tracer"
              subTitle="A Hobby Project"
              extra={[
                <Progress key="progress" percent={50} status="active" style={{display: "inline-block", width:"300px", marginRight: "20px"}} />,
                <Icon
                  className="trigger"
                  key="icon"
                  type={this.state.collapsed ? 'menu-unfold' : 'menu-fold'}
                  onClick={this.toggle}
                />,
                <Button key="button" type="primary">Render</Button>,
              ]}
            ></PageHeader>
          </Header>
          <Content style={{ background: '#fff', marginTop: 64, marginBottom: 64, height:"calc(100vh - 132px)" }}>
            <SplitPane split="vertical" minSize="40%" style={{ position: "relative" }}>
              <TextArea autoSize={false} style={{ padding: "10px", resize: "none", height:"calc(100vh - 132px)" }} />
              <div style={{ background: '#aaa', marginLeft: "5px", height: "100%"}}><canvas id="canvas" style={{padding: 0, margin: "auto", display: "block", verticalAlign: "middle"}}/></div>
            </SplitPane>
          </Content>
          <Sider trigger={null} collapsible style={{ background: "#fff", marginTop: 64, marginBottom:0,  padding: "10px", overflow: "auto", height:"calc(100vh - 132px)" }}>
            <Form layout="vertical">
            {/*
              <Form.Item label="Progressive Rendering">
                <Switch />
              </Form.Item>
              <Form.Item label="Parallel Rendering">
                <Switch />
              </Form.Item>
            */}
              <Form.Item label="Width" style={{padding: 0}}>
                <Input placeholder="input placeholder" />
              </Form.Item>
              <Form.Item label="Height" style={{padding: 0}}>
                <Input placeholder="input placeholder" />
              </Form.Item>
              <Form.Item label="Rendering strategy" hasFeedback style={{padding: 0}}>
                <Select placeholder="Please select a strategy">
                  <Option value="china">Normal</Option>
                  <Option value="usa">Random</Option>
                </Select>
        </Form.Item>
              <Form.Item
                label="Number of rays"
                style={{padding: 0}}
              >
                <InputNumber min={8} max={12} />
              </Form.Item>
            </Form>
          </Sider>
          <Footer style={{ textAlign: 'center', position: 'fixed', bottom: 0, width: "100%" }}>Copyright (c) 2020 Vincent Hiribarren</Footer>
        </Layout>
      </div>
    );
  }
}
