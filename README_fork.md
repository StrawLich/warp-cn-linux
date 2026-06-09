# StrawLich/warp-cn-linux

[Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn) 的 Linux 自维护构建。

warp-cn 是 [Warp 终端](https://github.com/warpdotdev/warp) 的中文社区 fork，提供完整 UI 汉化和国产模型路由（Direct LLM Backend）。但官方 Release 仅提供 macOS 和 Windows 构建产物，**不提供 Linux 版本**。

本项目基于 warp-cn 源码，在 Linux 上从源码构建并修复了 Linux 特有的问题，使其在 Linux 桌面环境下可用。

## 与上游的区别

| 修复 | 说明 |
|------|------|
| **Linux IME 输入法** | 上游的 IME 代码仅对 macOS/Windows 启用。本构建在 Linux 上启用 `ImeMarkedText` 功能标记并调用 `set_ime_allowed(true)`，使 fcitx/ibus 等中文输入法正常工作 |
| **i18n 初始化崩溃** | 上游 `settings_billing.ftl` 中存在重复的 Fluent key，导致 i18n 初始化静默失败，所有 UI 文本显示为 `{key}` 占位符。本构建移除了重复条目 |

上游代码本身完全兼容 Linux（i18n、Direct LLM Backend、安全存储均为跨平台实现），仅缺少上述两处平台适配。

## 构建

> 需要 Rust 1.92+ 和 protoc v25.1

```bash
# 安装系统依赖（Debian/Ubuntu/Deepin）
sudo apt-get install -y build-essential cmake pkg-config curl git \
  libssl-dev libfreetype-dev libexpat1-dev libgit2-dev \
  libfontconfig1-dev libasound2-dev libclang-dev clang-format \
  jq brotli python-is-python3

# 编译 OSS 版本
cargo build -p warp --profile release-lto --bin warp-oss \
  --features "release_bundle,gui,nld_classifier_v1,nld_heuristic_v1"

# 或使用仓库脚本打包 .deb
./script/linux/bundle -c oss --packages deb --release-tag v0.YYYY.MM.DD
```

## Direct LLM Backend

warp-cn 内置的国产模型路由功能，允许直连 LLM 提供商，绕过 Warp 云服务：

- **支持提供商:** OpenAI / Anthropic / Google Gemini / OpenAI Compatible（DeepSeek、Qwen、GLM、SiliconFlow 等）
- **配置方式:** Warp 设置 → AI → Direct Backend
- **无需登录** 即可使用 AI 功能

## 同步上游

本仓库定期从 [Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn) 同步更新。

## 致谢

- [warpdotdev/warp](https://github.com/warpdotdev/warp) — Warp 终端原版
- [Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn) — 中文社区版（完整汉化 + 国产模型路由）
