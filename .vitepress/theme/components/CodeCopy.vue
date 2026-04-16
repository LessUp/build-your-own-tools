<template>
  <div class="code-enhancements">
    <!-- 为代码块添加语言标签和复制按钮 -->
  </div>
</template>

<script setup>
import { onMounted } from 'vue'

function enhanceCodeBlocks() {
  const codeBlocks = document.querySelectorAll('.vp-doc div[class*="language-"]')
  
  codeBlocks.forEach(block => {
    if (block.querySelector('.code-copy-button')) return
    
    const lang = block.className.match(/language-(\w+)/)?.[1] || 'txt'
    
    // 创建复制按钮
    const copyBtn = document.createElement('button')
    copyBtn.className = 'code-copy-button'
    copyBtn.innerHTML = `
      <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
        <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
      </svg>
      <span>复制</span>
    `
    
    copyBtn.addEventListener('click', async () => {
      const code = block.querySelector('code')?.textContent || ''
      try {
        await navigator.clipboard.writeText(code)
        copyBtn.classList.add('copied')
        copyBtn.innerHTML = `
          <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"></polyline>
          </svg>
          <span>已复制!</span>
        `
        setTimeout(() => {
          copyBtn.classList.remove('copied')
          copyBtn.innerHTML = `
            <svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
              <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
            </svg>
            <span>复制</span>
          `
        }, 2000)
      } catch (err) {
        console.error('复制失败:', err)
      }
    })
    
    // 添加语言标签
    const langLabel = document.createElement('span')
    langLabel.className = 'code-lang-label'
    langLabel.textContent = lang
    
    block.style.position = 'relative'
    block.appendChild(langLabel)
    block.appendChild(copyBtn)
  })
}

onMounted(() => {
  enhanceCodeBlocks()
  
  // 监听动态添加的代码块
  const observer = new MutationObserver(() => {
    enhanceCodeBlocks()
  })
  
  observer.observe(document.body, {
    childList: true,
    subtree: true
  })
})
</script>

<style>
.code-lang-label {
  position: absolute;
  top: 8px;
  left: 16px;
  font-size: 12px;
  font-weight: 600;
  color: var(--vp-code-lang-color);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  z-index: 2;
}

.code-copy-button {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: var(--vp-code-copy-code-bg);
  color: var(--vp-code-copy-code-color);
  border: none;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  z-index: 3;
  opacity: 0;
}

[class*="language-"]:hover .code-copy-button,
.code-copy-button.copied {
  opacity: 1;
}

.code-copy-button:hover {
  background: var(--vp-code-copy-code-hover-bg);
  color: var(--vp-code-copy-code-hover-color);
  transform: translateY(-1px);
}

.code-copy-button.copied {
  background: #22c55e;
  color: white;
}
</style>
