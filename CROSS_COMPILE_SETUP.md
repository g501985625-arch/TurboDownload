# Linux交叉编译设置指南

要在macOS上为Linux构建TurboDownload应用，需要以下设置：

## 方法一：使用Docker（推荐）

1. 安装Docker Desktop for Mac
2. 使用官方Rust Docker镜像进行构建：
   ```bash
   cd ~/.openclaw/workspace/projects/TurboDownload
   docker run --rm -v "$(pwd)":/usr/src/myapp -w /usr/src/myapp rust:latest sh -c "apt-get update && apt-get install -y musl-tools libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev && rustup target add x86_64-unknown-linux-gnu && cargo tauri build --target x86_64-unknown-linux-gnu"
   ```

## 方法二：本地交叉编译设置

1. 安装Cross工具：
   ```bash
   cargo install cross
   ```

2. 使用cross进行构建：
   ```bash
   cd ~/.openclaw/workspace/projects/TurboDownload
   cross build --target x86_64-unknown-linux-gnu --release
   ```

3. 然后手动打包为deb文件：
   ```bash
   cargo tauri build --target x86_64-unknown-linux-gnu
   ```

## 方法三：使用虚拟机

在Linux虚拟机或通过WSL（如果在Windows上）中进行原生构建。

## 当前状态

当前环境中缺少必要的Linux交叉编译工具链，无法直接在macOS上构建Linux版本。建议使用Docker方法进行构建。