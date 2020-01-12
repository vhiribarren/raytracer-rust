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
import SplitPane from 'react-split-pane'
import { Layout, Input, notification } from 'antd';
import { TitleBar } from "./TitleBar";
import { Config } from "./Config";
import { Renderer } from "./Renderer";

const { Header, Footer, Sider, Content } = Layout;
const { TextArea } = Input;

import { readFileSync } from "fs";
const sample_scene = readFileSync("../samples/show_room_1.toml", 'utf8')

export class App extends React.Component {

  constructor(props) {
    super(props);
    this.state = {
      showConfigPanel: false,
      sceneDescription: sample_scene
    };
    this.rendererRef = React.createRef();

    this.onActionConfigPanel = (e) => {
      this.setState(state => ({
        showConfigPanel: !state.showConfigPanel
      }));
    };

    this.onActionRender = (e) => {
      this.rendererRef.current.renderScene(this.state.sceneDescription);
    };

    this.onEditorChange = (e) => {
      e.persist();
      this.setState(state => ({
        sceneDescription: e.target.value
      }));
    }

    this.onRendererError = (msg) => {
      this.openNotification(msg);
    }
  }

  openNotification(msg) {
    notification.error({
      message: 'Rendering error',
      description: msg,
      duration: 0,
      top: 70,
    });
  }

  render() {
    return (
      <div>
        <Layout>
          <Header className="header">
            <TitleBar
              onActionRender={this.onActionRender}
              onActionConfigPanel={this.onActionConfigPanel}
              configPanelVisible={this.state.showConfigPanel}
              progressBarVisible={false} />
          </Header>
          <Content className="content">
            <SplitPane className="content__split" split="vertical" minSize="40%">
              <div>
                <TextArea className="editor" autoSize={false} value={this.state.sceneDescription} onChange={this.onEditorChange} />
              </div>
              <Renderer ref={this.rendererRef} onError={this.onRendererError}/>
            </SplitPane>
          </Content>
          <Sider className="sider" trigger={null} collapsible collapsed={!this.state.showConfigPanel} collapsedWidth={0}>
            <Config />
          </Sider>
          <Footer className="footer">Copyright (c) 2020 Vincent Hiribarren</Footer>
        </Layout>
      </div>
    );
  }
}
