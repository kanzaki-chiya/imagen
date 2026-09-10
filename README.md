# Imagen

一个本地优先的 AI 图像创作前端，使用 Vue 3、TypeScript 和 Vite。以图像画布为中心，提供 Generate、History、Presets 和 Settings 四个工作区，以及完整的 Light / Dark / System 主题。

## 启动

需要 Node.js 22.12+（或 24 LTS）和 npm。

```sh
npm install
npm run dev
```

访问 `http://127.0.0.1:1420`。

```sh
npm run build   # TypeScript 检查与生产构建
npm run preview # 预览生产构建
npm test        # 参数与 Mock 服务校验
cd src-tauri && cargo test  # 本地模拟 Provider 端到端校验（成功/限流/错误/损坏图片）
```

## 桌面应用（Tauri 2 + Rust）

需要 Rust 工具链（stable-x86_64-pc-windows-msvc）与 WebView2。

```sh
npm run desktop   # 等价于 npx tauri dev，自动先启动 Vite
```

桌面模式下生成走 Rust 后端：前端通过统一接口调用 `generate_images` 命令，由 Rust 以 OpenAI Images API 兼容协议请求 Provider（POST `{baseUrl}/images/generations`），生成的图片写入应用数据目录 `…/com.imagen.studio/images/`，再经 asset 协议回显到画布。API Key 随每次请求传递给 Rust，不落盘、不进持久存储。

- **Mock / Real 切换**：Settings → Workspace → Always use mock generation；浏览器环境始终为 Mock。
- **真实连接测试**：Provider 编辑器在桌面模式下请求 `{baseUrl}/models` 并返回模型数与延迟。
- **取消**：点击取消会向 Rust 发送 `cancel_generation`，中断进行中的请求并清理未完成的文件。
- **错误结构**：`{ kind: config | auth | rate_limit | network | server | invalid_response | cancelled, message, status? }`。
- **OpenAI 参数映射**：1:1→1024×1024、3:2 与 16:9→1536×1024、2:3→1024×1536；Quality 映射 gpt-image（low/medium/high/auto）与 dall-e（standard/hd）。seed、guidance、negative prompt 与参考图会被记录但不发送。

## 使用

- **Generate**：编辑 Prompt / Negative prompt，选择 Provider、模型、比例、分辨率、质量、图片数量及种子，点击生成或按 `Ctrl + Enter`。支持取消与失败重试。
- **画布**：切换输出、收藏、缩放、双图比较、全屏查看、检查元数据，以及导出 PNG / JPEG / WebP。聚焦画布后可用方向键切换。
- **参考图**：点击上传或拖入 PNG、JPEG、WebP，最多 3 张，每张不超过 10 MB，可调整参考强度。
- **History**：按 Prompt 搜索、模型筛选、收藏过滤和时间排序；支持网格 / 列表视图，并可恢复历史 Prompt、参数和参考图。
- **Presets**：应用、创建、编辑和删除常用提示词及参数模板。删除前需确认。
- **Settings**：编辑 OpenAI、OpenAI Compatible、Gemini 和 Custom Provider，支持自定义模型与模拟连接测试。API Key 只在当前会话内存中保留，不写入持久存储。
- **主题**：顶部随时切换 Light / Dark / System，默认跟随系统。
- **语言**：Settings → Appearance 切换 中文 / English，默认中文，选择会保存在 localStorage。
- **任务**：生成中再次生成会进入队列（并发 1，上限 8）；Settings → 任务查看状态或取消，重启后未完成任务标记为「已中断」。
- **本地数据**：桌面端保存在应用数据目录 `imagen.db`（SQLite）；API Key 存系统凭据管理器，不落盘。浏览器端仍在 IndexedDB。
- **文件**：桌面端历史详情可「打开所在文件夹」，下载走原生另存为对话框。
- **快捷键**：`Ctrl + Alt + 1/2/3/4` 切换工作区，`?` 查看快捷键。

## Mock 模式

所有 Provider 和图像生成均为模拟，不发送真实 API 请求，也不需要真实 API Key。四张随应用提供的 AI 演示图可离线使用。Prompt 的场景词选择样图，Seed 改变色彩处理；比例、分辨率、文件格式作用于导出图片。质量、Guidance、Negative prompt 与参考图会被记录，但不调用 AI 模型。

在 **Settings → Workspace → Simulate a failed generation** 中，可以让下一次生成模拟超时。重试随后正常完成。在 Provider Base URL 中加入 `fail` 可测试连接失败状态。

桌面端数据在应用数据目录（`imagen.db` SQLite + `images/` + `thumbs/`）；API Key 在系统凭据存储。浏览器端为 IndexedDB，首次启动桌面端会自动迁移。主题偏好保存在 localStorage。清理站点数据会移除浏览器内容。存储不可用时会显示提示，当前会话仍可使用。

## 桌面与部署

主要适配 1080p / 1440p 桌面窗口；较窄窗口下导航缩为图标，参数面板变为可打开的抽屉。页面采用本地 hash 导航、相对静态资源路径，不依赖 CDN 或服务端路由。

生产静态文件输出到 `dist/`。桌面外壳位于 `src-tauri/`（Tauri 2 + Rust）；`npx tauri build` 产出 Windows 安装包（暂未验证）。
