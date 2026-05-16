import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'
import llmstxt from 'vitepress-plugin-llms'
import { generateSitemap as sitemap } from 'sitemap-ts'
import { resolve } from 'path'

// 动态 base path - 支持 GitHub Pages 部署
const rawBase = process.env.VITEPRESS_BASE
const base = rawBase
  ? rawBase.startsWith('/')
    ? rawBase.endsWith('/') ? rawBase : `${rawBase}/`
    : `/${rawBase}/`
  : '/build-your-own-tools/'

// https://vitepress.dev/reference/site-config
export default withMermaid(defineConfig({
  // ===========================================================================
  // 基础配置
  // ===========================================================================
  title: 'Build Your Own Tools',
  description: '系统编程技术白皮书 - Rust × Go 双实现架构对比研究',

  // 基础路径
  base,

  // 干净的 URL（无 .html）
  cleanUrls: true,

  // 最后更新时间
  lastUpdated: true,

  // 忽略失效链接
  ignoreDeadLinks: true,

  // srcDir 为根目录
  srcDir: '.',

  // srcExclude 排除不需要构建的文件
  srcExclude: [
    'AGENTS.md',
    'CLAUDE.md',
    'CHANGELOG.md',
    'README.md',
    'README.zh-CN.md',
    'openspec/**',
    '.opencode/**',
    '.claude/**',
    '.github/**',
    '.backup-old-docs/**',
    'dos2unix/src/**',
    'gzip/**/src/**',
    'htop/**/src/**',
    'target/**',
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
      description: '系统编程技术白皮书 - Rust × Go 双实现架构对比研究',
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
          pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/docs/:path',
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
        socialLinks: [
          { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
        ],
        search: {
          provider: 'local',
          options: {
            locales: {
              root: {
                translations: {
                  button: { buttonText: '搜索文档', buttonAriaLabel: '搜索文档' },
                  modal: {
                    noResultsText: '无法找到相关结果',
                    resetButtonTitle: '清除查询条件',
                    footer: { selectText: '选择', navigateText: '切换', closeText: '关闭' }
                  }
                }
              }
            }
          }
        },
        // 中文导航 - 技术白皮书风格
        nav: [
          { text: '学院', link: '/docs/academy/', activeMatch: '/docs/academy/' },
          { text: '白皮书', link: '/docs/whitepaper/', activeMatch: '/docs/whitepaper/' },
          { text: '技术规范', link: '/docs/specs/', activeMatch: '/docs/specs/' },
          { text: '对比研究', link: '/docs/comparison/', activeMatch: '/docs/comparison/' },
          {
            text: '工具实现',
            items: [
              { text: '🔧 dos2unix', link: '/docs/dos2unix/' },
              { text: '📦 gzip', link: '/docs/gzip/' },
              { text: '📊 htop', link: '/docs/htop/' },
            ]
          },
          { text: '参考文献', link: '/docs/reference/', activeMatch: '/docs/reference/' },
        ],
        // 中文侧边栏
        sidebar: {
          '/docs/academy/': [
            {
              text: '学院',
              items: [
                { text: '概览', link: '/docs/academy/' },
                { text: '模块一：dos2unix', link: '/docs/academy/module-01-dos2unix/' },
                { text: '模块二：gzip', link: '/docs/academy/module-02-gzip/' },
                { text: '模块三：htop', link: '/docs/academy/module-03-htop/' },
              ]
            },
          ],
          '/docs/whitepaper/': [
            {
              text: '白皮书',
              items: [
                { text: '概览', link: '/docs/whitepaper/' },
                { text: '项目概览', link: '/docs/whitepaper/overview' },
                { text: '系统架构', link: '/docs/whitepaper/architecture' },
                { text: '设计决策', link: '/docs/whitepaper/decisions' },
                { text: '性能分析', link: '/docs/whitepaper/performance' },
              ]
            },
          ],
          '/docs/specs/': [
            {
              text: '技术规范',
              items: [
                { text: '概览', link: '/docs/specs/' },
                { text: 'OpenSpec 工作流', link: '/docs/specs/openspec-workflow' },
                { text: 'dos2unix 规范', link: '/docs/specs/dos2unix' },
                { text: 'gzip 规范', link: '/docs/specs/gzip' },
                { text: 'htop 规范', link: '/docs/specs/htop' },
              ]
            },
          ],
          '/docs/comparison/': [
            {
              text: '对比研究',
              items: [
                { text: '概览', link: '/docs/comparison/' },
                { text: '内存模型', link: '/docs/comparison/memory' },
                { text: '并发模型', link: '/docs/comparison/concurrency' },
                { text: '错误处理', link: '/docs/comparison/errors' },
                { text: '性能基准', link: '/docs/comparison/benchmarks' },
              ]
            },
          ],
          '/docs/reference/': [
            {
              text: '参考文献',
              items: [
                { text: '概览', link: '/docs/reference/' },
                { text: '学术论文', link: '/docs/reference/papers' },
                { text: '相关项目', link: '/docs/reference/projects' },
                { text: '演进思考', link: '/docs/reference/evolution' },
              ]
            },
          ],
          '/docs/dos2unix/': [
            { text: '概览', link: '/docs/dos2unix/' },
            { text: '← 返回首页', link: '/' },
          ],
          '/docs/gzip/': [
            { text: '概览', link: '/docs/gzip/' },
            { text: 'Go 实现', link: '/docs/gzip/go/' },
            { text: 'Rust 实现', link: '/docs/gzip/rust/' },
            { text: '← 返回首页', link: '/' },
          ],
          '/docs/htop/': [
            { text: '概览', link: '/docs/htop/' },
            { text: 'Unix Rust', link: '/docs/htop/unix/rust/' },
            { text: 'Unix Go', link: '/docs/htop/unix/go/' },
            { text: 'Windows Rust', link: '/docs/htop/win/rust/' },
            { text: 'Windows Go', link: '/docs/htop/win/go/' },
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
      description: 'Technical Whitepaper - Rust × Go Dual-Implementation Architecture Comparison',
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
          pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/docs/:path',
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
        socialLinks: [
          { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
        ],
        search: {
          provider: 'local',
        },
        // 英文导航
        nav: [
          { text: 'Academy', link: '/en/academy/', activeMatch: '/en/academy/' },
          { text: 'Whitepaper', link: '/en/whitepaper/', activeMatch: '/en/whitepaper/' },
          { text: 'Specifications', link: '/en/specs/', activeMatch: '/en/specs/' },
          { text: 'Comparison', link: '/en/comparison/', activeMatch: '/en/comparison/' },
          {
            text: 'Implementations',
            items: [
              { text: '🔧 dos2unix', link: '/en/dos2unix/' },
              { text: '📦 gzip', link: '/en/gzip/' },
              { text: '📊 htop', link: '/en/htop/' },
            ]
          },
          { text: 'References', link: '/en/reference/', activeMatch: '/en/reference/' },
        ],
        // 英文侧边栏
        sidebar: {
          '/en/academy/': [
            {
              text: 'Academy',
              items: [
                { text: 'Overview', link: '/en/academy/' },
                { text: 'Module 1: dos2unix', link: '/en/academy/module-01-dos2unix/' },
                { text: 'Module 2: gzip', link: '/en/academy/module-02-gzip/' },
                { text: 'Module 3: htop', link: '/en/academy/module-03-htop/' },
              ]
            },
          ],
          '/en/whitepaper/': [
            {
              text: 'Whitepaper',
              items: [
                { text: 'Overview', link: '/en/whitepaper/' },
                { text: 'Project Overview', link: '/en/whitepaper/overview' },
                { text: 'System Architecture', link: '/en/whitepaper/architecture' },
                { text: 'Design Decisions', link: '/en/whitepaper/decisions' },
                { text: 'Performance', link: '/en/whitepaper/performance' },
              ]
            },
          ],
          '/en/specs/': [
            {
              text: 'Specifications',
              items: [
                { text: 'Overview', link: '/en/specs/' },
                { text: 'OpenSpec Workflow', link: '/en/specs/openspec-workflow' },
                { text: 'dos2unix Spec', link: '/en/specs/dos2unix' },
                { text: 'gzip Spec', link: '/en/specs/gzip' },
                { text: 'htop Spec', link: '/en/specs/htop' },
              ]
            },
          ],
          '/en/comparison/': [
            {
              text: 'Comparison',
              items: [
                { text: 'Overview', link: '/en/comparison/' },
                { text: 'Memory Model', link: '/en/comparison/memory' },
                { text: 'Concurrency', link: '/en/comparison/concurrency' },
                { text: 'Error Handling', link: '/en/comparison/errors' },
                { text: 'Benchmarks', link: '/en/comparison/benchmarks' },
              ]
            },
          ],
          '/en/reference/': [
            {
              text: 'References',
              items: [
                { text: 'Overview', link: '/en/reference/' },
                { text: 'Papers', link: '/en/reference/papers' },
                { text: 'Projects', link: '/en/reference/projects' },
                { text: 'Evolution', link: '/en/reference/evolution' },
              ]
            },
          ],
          '/en/dos2unix/': [
            { text: 'Overview', link: '/en/dos2unix/' },
            { text: '← Back to Home', link: '/' },
          ],
          '/en/gzip/': [
            { text: 'Overview', link: '/en/gzip/' },
            { text: 'Go Implementation', link: '/en/gzip/go/' },
            { text: 'Rust Implementation', link: '/en/gzip/rust/' },
            { text: '← Back to Home', link: '/' },
          ],
          '/en/htop/': [
            { text: 'Overview', link: '/en/htop/' },
            { text: 'Unix Rust', link: '/en/htop/unix/rust/' },
            { text: 'Unix Go', link: '/en/htop/unix/go/' },
            { text: 'Windows Rust', link: '/en/htop/win/rust/' },
            { text: 'Windows Go', link: '/en/htop/win/go/' },
            { text: '← Back to Home', link: '/' },
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
  // Mermaid 配置
  // ===========================================================================
  mermaid: {
    // 参考: https://mermaid.js.org/config/theming.html
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
    plugins: [llmstxt()],
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
}))
