import { defineConfig } from 'vitepress'
import { generateSitemap as sitemap } from 'sitemap-ts'
import { resolve } from 'path'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  // ===========================================================================
  // 基础配置
  // ===========================================================================
  lang: 'zh-CN',
  title: 'Build Your Own Tools',
  titleTemplate: ':title | BYOT',
  description: '用 Rust / Go 手写常用命令行工具的学习仓库，包含 dos2unix、gzip、htop 的实现',
  
  // 基础路径 - 必须匹配 GitHub Pages 路径
  base: '/build-your-own-tools/',
  
  // 干净的 URL（无 .html）
  cleanUrls: true,
  
  // 最后更新时间
  lastUpdated: true,
  
  // 忽略失效链接（changelog 文件通常有相对链接）
  ignoreDeadLinks: true,
  
  // ===========================================================================
  // 头信息 - 完整的 SEO 和 PWA 支持
  // ===========================================================================
  head: [
    // 视口和主题
    ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no' }],
    ['meta', { name: 'theme-color', content: '#f59e0b' }],
    ['meta', { name: 'msapplication-TileColor', content: '#f59e0b' }],
    ['meta', { name: 'apple-mobile-web-app-capable', content: 'yes' }],
    ['meta', { name: 'apple-mobile-web-app-status-bar-style', content: 'black-translucent' }],
    ['meta', { name: 'apple-mobile-web-app-title', content: 'BYOT' }],
    ['meta', { name: 'msapplication-TileImage', content: '/build-your-own-tools/logo.svg' }],
    
    // 搜索引擎
    ['meta', { name: 'robots', content: 'index, follow' }],
    ['meta', { name: 'googlebot', content: 'index, follow, max-video-preview:-1, max-image-preview:large, max-snippet:-1' }],
    ['meta', { name: 'bingbot', content: 'index, follow' }],
    ['meta', { name: 'referrer', content: 'no-referrer-when-downgrade' }],
    
    // Open Graph
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:locale', content: 'zh_CN' }],
    ['meta', { property: 'og:url', content: 'https://lessup.github.io/build-your-own-tools/' }],
    ['meta', { property: 'og:site_name', content: 'Build Your Own Tools' }],
    ['meta', { property: 'og:title', content: 'Build Your Own Tools - 用 Rust / Go 手写命令行工具' }],
    ['meta', { property: 'og:description', content: '从零开始实现常用 CLI 工具，学习底层系统编程和 Rust vs Go 跨语言对比' }],
    ['meta', { property: 'og:image', content: 'https://lessup.github.io/build-your-own-tools/og-image.png' }],
    ['meta', { property: 'og:image:width', content: '1200' }],
    ['meta', { property: 'og:image:height', content: '630' }],
    
    // Twitter Card
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'Build Your Own Tools' }],
    ['meta', { name: 'twitter:description', content: '用 Rust / Go 手写命令行工具，学习系统编程' }],
    ['meta', { name: 'twitter:image', content: 'https://lessup.github.io/build-your-own-tools/og-image.png' }],
    
    // 预连接
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    
    // DNS 预解析
    ['link', { rel: 'dns-prefetch', href: 'https://github.com' }],
    ['link', { rel: 'dns-prefetch', href: 'https://lessup.github.io' }],
    
    // Canonical URL
    ['link', { rel: 'canonical', href: 'https://lessup.github.io/build-your-own-tools/' }],
    
    // Icons
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/build-your-own-tools/logo.svg' }],
    ['link', { rel: 'alternate icon', href: '/build-your-own-tools/favicon.ico' }],
    ['link', { rel: 'mask-icon', href: '/build-your-own-tools/logo.svg', color: '#f59e0b' }],
    
    // PWA Manifest
    ['link', { rel: 'manifest', href: '/build-your-own-tools/manifest.json' }],
    
    // 作者和版权
    ['meta', { name: 'author', content: 'LessUp' }],
    ['meta', { name: 'copyright', content: 'Copyright 2025-2026 LessUp. All rights reserved.' }],
    
    // 关键词
    ['meta', { name: 'keywords', content: 'Rust, Go, CLI, 命令行工具, 系统编程, TUI, dos2unix, gzip, htop, 学习, 教程, programming, terminal' }],
    
    // 结构化数据 (JSON-LD)
    ['script', { type: 'application/ld+json' }, JSON.stringify({
      '@context': 'https://schema.org',
      '@type': 'WebSite',
      name: 'Build Your Own Tools',
      description: '用 Rust / Go 手写常用命令行工具的学习仓库',
      url: 'https://lessup.github.io/build-your-own-tools/',
      potentialAction: {
        '@type': 'SearchAction',
        target: 'https://lessup.github.io/build-your-own-tools/search?q={search_term_string}',
        'query-input': 'required name=search_term_string'
      },
      publisher: {
        '@type': 'Organization',
        name: 'LessUp',
        url: 'https://github.com/LessUp',
        logo: {
          '@type': 'ImageObject',
          url: 'https://lessup.github.io/build-your-own-tools/logo.svg'
        }
      },
      sameAs: [
        'https://github.com/LessUp/build-your-own-tools'
      ]
    })],
  ],
  
  // ===========================================================================
  // 搜索配置 - 多语言本地搜索
  // ===========================================================================
  search: {
    provider: 'local',
    options: {
      locales: {
        root: {
          translations: {
            button: {
              buttonText: '搜索文档',
              buttonAriaLabel: '搜索文档'
            },
            modal: {
              noResultsText: '无法找到相关结果',
              resetButtonTitle: '清除查询条件',
              footer: {
                selectText: '选择',
                navigateText: '切换',
                closeText: '关闭',
              }
            }
          }
        }
      },
      miniSearch: {
        options: {
          fields: ['title', 'titles', 'text'],
          storeFields: ['title', 'titles'],
          searchOptions: {
            fuzzy: 0.2,
            prefix: true,
            boost: { title: 4, titles: 2, text: 1 }
          }
        }
      }
    }
  },
  
  // ===========================================================================
  // Markdown 配置
  // ===========================================================================
  markdown: {
    lineNumbers: true,
    anchor: {
      level: [1, 2, 3, 4],
      permalink: true,
      permalinkBefore: true,
      permalinkSymbol: '#'
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
  // 主题配置
  // ===========================================================================
  themeConfig: {
    // Logo
    logo: {
      src: '/logo.svg',
      width: 24,
      height: 24,
      alt: 'BYOT Logo'
    },
    siteTitle: 'BYOT',
    
    // 导航
    nav: [
      {
        text: '指南',
        items: [
          { text: '🚀 快速开始', link: '/docs/GETTING-STARTED' },
          { text: '🏗️ 架构指南', link: '/docs/ARCHITECTURE' },
          { text: '🔍 语言对比', link: '/docs/COMPARISON' },
          { text: '📚 API 参考', link: '/docs/API' },
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
          { text: '📑 日志索引', link: '/docs/changelogs/INDEX' },
          { text: '⬆️ 迁移指南', link: '/docs/changelogs/MIGRATION' },
          { text: '🤝 贡献指南', link: '/CONTRIBUTING' },
        ]
      }
    ],
    
    // 侧边栏
    sidebar: {
      '/docs/': [
        {
          text: '📚 文档',
          collapsed: false,
          items: [
            { text: '🚀 快速开始', link: '/docs/GETTING-STARTED' },
            { text: '🏗️ 架构指南', link: '/docs/ARCHITECTURE' },
            { text: '🔀 语言对比', link: '/docs/COMPARISON' },
            { text: '🔌 API 参考', link: '/docs/API' },
          ]
        },
        {
          text: '📝 变更日志',
          collapsed: false,
          items: [
            { text: '📋 项目日志', link: '/CHANGELOG' },
            { text: '📑 日志索引', link: '/docs/changelogs/INDEX' },
            { text: '⬆️ 迁移指南', link: '/docs/changelogs/MIGRATION' },
          ]
        },
        {
          text: '🤝 社区',
          collapsed: false,
          items: [
            { text: '参与贡献', link: '/CONTRIBUTING' },
            { text: '行为准则', link: '/CODE_OF_CONDUCT' },
            { text: '安全政策', link: '/SECURITY' },
          ]
        },
      ],
      '/dos2unix/': [
        { text: '概览', link: '/dos2unix/' },
        { text: '变更日志', link: '/dos2unix/changelog/CHANGELOG' },
        { text: '← 返回首页', link: '/' },
      ],
      '/gzip/': [
        {
          text: 'gzip',
          items: [
            { text: '概览', link: '/gzip/' },
            { text: 'Go 实现', link: '/gzip/go/' },
            { text: 'Rust 实现', link: '/gzip/rust/' },
          ]
        },
        {
          text: '日志',
          items: [
            { text: 'Go 日志', link: '/gzip/go/changelog/CHANGELOG' },
            { text: 'Rust 日志', link: '/gzip/rust/changelog/CHANGELOG' },
          ]
        },
        { text: '← 返回首页', link: '/' },
      ],
      '/htop/': [
        {
          text: 'htop',
          items: [
            { text: '概览', link: '/htop/' },
            { text: '变更日志', link: '/htop/changelog/CHANGELOG' },
          ]
        },
        {
          text: '实现版本',
          items: [
            { text: 'Unix Rust', link: '/htop/unix/rust/' },
            { text: 'Windows Rust', link: '/htop/win/rust/' },
            { text: 'Windows Go', link: '/htop/win/go/' },
          ]
        },
        { text: '← 返回首页', link: '/' },
      ],
    },
    
    // 编辑链接
    editLink: {
      pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/:path',
      text: '在 GitHub 上编辑此页'
    },
    
    // 社交链接
    socialLinks: [
      { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
    ],
    
    // 页脚
    footer: {
      message: '基于 <a href="https://github.com/LessUp/build-your-own-tools/blob/master/LICENSE">MIT OR Apache-2.0</a> 许可发布',
      copyright: `Copyright © 2025-${new Date().getFullYear()} <a href="https://github.com/LessUp">LessUp</a>`
    },
    
    // 大纲
    outline: {
      level: [2, 3],
      label: '目录'
    },
    
    // 最后更新时间
    lastUpdated: {
      text: '最后更新于',
      formatOptions: {
        dateStyle: 'short',
        timeStyle: 'short'
      }
    },
    
    // 页面导航
    docFooter: {
      prev: '上一页',
      next: '下一页'
    },
    
    // 返回顶部
    returnToTopLabel: '返回顶部',
    sidebarMenuLabel: '菜单',
    darkModeSwitchLabel: '主题',
    lightModeSwitchTitle: '切换到浅色模式',
    darkModeSwitchTitle: '切换到深色模式',
    
    // 外部链接图标
    externalLinkIcon: true,
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
      lastmod: new Date().toISOString(),
    })
  },
  
  // ===========================================================================
  // Vite 配置 - 性能优化
  // ===========================================================================
  vite: {
    build: {
      chunkSizeWarningLimit: 1000,
      rollupOptions: {
        output: {
          assetFileNames: (assetInfo) => {
            const info = assetInfo.name.split('.')
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
    css: {
      postcss: {
        plugins: [],
      },
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
