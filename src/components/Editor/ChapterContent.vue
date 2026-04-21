<template>
  <div class="editor-content">
    <textarea
      class="editor-textarea"
      :value="chapter?.content ?? ''"
      @input="e => $emit('update:content', (e.target as HTMLTextAreaElement).value)"
      placeholder="选择或创建章节，开始写作…"
    />
  </div>
</template>

<script setup lang="ts">
import type { Chapter } from '../../types'

defineProps<{ chapter: Chapter | null }>()
defineEmits(['update:content'])
</script>
