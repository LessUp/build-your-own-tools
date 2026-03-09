---
layout: home

hero:
  name: Build Your Own Tools
  text: 用 Rust / Go 手写命令行工具
  tagline: 练习底层实现、命令行设计与跨语言对比
  actions:
    - theme: brand
      text: 架构说明
      link: /docs/ARCHITECTURE
    - theme: alt
      text: 语言对比
      link: /docs/COMPARISON
    - theme: alt
      text: GitHub
      link: https://github.com/LessUp/build-your-own-tools

features:
  - icon: 🔧
    title: dos2unix
    details: Rust 实现的 CRLF → LF 转换工具，演示基本文件 I/O 操作
    link: /dos2unix/
  - icon: 📦
    title: gzip
    details: Go + Rust 双语言实现的压缩/解压命令行，对比流处理差异
    link: /gzip/
  - icon: 📊
    title: htop
    details: 跨平台 TUI 系统监控（Unix + Windows），Rust ratatui / Go tview
    link: /htop/
---
