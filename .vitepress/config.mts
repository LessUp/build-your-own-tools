import { defineConfig } from 'vitepress'
import { generateSitemap as sitemap } from 'sitemap-ts'
import { resolve } from 'path'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  // ===========================================================================
  // 基础配置
  // ===========================================================================
  title: 'Build Your Own Tools',
  description: '用 Rust / Go 手写常用命令行工具的学习仓库',

  // 基础路径 - 必须匹配 GitHub Pages 路径
  base: '/build-your-own-tools/',

  // 干净的 URL（无 .html）
  cleanUrls: true,

  // 最后更新时间
  lastUpdated: true,

  // 忽略失效链接
  ignoreDeadLinks: true,

  // 排除内部规范与工具说明，避免被站点当作公开页面构建
  srcExclude: [
    'AGENTS.md',
    'CLAUDE.md',
    'release-notes.md',
    'openspec/**',
    '.opencode/**',
    '.claude/**',
    '.github/**',
  ],

  // ===========================================================================
  // 国际化配置
  // ===========================================================================
  locales: {
    root: {
      label: '简体中文',
      lang: 'zh-CN',
      title: 'Build Your Own Tools',
      titleTemplate: ':title | BYOT',
      description: '用 Rust / Go 手写常用命令行工具的学习仓库，包含 dos2unix、gzip、htop 的实现',
      themeConfig: {
        siteTitle: 'BYOT',
        outline: {
          level: [2, 3],
          label: '目录'
        },
        docFooter: {
          prev: '上一页',
          next: '下一页'
        },
        returnToTopLabel: '返回顶部',
        sidebarMenuLabel: '菜单',
        darkModeSwitchLabel: '主题',
        lightModeSwitchTitle: '切换到浅色模式',
        darkModeSwitchTitle: '切换到深色模式',
        editLink: {
          pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/:path',
          text: '在 GitHub 上编辑此页'
        },
        lastUpdated: {
          text: '最后更新于',
          formatOptions: {
            dateStyle: 'short',
            timeStyle: 'short'
          }
        },
        footer: {
          message: '基于 <a href="https://github.com/LessUp/build-your-own-tools/blob/master/LICENSE">MIT OR Apache-2.0</a> 许可发布',
          copyright: `Copyright © 2025-${new Date().getFullYear()} <a href="https://github.com/LessUp">LessUp</a>`
        },
        // 社交链接
        socialLinks: [
          { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
        ],
        // 中文导航
        nav: [
          {
            text: '指南',
            items: [
              { text: '🚀 快速开始', link: '/docs/setup/GETTING-STARTED' },
              { text: '🏗️ 架构指南', link: '/docs/architecture/ARCHITECTURE' },
              { text: '🔍 语言对比', link: '/docs/tutorials/COMPARISON' },
            ]
          },
          {
            text: '子项目',
            items: [
              { text: '🔧 dos2unix', link: '/dos2unix/' },
              { text: '📦 gzip', link: '/gzip/' },
              { text: '📊 htop', link: '/htop/' },
            ]
          },
          {
            text: '更多',
            items: [
              { text: '📋 变更日志', link: '/CHANGELOG' },
              { text: '⭐ GitHub', link: 'https://github.com/LessUp/build-your-own-tools' },
            ]
          }
        ],
        // 中文侧边栏
        sidebar: {
          '/docs/': [
            {
              text: '📚 文档',
              items: [
                { text: '🚀 快速开始', link: '/docs/setup/GETTING-STARTED' },
                { text: '🏗️ 架构指南', link: '/docs/architecture/ARCHITECTURE' },
                { text: '🔍 语言对比', link: '/docs/tutorials/COMPARISON' },
              ]
            },
          ],
          '/dos2unix/': [
            { text: '概览', link: '/dos2unix/' },
            { text: '← 返回首页', link: '/' },
          ],
          '/gzip/': [
            { text: '概览', link: '/gzip/' },
            { text: 'Go 实现', link: '/gzip/go/' },
            { text: 'Rust 实现', link: '/gzip/rust/' },
            { text: '← 返回首页', link: '/' },
          ],
          '/htop/': [
            { text: '概览', link: '/htop/' },
            { text: 'Unix Rust', link: '/htop/unix/rust/' },
            { text: 'Windows Rust', link: '/htop/win/rust/' },
            { text: 'Windows Go', link: '/htop/win/go/' },
            { text: '← 返回首页', link: '/' },
          ],
        },
      }
    },
    en: {
      label: 'English',
      lang: 'en',
      title: 'Build Your Own Tools',
      titleTemplate: ':title | BYOT',
      description: 'Learn system programming by re-implementing common CLI tools in Rust and Go',
      link: '/en/',
      themeConfig: {
        siteTitle: 'BYOT',
        outline: {
          level: [2, 3],
          label: 'On this page'
        },
        docFooter: {
          prev: 'Previous page',
          next: 'Next page'
        },
        returnToTopLabel: 'Return to top',
        sidebarMenuLabel: 'Menu',
        darkModeSwitchLabel: 'Appearance',
        lightModeSwitchTitle: 'Switch to light theme',
        darkModeSwitchTitle: 'Switch to dark theme',
        editLink: {
          pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/:path',
          text: 'Edit this page on GitHub'
        },
        lastUpdated: {
          text: 'Updated at',
          formatOptions: {
            dateStyle: 'short',
            timeStyle: 'short'
          }
        },
        footer: {
          message: 'Released under the <a href="https://github.com/LessUp/build-your-own-tools/blob/master/LICENSE">MIT OR Apache-2.0</a> License',
          copyright: `Copyright © 2025-${new Date().getFullYear()} <a href="https://github.com/LessUp">LessUp</a>`
        },
        // 社交链接
        socialLinks: [
          { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
        ],
        // 英文导航
        nav: [
          {
            text: 'Guide',
            items: [
              { text: '🚀 Getting Started', link: '/en/docs/setup/GETTING-STARTED' },
              { text: '🏗️ Architecture', link: '/en/docs/architecture/ARCHITECTURE' },
              { text: '🔍 Comparison', link: '/en/docs/tutorials/COMPARISON' },
            ]
          },
          {
            text: 'Projects',
            items: [
              { text: '🔧 dos2unix', link: '/en/dos2unix/' },
              { text: '📦 gzip', link: '/en/gzip/' },
              { text: '📊 htop', link: '/en/htop/' },
            ]
          },
          {
            text: 'More',
            items: [
              { text: '📋 Changelog', link: '/en/CHANGELOG' },
              { text: '⭐ GitHub', link: 'https://github.com/LessUp/build-your-own-tools' },
            ]
          }
        ],
        // 英文侧边栏
        sidebar: {
          '/en/docs/': [
            {
              text: '📚 Documentation',
              items: [
                { text: '🚀 Getting Started', link: '/en/docs/setup/GETTING-STARTED' },
                { text: '🏗️ Architecture', link: '/en/docs/architecture/ARCHITECTURE' },
                { text: '🔍 Comparison', link: '/en/docs/tutorials/COMPARISON' },
              ]
            },
          ],
          '/en/dos2unix/': [
            { text: 'Overview', link: '/en/dos2unix/' },
            { text: '← Back to Home', link: '/en/' },
          ],
          '/en/gzip/': [
            { text: 'Overview', link: '/en/gzip/' },
            { text: 'Go Implementation', link: '/en/gzip/go/' },
            { text: 'Rust Implementation', link: '/en/gzip/rust/' },
            { text: '← Back to Home', link: '/en/' },
          ],
          '/en/htop/': [
            { text: 'Overview', link: '/en/htop/' },
            { text: 'Unix Rust', link: '/en/htop/unix/rust/' },
            { text: 'Windows Rust', link: '/en/htop/win/rust/' },
            { text: 'Windows Go', link: '/en/htop/win/go/' },
            { text: '← Back to Home', link: '/en/' },
          ],
        },
      }
    }
  },

  // ===========================================================================
  // 头信息 - SEO 和 PWA 支持
  // ===========================================================================
  head: [
    ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no' }],
    ['meta', { name: 'theme-color', content: '#f59e0b' }],
    ['meta', { name: 'msapplication-TileColor', content: '#f59e0b' }],
    ['meta', { name: 'apple-mobile-web-app-capable', content: 'yes' }],
    ['meta', { name: 'apple-mobile-web-app-status-bar-style', content: 'black-translucent' }],
    ['meta', { name: 'apple-mobile-web-app-title', content: 'BYOT' }],
    ['meta', { name: 'msapplication-TileImage', content: '/build-your-own-tools/logo.svg' }],

    ['meta', { name: 'robots', content: 'index, follow' }],
    ['meta', { name: 'referrer', content: 'no-referrer-when-downgrade' }],

    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { rel: 'dns-prefetch', href: 'https://github.com' }],
    ['link', { rel: 'dns-prefetch', href: 'https://lessup.github.io' }],

    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/build-your-own-tools/logo.svg' }],
    ['link', { rel: 'mask-icon', href: '/build-your-own-tools/logo.svg', color: '#f59e0b' }],
    ['link', { rel: 'manifest', href: '/build-your-own-tools/manifest.json' }],

    ['meta', { name: 'author', content: 'LessUp' }],
    ['meta', { name: 'copyright', content: 'Copyright 2025-2026 LessUp. All rights reserved.' }],
  ],

  // ===========================================================================
  // Markdown 配置
  // ===========================================================================
  markdown: {
    lineNumbers: true,
    anchor: {
      level: [1, 2, 3, 4]
    },
    toc: {
      level: [2, 3],
    },
    languageAlias: {
      toml: 'ini',
      rs: 'rust',
      golang: 'go',
      bash: 'shell',
      zsh: 'shell',
    }
  },

  // ===========================================================================
  // 构建钩子 - 生成 Sitemap
  // ===========================================================================
  buildEnd: async (siteConfig) => {
    await sitemap({
      hostname: 'https://lessup.github.io/build-your-own-tools/',
      outDir: siteConfig.outDir,
      exclude: ['/404.html', '/search.html'],
      changefreq: 'weekly',
      priority: 0.8,
      lastmod: new Date(),
    })
  },

  // ===========================================================================
  // Vite 配置
  // ===========================================================================
  vite: {
    build: {
      chunkSizeWarningLimit: 1000,
      rollupOptions: {
        output: {
          assetFileNames: (assetInfo) => {
            const info = (assetInfo.name ?? 'asset').split('.')
            const ext = info[info.length - 1]
            if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(ext)) {
              return `assets/images/[name][extname]`
            }
            return `assets/[name]-[hash][extname]`
          },
        },
      },
    },
    optimizeDeps: {
      include: ['vue', '@vue/runtime-dom'],
    },
    resolve: {
      alias: {
        '@': resolve(__dirname, '.'),
      },
    },
    server: {
      port: 5173,
      strictPort: false,
      open: true,
    },
    preview: {
      port: 4173,
      strictPort: false,
    },
  },

  // ===========================================================================
  // Vue 配置
  // ===========================================================================
  vue: {
    template: {
      compilerOptions: {
        isCustomElement: (tag) => tag.includes('-'),
      },
    },
  },
})
