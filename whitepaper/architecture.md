# 系统架构

本文档描述 Build Your Own Tools 项目的整体架构设计。

## 架构全景图

```mermaid
graph TB
    subgraph "用户层"
        U[用户] --> CLI[CLI 工具]
    end
    
    subgraph "应用层"
        CLI --> D1[dos2unix]
        CLI --> D2[gzip]
        CLI --> D3[htop]
    end
    
    subgraph "库层"
        D1 --> L1[流处理库]
        D2 --> L2[压缩库]
        D3 --> L3[系统监控库]
    end
    
    subgraph "平台层"
        L3 --> P1[Unix API]
        L3 --> P2[Windows API]
    end
    
    subgraph "工程化"
        E1[OpenSpec] --> E2[需求]
        E1 --> E3[变更]
        E4[CI/CD] --> E5[构建]
        E4 --> E6[测试]
        E4 --> E7[发布]
    end
    
    style U fill:#f59e0b,color:#fff
    style CLI fill:#3b82f6,color:#fff
    style E1 fill:#10b981,color:#fff
    style E4 fill:#10b981,color:#fff
```

## 仓库结构设计

### Monorepo 架构

项目采用 **Monorepo** 架构，将所有工具放在同一仓库：

```mermaid
graph LR
    A[build-your-own-tools] --> B[dos2unix/]
    A --> C[gzip/]
    A --> D[htop/]
    A --> E[openspec/]
    A --> F[docs/]
    A --> G[.github/]
    
    B --> B1[Cargo.toml]
    B --> B2[src/]
    
    C --> C1[go/]
    C --> C2[rust/]
    
    D --> D1[unix/]
    D --> D2[win/]
    
    style A fill:#f59e0b,color:#fff
```

**优点**：
- 统一版本管理
- 共享 CI/CD
- 原子提交
- 简化依赖

**缺点**：
- 仓库体积大
- 构建时间长
- 权限粒度粗

### Rust Workspace

`gzip/rust/` 和 `htop/unix/rust/` 使用 Rust workspace：

```toml
# gzip/rust/Cargo.toml
[workspace]
members = ["."]

[package]
name = "gzip"
version = "0.1.0"
```

### Go Workspace

`gzip/go/` 和 `htop/` 使用 Go workspace：

```text
# go.work
go 1.22

use ./gzip/go
use ./htop/unix/go
use ./htop/win/go
```

## 模块设计

### 库 + 二进制模式

以 `gzip/rust/` 为例：

```mermaid
graph LR
    A[gzip] --> B[src/lib.rs]
    A --> C[src/bin/gzip.rs]
    
    B --> D[压缩 API]
    B --> E[解压 API]
    
    C --> F[CLI 解析]
    C --> G[调用 lib]
    
    style A fill:#f59e0b,color:#fff
    style B fill:#3b82f6,color:#fff
    style C fill:#06b6d4,color:#fff
```

**好处**：
- 库可被其他项目引用
- 二进制专注于 CLI
- 便于测试

### 源码结构

```
gzip/rust/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 库入口
│   ├── compress.rs     # 压缩逻辑
│   ├── decompress.rs   # 解压逻辑
│   └── bin/
│       └── gzip.rs     # CLI 入口
└── tests/
    └── integration.rs
```

## 跨平台策略

### htop 架构

htop 需要支持 Unix 和 Windows 两个平台，采用分层设计：

```mermaid
graph TB
    subgraph "平台无关层"
        A[CLI] --> B[UI 组件]
        B --> C[平台抽象]
    end
    
    subgraph "平台适配层"
        C --> D[Unix Adapter]
        C --> E[Windows Adapter]
    end
    
    subgraph "系统调用"
        D --> F[libc/syscall]
        E --> G[Win32 API]
    end
    
    style A fill:#f59e0b,color:#fff
    style C fill:#10b981,color:#fff
```

### 条件编译

Rust 使用 `cfg` 属性：

```rust
#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;
```

Go 使用构建标签：

```go
//go:build unix
package main

//go:build windows
package main
```

### 差异处理

| 功能 | Unix | Windows |
|------|------|---------|
| 进程列表 | `/proc` 文件系统 | CreateToolhelp32Snapshot |
| CPU 使用率 | `/proc/stat` | GetSystemTimes |
| 内存信息 | `/proc/meminfo` | GlobalMemoryStatusEx |
| 终端大小 | ioctl TIOCGWINSZ | GetConsoleScreenBufferInfo |

## 数据流设计

### dos2unix 流处理

```mermaid
sequenceDiagram
    participant F as 文件
    participant R as Reader
    participant P as 处理器
    participant W as Writer
    participant O as 输出
    
    F->>R: 打开文件
    R->>P: 读取块 (8KB)
    P->>P: 转换换行符
    P->>W: 写入块
    W->>O: 写入文件
    loop 直到 EOF
        R->>P: 读取块
        P->>P: 转换
        P->>W: 写入
    end
```

### gzip 压缩管线

```mermaid
sequenceDiagram
    participant CLI as CLI
    participant R as Reader
    participant C as Compressor
    participant W as Writer
    
    CLI->>R: 打开源文件
    CLI->>W: 创建输出文件
    loop
        R->>C: 读取数据块
        C->>C: 压缩处理
        C->>W: 写入压缩数据
    end
    C->>W: 写入 GZIP 尾部
```

## 工程化架构

### CI/CD 流水线

```mermaid
graph LR
    A[Push/PR] --> B[lint]
    A --> C[test]
    A --> D[build]
    
    B --> E{全部通过?}
    C --> E
    D --> E
    
    E -->|Yes| F[合并]
    E -->|No| G[阻止合并]
    
    F --> H[Release]
    H --> I[构建产物]
    I --> J[GitHub Release]
    
    style A fill:#f59e0b,color:#fff
    style E fill:#3b82f6,color:#fff
    style H fill:#10b981,color:#fff
```

### OpenSpec 工作流

```mermaid
stateDiagram-v2
    [*] --> Proposal: 提交提案
    Proposal --> Design: 设计文档
    Design --> Tasks: 任务分解
    Tasks --> Implementation: 开始实现
    Implementation --> Review: 代码审查
    Review --> Archived: 合并入档
    Archived --> [*]
    
    note right of Proposal: openspec/changes/
    note right of Design: design.md
    note right of Tasks: tasks.md
    note right of Archived: archive/
```

## 技术债务

### 已知问题

| 问题 | 影响 | 优先级 |
|------|------|--------|
| htop Windows 版功能不完整 | 功能缺失 | 高 |
| 缺少基准测试覆盖 | 性能不可度量 | 中 |
| 文档国际化不完整 | 用户体验 | 低 |

### 改进方向

1. **性能基准** — 添加 criterion 集成
2. **模糊测试** — 添加 cargo-fuzz 测试
3. **API 文档** — 生成 rustdoc 和 godoc

## 相关文档

- [设计决策](/whitepaper/decisions) — ADR 风格的决策记录
- [性能分析](/whitepaper/performance) — 基准测试和优化
- [CI/CD 设计](/engineering/cicd) — 工作流详情
