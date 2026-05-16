<script setup lang="ts">
import { useData } from 'vitepress'
import { computed } from 'vue'

const props = defineProps<{
  lightSrc: string
  darkSrc: string
  alt?: string
  class?: string
}>()

const { isDark } = useData()

const src = computed(() => isDark.value ? props.darkSrc : props.lightSrc)
</script>

<template>
  <img :src="src" :alt="alt || ''" :class="class" />
</template>
