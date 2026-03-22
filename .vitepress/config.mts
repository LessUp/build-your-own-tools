import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'zh-CN',
  title: 'Build Your Own Tools',
  description: '用 Rust / Go 手写常用命令行工具的学习仓库',

  base: '/build-your-own-tools/',

  srcExclude: [
    'README.md',
    '**/node_modules/**',
    '**/target/**',
    '**/bin/**',
    'changelog/**',
    '**/changelog/**',
    '.kiro/**',
    '.github/**',
  ],

  ignoreDeadLinks: [
    /\/changelog\//,
  ],

  head: [
    ['link', { rel: 'canonical', href: 'https://lessup.github.io/build-your-own-tools/' }],
    ['meta', { name: 'theme-color', content: '#0f172a' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Build Your Own Tools' }],
    ['meta', { property: 'og:description', content: '用 Rust / Go 手写常用命令行工具的学习仓库' }],
    ['meta', { property: 'og:url', content: 'https://lessup.github.io/build-your-own-tools/' }],
    ['meta', { name: 'twitter:card', content: 'summary' }],
    ['meta', { name: 'twitter:title', content: 'Build Your Own Tools' }],
    ['meta', { name: 'twitter:description', content: '用 Rust / Go 手写常用命令行工具的学习仓库' }],
  ],

  markdown: {
    lineNumbers: true,
    languageAlias: {
      toml: 'ini',
    },
  },

  lastUpdated: true,

  themeConfig: {
    nav: [
      { text: '指南', link: '/docs/ARCHITECTURE' },
      {
        text: '子项目',
        items: [
          { text: 'dos2unix', link: '/dos2unix/' },
          { text: 'gzip', link: '/gzip/' },
          { text: 'htop', link: '/htop/' },
        ],
      },
      { text: '语言对比', link: '/docs/COMPARISON' },
    ],

    sidebar: {
      '/docs/': [
        {
          text: '项目文档',
          items: [
            { text: '架构说明', link: '/docs/ARCHITECTURE' },
            { text: 'Rust vs Go 对比', link: '/docs/COMPARISON' },
          ],
        },
        {
          text: '社区',
          items: [
            { text: '贡献指南', link: '/CONTRIBUTING' },
            { text: '行为准则', link: '/CODE_OF_CONDUCT' },
            { text: '安全政策', link: '/SECURITY' },
            { text: '变更日志', link: '/CHANGELOG' },
          ],
        },
      ],
      '/dos2unix/': [
        {
          text: 'dos2unix',
          items: [
            { text: '概述', link: '/dos2unix/' },
          ],
        },
        { text: '← 返回首页', link: '/' },
      ],
      '/gzip/': [
        {
          text: 'gzip',
          items: [
            { text: '概述', link: '/gzip/' },
            { text: 'Go 实现', link: '/gzip/go/' },
            { text: 'Rust 实现', link: '/gzip/rust/' },
          ],
        },
        { text: '← 返回首页', link: '/' },
      ],
      '/htop/': [
        {
          text: 'htop',
          items: [
            { text: '概述', link: '/htop/' },
            { text: 'Unix Rust', link: '/htop/unix/rust/' },
            { text: 'Windows Rust', link: '/htop/win/rust/' },
            { text: 'Windows Go', link: '/htop/win/go/' },
          ],
        },
        { text: '← 返回首页', link: '/' },
      ],
    },

    editLink: {
      pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/:path',
      text: '在 GitHub 上编辑此页',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
    ],

    footer: {
      message: '基于 MIT 或 Apache-2.0 许可发布',
      copyright: 'Copyright © 2025-2026 LessUp',
    },

    search: {
      provider: 'local',
    },

    outline: {
      level: [2, 3],
      label: '目录',
    },

    lastUpdated: {
      text: '最后更新',
    },

    docFooter: {
      prev: '上一页',
      next: '下一页',
    },

    returnToTopLabel: '返回顶部',
    sidebarMenuLabel: '菜单',
    darkModeSwitchLabel: '主题',
    langMenuLabel: '语言',
    externalLinkIcon: true,
  },
})
