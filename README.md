# Imagen

一方本地优先的 AI 图像创作桌面应用 —— Vue 3 + TypeScript 前端，Tauri 2 + Rust 本地后端，支持 OpenAI / OpenAI 兼容图像接口的真实生成，也内置不联网的模拟模式。

[English README → README_EN.md](README_EN.md)

## 功能

- **生成**：提示词 + 负面提示词，选择服务商 / 模型 / 方向与宽高比（横屏 16:9、3:2、4:3；竖屏 9:16、2:3、3:4；方形 1:1）/ 分辨率（1K–4K）/ 质量 / 数量（1–4），`Ctrl + Enter` 生成，支持取消、失败重试。
- **任务队列**：生成中再次提交会排队（并发 1，上限 8），Settings → 任务 查看状态或单独取消；重启后未完成的任务标记为「已中断」。
- **历史**：搜索、模型筛选、收藏、网格 / 列表视图、按日期分组；复用提示词与参数；详情弹窗展示完整参数与参考图；可打开所在文件夹，导出走系统另存为对话框。
- **预设**：应用、保存、编辑、删除；删除前需确认。
- **服务商**：多 Provider（OpenAI / 兼容 / Custom），从 `{baseUrl}/models` 拉取模型列表，连接测试与生成请求分开；按协议能力禁用不支持的参数（Seed、引导强度、负面提示词、参考图仅记录）。
- **参考图**：上传 ≤3 张 PNG / JPEG / WebP（每张 ≤10 MB），可设参考强度；随生成记录保存。
- **主题**：Light / Dark / System，默认跟随系统。
- **语言**：Settings → 外观 切换 中文 / English，默认中文，选择保存在 localStorage。

## 本地数据

桌面端数据保存在应用数据目录（`%APPDATA%\com.imagen.studio\`）：

- `imagen.db` — SQLite（WAL 模式）：服务商配置、历史记录、预设、任务、会话状态。
- `images/` — 生成的原图；`thumbs/` — 384px 缩略图（历史网格用）。
- API Key 存 **Windows 凭据管理器**（service `com.imagen.studio`），数据库与日志不记录明文。

浏览器开发模式下数据在 IndexedDB；首次以桌面端启动时自动迁移到 SQLite。清理站点数据会移除浏览器端内容。

## 运行

需要 Node.js 22.12+（建议 24 LTS）与 npm。

```sh
npm install
npm run dev        # http://127.0.0.1:1420 ，浏览器内为模拟模式
```

## 桌面应用（Tauri 2 + Rust）

需要 Rust 工具链（stable-x86_64-pc-windows-msvc）与 WebView2。

```sh
npm run desktop    # tauri dev，自动复用已启动的 Vite
npx tauri build    # 产出 MSI + NSIS 安装包（src-tauri/target/release/bundle/）
```

桌面模式下前端通过统一接口调用 Rust 命令：`generate_images`、`cancel_generation`、`test_connection`、SQLite 工作区读写与 keyring 密钥管理。Rust 端按 OpenAI Images API 兼容协议向 Provider 发送 `POST {baseUrl}/images/generations`，图片写入 `images/` 后经 asset 协议回显。

## 测试

```sh
npm test                    # 前端参数校验 / Mock 场景（node --test）
cd src-tauri && cargo test  # 本地模拟 Provider 端到端：成功、限流、错误、损坏图片
```

`scripts/` 内含辅助工具：`i18n-check.mjs`（双语词条完整性检查）、`db-inspect.py`（查看 SQLite 内容）、`dev-or-attach.mjs`（复用已启动的 Vite dev server）。

## 设计

目标分辨率 1080p / 1440p 桌面窗口；窄窗口下侧栏收缩为图标、参数面板收为抽屉。页面资源使用相对路径（`base: "./"`），不依赖 CDN。
