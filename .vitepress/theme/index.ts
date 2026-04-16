import { h } from 'vue'
import type { Theme } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import Layout from './Layout.vue'
import './style.css'

export default {
  extends: DefaultTheme,
  Layout: Layout,
  enhanceApp({ app, router, siteData }) {
    // 注册自定义指令
    app.directive('scroll-reveal', {
      mounted(el) {
        el.style.opacity = '0'
        el.style.transform = 'translateY(20px)'
        el.style.transition = 'all 0.6s cubic-bezier(0.4, 0, 0.2, 1)'
        
        const observer = new IntersectionObserver((entries) => {
          entries.forEach(entry => {
            if (entry.isIntersecting) {
              el.style.opacity = '1'
              el.style.transform = 'translateY(0)'
              observer.unobserve(el)
            }
          })
        }, { threshold: 0.1 })
        
        observer.observe(el)
      }
    })

    // 路由变化时执行的操作
    router.onAfterRouteChanged = (to) => {
      // 发送页面浏览事件到分析服务
      if (typeof window !== 'undefined' && (window as any).gtag) {
        (window as any).gtag('config', 'GA_MEASUREMENT_ID', {
          page_path: to
        })
      }
    }
  }
} satisfies Theme
