<template>
  <div class="editor-toolbar">
    <input
      class="chapter-title-input"
      :value="chapter?.title ?? ''"
      @input="e => $emit('update:title', (e.target as HTMLInputElement).value)"
      placeholder="章节标题…"
    />
    <div class="toolbar-sep"></div>
    <button class="toolbar-btn" :class="{ active: showCharPicker }" @click="$emit('toggle-char-picker')">
      👤 出场人物
    </button>
    <button class="toolbar-btn" :class="{ active: showPlotlinePicker }" @click="$emit('toggle-plotline-picker')">
      🔗 情节线
    </button>
    <button class="toolbar-btn" @click="$emit('toggle-ai')">🤖 AI 续写</button>
    <div class="toolbar-sep"></div>
    <button class="toolbar-btn" @click="$emit('save')">💾 保存</button>
    <span class="save-indicator" :class="{ saved }">{{ saved ? '已保存' : '未保存' }}</span>
    <span class="word-count" v-if="chapterWordCount > 0">{{ chapterWordCount }} 字</span>
  </div>

  <CharPicker v-if="showCharPicker" :book="book" :chapter="chapter" @toggle="$emit('toggle-char-picker')" />
  <PlotlinePicker v-if="showPlotlinePicker" :book="book" :chapter="chapter" @toggle="$emit('toggle-plotline-picker')" />
</template>

<script setup lang="ts">
import CharPicker from './CharPicker.vue'
import PlotlinePicker from './PlotlinePicker.vue'
import type { Chapter, Book } from '../../types'

defineProps<{
  chapter: Chapter | null
  book: Book | null
  showCharPicker: boolean
  showPlotlinePicker: boolean
  saved: boolean
  chapterWordCount: number
}>()
defineEmits(['update:title', 'toggle-char-picker', 'toggle-plotline-picker', 'toggle-ai', 'save'])
</script>

<style scoped>
.word-count {
  font-size: 12px;
  color: var(--muted);
  padding: 0 6px;
  white-space: nowrap;
}
</style>
