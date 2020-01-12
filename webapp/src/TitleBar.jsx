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
import { PageHeader, Button, Progress, Switch } from 'antd';

export class TitleBar extends React.Component {

  constructor(props) {
    super(props);
    this.onActionConfigPanel = (e) => {
      this.props.onActionConfigPanel();
    };
    this.onActionRender = (e) => {
      this.props.onActionRender();
    };
  }

  static get defaultProps() {
    return {
      progressBarPercent: 50
    };
  }

  render() {
    const progressBar = <Progress
      className="titlebar__progress"
      key="progress"
      percent={this.props.progressBarPercent}
      status="active" />;
    const extraContent = [
      <Switch
        key="settings"
        checkedChildren="Settings"
        unCheckedChildren="Settings"
        onClick={this.onActionConfigPanel}
        checked={this.props.configPanelVisible} />,
      <Button
        key="render"
        onClick={this.onActionRender}
        type="primary">{this.props.isRendering ? "Stop" : "Render"}</Button>,
    ];
    if (this.props.isRendering) {
      extraContent.unshift(progressBar);
    }

    return (
      <PageHeader
        title="Rust Ray Tracer"
        subTitle="A Hobby Project"
        extra={extraContent}
      ></PageHeader>
    );
  }
}
