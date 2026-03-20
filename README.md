# Scroll Video Tauri

一个基于 Tauri 2 的桌面小工具，用来把长文案生成滚屏字幕预览，并导出为 `WebM` 视频。

应用界面提供文案输入、字体风格、颜色、字号、行高、滚动速度、背景风格、背景音乐等调节项。预览渲染在 `canvas` 上，录制时通过浏览器 `MediaRecorder` 采集视频流，并在 Tauri 桌面端通过 Rust 命令弹出保存对话框，把视频保存到本地。

## 功能特性

- 输入文案并实时生成滚屏字幕预览
- 支持调整字体、字号、行高、段落间距与滚动速度
- 支持主字颜色和强调色配置
- 支持背景风格切换和自定义背景图片
- 支持导入背景音乐并一起录制导出
- 在 Tauri 桌面端导出 `WebM` 视频到本地

## 技术栈

- Tauri 2
- Rust
- 原生 HTML / CSS / JavaScript
- Canvas
- MediaRecorder

## 项目结构

```text
.
├── src/
│   └── index.html        # 单文件前端界面与预览/录制逻辑
├── src-tauri/
│   ├── src/lib.rs        # Tauri 启动入口与 save_video 命令
│   ├── Cargo.toml        # Rust 依赖
│   └── tauri.conf.json   # Tauri 配置
├── package.json
└── README.md
```

## 本地开发

先确保本机已经安装：

- Node.js 18+
- Rust 工具链
- Tauri 2 所需的系统依赖

安装依赖：

```bash
npm install
```

启动开发环境：

```bash
npm run dev
```

打包构建：

```bash
npm run build
```

## 使用说明

1. 在左侧输入要滚动展示的文案。
2. 调整字体、颜色、速度、背景和音乐等参数。
3. 点击生成预览并检查滚动效果。
4. 点击“录制视频（WebM）”开始录制。
5. 再次点击停止录制，选择保存路径后导出视频。

## 导出说明

- 导出格式为 `WebM`
- 桌面端通过 Tauri `invoke` 调用 Rust 后端保存文件
- 如果在普通浏览器环境中打开页面，会回退为直接下载 `WebM`

## 备注

这个项目目前以前端单文件页面为主，适合快速原型和轻量桌面工具场景。如果后续功能继续增长，可以考虑把前端拆成独立模块或接入框架化构建流程。
