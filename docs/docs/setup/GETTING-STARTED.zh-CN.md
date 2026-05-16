# 快速开始

> build-your-own-tools 的构建和贡献快速入门指南

[English](GETTING-STARTED.md) | **简体中文**

---

## 目录

- [前置条件](#前置条件)
- [安装](#安装)
- [首次构建](#首次构建)
- [开发工作流](#开发工作流)
- [理解代码](#理解代码)
- [运行测试](#运行测试)
- [调试](#调试)
- [故障排除](#故障排除)
- [下一步](#下一步)

---

## 前置条件

### 必需工具

| 工具 | 版本 | 用途 | 安装链接 |
|------|---------|---------|--------------|
| Rust | 1.70+ | Rust 编译 | [rustup.rs](https://rustup.rs/) |
| Go | 1.21+ | Go 编译 | [golang.org](https://golang.org/dl/) |
| make | 任意 | 构建自动化 | 通常预装 |
| git | 2.0+ | 版本控制 | [git-scm.com](https://git-scm.com/) |

### 验证安装

```bash
# 检查 Rust
rustc --version
# 输出: rustc 1.80.0 (051478957 2024-07-21)

cargo --version
# 输出: cargo 1.80.0 (376290515 2024-07-16)

# 检查 Go
go version
# 输出: go version go1.23.0 linux/amd64

# 检查 make
make --version
# 输出: GNU Make 4.3
```

### 可选工具

| 工具 | 用途 | 安装 |
|------|---------|---------|
| `gh` | 用于发布的 GitHub CLI | `brew install gh` 或 [cli.github.com](https://cli.github.com/) |
| `jq` | JSON 处理 | `brew install jq` 或 `apt install jq` |
| `hyperfine` | 基准测试 | `cargo install hyperfine` |

---

## 安装

### 1. 克隆仓库

```bash
# HTTPS
git clone https://github.com/LessUp/build-your-own-tools.git

# 或 SSH
git clone git@github.com:LessUp/build-your-own-tools.git

cd build-your-own-tools
```

### 2. 验证项目结构

```bash
ls -la
```

预期输出:
```
build-your-own-tools/
├── dos2unix/          # CRLF 转换器
├── gzip/              # 压缩工具
├── htop/              # 系统监控
├── docs/              # 文档
├── .github/           # CI/CD 工作流
├── Cargo.toml         # Rust 工作区
├── go.work            # Go 工作区
├── Makefile           # 构建自动化
└── README.md          # 项目概览
```

### 3. 初始化工作区

```bash
# Rust 工作区（自动获取依赖）
cargo fetch

# Go 工作区
go work sync
```

---

## 首次构建

### 构建所有项目

```bash
make build-all
```

这将:
1. 构建所有 Rust 项目（发布模式）
2. 构建所有 Go 项目
3. 将二进制文件放在相应目录中

### 构建单个项目

**dos2unix (Rust)**:
```bash
cargo build --release -p dos2unix-rust

# 二进制位置: target/release/dos2unix-rust
```

**gzip - Rust**:
```bash
cargo build --release -p rgzip

# 二进制位置: target/release/rgzip
```

**gzip - Go**:
```bash
cd gzip/go
make build

# 二进制位置: gzip/go/bin/gzip-go
```

**htop - Unix Rust**:
```bash
cargo build --release -p htop-rust

# 二进制位置: target/release/htop-unix-rust
```

**htop - Windows Rust**:
```bash
cargo build --release -p htop-win-rust

# 二进制位置: target/release/htop-win-rust
```

**htop - Windows Go**:
```bash
cd htop/win/go
go build -o bin/htop-win-go ./cmd/htop-win-go

# 二进制位置: htop/win/go/bin/htop-win-go
```

### 测试你的构建

```bash
# 运行 dos2unix
echo "Hello" | ./target/release/dos2unix-rust

# 运行 gzip（先创建测试文件）
echo "Test content" > /tmp/test.txt
./target/release/rgzip /tmp/test.txt
ls -la /tmp/test.txt.gz

# 运行 htop（如果在 Unix 上）
./target/release/htop-unix-rust
```

---

## 开发工作流

### 典型开发周期

```
编辑代码 → 格式化 → Lint → 测试 → 构建 → 通过? → 提交
                                    ↓
                                  未通过 → 继续编辑
```

### 分步工作流

```bash
# 1. 修改源文件
# ...

# 2. 格式化代码
make fmt-all

# 3. 运行 linter
make lint-all

# 4. 运行测试
make test-all

# 5. 构建发布版本
make build-all
```

### 使用 Make 目标

```bash
# 构建命令
make build-all          # 构建所有项目
make build-rust         # 仅构建 Rust 项目
make build-go           # 仅构建 Go 项目
make build-dos2unix     # 构建特定项目

# 质量命令
make test-all           # 运行所有测试
make test-rust          # 仅运行 Rust 测试
make test-go            # 仅运行 Go 测试

make lint-all           # 运行所有 linter
make lint-rust          # 运行 Rust clippy
make lint-go            # 运行 go vet

make fmt-all            # 格式化所有代码
make fmt-rust           # 仅格式化 Rust
make fmt-go             # 仅格式化 Go

# 清理构建产物
make clean              # 移除构建目录
```

---

## 理解代码

### 项目复杂度进阶

```
dos2unix (⭐ 初级) → gzip (⭐⭐ 中级) → htop (⭐⭐⭐ 高级)
```

### 推荐学习路径

1. **从 dos2unix 开始**（最简单）
   - 学习文件 I/O 基础
   - 理解流式处理
   - 练习错误处理

2. **进阶到 gzip**（中级）
   - 学习压缩算法
   - 练习库设计
   - 对比 Rust vs Go

3. **完成 htop**（高级）
   - 学习 TUI 开发
   - 练习系统 API
   - 理解异步/并发

### 代码探索技巧

**导航 Rust 代码**:
```bash
# 查找所有公共函数
grep -r "^pub fn" dos2unix/src/

# 查找主入口点
grep -r "fn main" --include="*.rs"

# 列出所有测试
grep -r "#\[test\]" --include="*.rs"
```

**导航 Go 代码**:
```bash
# 查找所有函数
grep -r "^func " gzip/go/cmd/

# 查找 main 包
grep -r "package main" --include="*.go"

# 列出所有测试
grep -r "^func Test" --include="*.go"
```

---

## 运行测试

### 所有测试

```bash
make test-all
```

### Rust 测试

```bash
# 所有 Rust 测试
cargo test --all

# 带输出
cargo test --all -- --nocapture

# 特定包
cargo test -p dos2unix-rust
cargo test -p rgzip

# 特定测试
cargo test test_stream_large_data -- --nocapture

# 带覆盖率（需要 cargo-tarpaulin）
cargo tarpaulin --all
```

### Go 测试

```bash
# 所有 Go 测试
go test ./...

# 带详细输出
go test -v ./...

# 特定包
cd gzip/go && go test -v ./...
cd htop/win/go && go test -v ./...

# 特定测试
go test -run TestGzipStream -v ./...

# 带覆盖率
go test -cover ./...
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out
```

### 测试驱动开发

```bash
# 1. 编写失败的测试
# 2. 运行测试确认失败
cargo test --lib test_new_feature

# 3. 实现功能
# 4. 运行测试确认成功
cargo test --lib test_new_feature

# 5. 重构并重复
```

---

## 调试

### Rust 调试

**使用 println!（快速方法）**:
```rust
fn process_data(data: &[u8]) {
    println!("DEBUG: 处理 {} 字节", data.len());
    println!("DEBUG: 第一个字节: {:02x?}", data.first());
    // ...
}
```

**使用 dbg! 宏**:
```rust
fn complex_function(x: i32, y: i32) -> i32 {
    let sum = dbg!(x + y);  // 打印 file:line 和值
    sum * 2
}
```

**使用 GDB/LLDB**:
```bash
# 带调试符号构建
cargo build

# 使用调试器运行
gdb ./target/debug/dos2unix-rust
lldb ./target/debug/dos2unix-rust

# 或使用 IDE 调试器（带 CodeLLDB 的 VS Code）
```

**环境变量**:
```bash
# panic 时显示完整回溯
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run

# 日志级别（如果使用 tracing）
RUST_LOG=debug cargo run
```

### Go 调试

**使用 fmt.Println（快速方法）**:
```go
func processData(data []byte) {
    fmt.Printf("DEBUG: 处理 %d 字节\n", len(data))
    fmt.Printf("DEBUG: 数据: %x\n", data[:10])
    // ...
}
```

**使用 Delve 调试器**:
```bash
# 安装 delve
go install github.com/go-delve/delve/cmd/dlv@latest

# 调试
dlv debug ./cmd/gzip-go
dlv test ./...

# 调试器内常用命令:
# (dlv) break main.main
# (dlv) continue
# (dlv) print variable
# (dlv) locals
# (dlv) step
# (dlv) quit
```

**环境变量**:
```bash
# 详细测试输出
GO111MODULE=on go test -v ./...

# 竞争检测器
go run -race ./...
go test -race ./...

# 内存分析
go test -memprofile=mem.prof ./...
go tool pprof mem.prof
```

---

## 故障排除

### 常见构建问题

#### Rust: "linker 'cc' not found"

**Linux (Ubuntu/Debian)**:
```bash
sudo apt update
sudo apt install build-essential
```

**macOS**:
```bash
xcode-select --install
```

**Linux (Fedora/RHEL)**:
```bash
sudo dnf install gcc gcc-c++ make
```

#### Rust: "cannot find -lpq"（或类似错误）

缺少系统库。安装开发包:
```bash
# Ubuntu/Debian
sudo apt install libpq-dev

# macOS with Homebrew
brew install libpq
```

#### Go: "cannot find main module"

```bash
# 确保在正确目录
cd gzip/go

# 验证 go.mod 存在
ls go.mod

# 如果缺失，重新创建
go mod init github.com/LessUp/build-your-own-tools/gzip/go
```

#### Make: "command not found"

```bash
# Windows (Git Bash)
# Make 包含在 Git for Windows 中

# Windows (PowerShell)
# 通过 chocolatey 安装
choco install make

# 或使用 mingw
# 从 https://www.mingw-w64.org/ 下载
```

### 常见运行时问题

#### 运行二进制文件时 "Permission denied"

```bash
# 添加执行权限
chmod +x ./target/release/dos2unix-rust

# 或移动到 PATH
sudo cp ./target/release/dos2unix-rust /usr/local/bin/
```

#### htop: "terminal not supported"

```bash
# 设置 TERM 变量
export TERM=xterm-256color

# 或尝试不同终端
TERM=screen-256color ./target/release/htop-unix-rust
```

#### gzip: "invalid gzip header"

输入文件不是有效的 gzip 格式:
```bash
# 检查文件类型
file corrupted.gz

# 使用 gunzip 验证
gunzip -t corrupted.gz
```

### 性能问题

#### Rust 编译慢

```bash
# 使用更快的链接器（添加到 .cargo/config.toml）
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# 或使用 mold 链接器
cargo install mold
```

#### Go 构建慢

```bash
# 启用构建缓存
export GOCACHE=$HOME/.cache/go-build

# 使用 -p 标志并行构建
go build -p 4 ./...
```

### 获取帮助

1. **检查日志**:
   ```bash
   cargo build 2>&1 | tee build.log
   ```

2. **搜索 issues**:
   ```bash
   # GitHub CLI
   gh issue list --search "你的错误"
   ```

3. **在讨论区提问**:
   - GitHub Discussions
   - Rust Discord/Reddit
   - Go Slack/Forum

---

## 下一步

### 了解更多

- 📖 [架构指南](../architecture/ARCHITECTURE.zh-CN.md) - 系统设计和模式
- 📊 [对比指南](../tutorials/COMPARISON.zh-CN.md) - Rust vs Go 分析
- 📚 [API 参考](../architecture/API.zh-CN.md) - 库文档

### 贡献

1. 阅读 [CONTRIBUTING.md](../../CONTRIBUTING.md)
2. 选择一个 issue 或建议新功能
3. 遵循 [开发工作流](#开发工作流)
4. 提交 Pull Request

### 探索相关项目

| 项目 | 语言 | 说明 |
|---------|----------|-------------|
| [redis](https://github.com/antirez/redis) | C | 内存数据库 |
| [rclone](https://github.com/rclone/rclone) | Go | 云存储同步 |
| [ripgrep](https://github.com/BurntSushi/ripgrep) | Rust | 快速文本搜索 |
| [fzf](https://github.com/junegunn/fzf) | Go | 模糊查找器 |

### 构建你自己的工具

尝试实现这些工具:
- `cat` - 文件连接
- `wc` - 字数统计
- `sort` - 行排序
- `uniq` - 重复过滤
- `head`/`tail` - 行提取

---

**术语中英文对照表**

| 英文 | 中文 |
|------|------|
| Build | 构建 |
| Debugging | 调试 |
| Error | 错误 |
| Linter | 代码检查工具 |
| Module | 模块 |
| Test | 测试 |
| Toolchain | 工具链 |
| Troubleshooting | 故障排除 |
| Workspace | 工作区 |

---

**最后更新**: 2026-04-16  
**版本**: 1.0
