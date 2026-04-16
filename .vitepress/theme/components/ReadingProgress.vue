<template>
  <div class="reading-progress" :style="{ transform: `scaleX(${progress})` }" />
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const progress = ref(0)

function updateProgress() {
  const scrollTop = window.scrollY
  const docHeight = document.documentElement.scrollHeight - window.innerHeight
  progress.value = docHeight > 0 ? scrollTop / docHeight : 0
}

onMounted(() => {
  window.addEventListener('scroll', updateProgress, { passive: true })
  updateProgress()
})

onUnmounted(() => {
  window.removeEventListener('scroll', updateProgress)
})
</script>

<style scoped>
.reading-progress {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, 
    var(--vp-c-brand-1) 0%, 
    var(--vp-c-brand-2) 50%,
    var(--vp-c-brand-3) 100%
  );
  transform-origin: 0 50%;
  z-index: 9999;
  box-shadow: 0 0 10px var(--vp-c-brand-1);
}
</style>
