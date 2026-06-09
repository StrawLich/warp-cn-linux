# StrawLich/warp-cn

这是 [Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn) 的个人维护 fork，主要用于在 **Deepin 25 (Linux x86_64)** 上构建和使用 Warp 中文社区版终端。

## 与上游的区别

| 修复 | 说明 |
|------|------|
| **i18n 初始化崩溃** | 修复 `settings_billing.ftl` 中重复的 Fluent key 导致汉化完全失效的问题 |
| **Linux IME 输入法** | 启用 Linux 上的 IME 支持（fcitx/ibus），使中文输入法正常工作 |

上游代码本身完全兼容 Linux，但这两个问题导致 Linux 上的实际使用体验受损。

## 构建

> 需要 Rust 1.92+ 和 protoc v25.1

```bash
# 安装系统依赖
sudo apt-get install -y build-essential cmake pkg-config curl git \
  libssl-dev libfreetype-dev libexpat1-dev libgit2-dev \
  libfontconfig1-dev libasound2-dev libclang-dev clang-format \
  jq brotli python-is-python3

# 编译
cargo build -p warp --profile release-lto --bin warp-oss \
  --features "release_bundle,gui,nld_classifier_v1,nld_heuristic_v1"

# 或使用仓库脚本打包 .deb
./script/linux/bundle -c oss --packages deb --release-tag v0.YYYY.MM.DD
```

## 同步上游

```bash
git fetch upstream
git checkout master
git merge upstream/master
# 解决冲突后重新构建
```

详细说明见 [CLAUDE.md](./CLAUDE.md)。

## 致谢

- [warpdotdev/warp](https://github.com/warpdotdev/warp) — Warp 终端原版
- [Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn) — 中文社区版（完整汉化 + 国产模型路由）
