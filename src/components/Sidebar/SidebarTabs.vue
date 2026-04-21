<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <button class="back-btn" @click="$emit('back')">←</button>
      <template v-if="editingTitle">
        <input
          class="book-title-input"
          v-model="titleValue"
          @blur="saveTitle"
          @keydown.enter="saveTitle"
          autofocus
        />
      </template>
      <template v-else>
        <div class="book-title-display" @click="startEditTitle">{{ book?.name ?? '—' }}</div>
      </template>
    </div>

    <div class="sidebar-body">
      <!-- 信息 -->
      <div class="sidebar-section">
        <div class="section-header" :class="{ active: expanded === 'info' }" @click="toggle('info')">
          <span class="section-icon">📖</span>
          <span class="section-label">信息</span>
          <span class="section-arrow">{{ expanded === 'info' ? '▼' : '▶' }}</span>
        </div>
        <div class="section-content panel-info" v-show="expanded === 'info'">
          <InfoPanel :book="book" @rename="startEditTitle" />
        </div>
      </div>

      <!-- 章节 -->
      <div class="sidebar-section">
        <div class="section-header" :class="{ active: expanded === 'chapters' }" @click="toggle('chapters')">
          <span class="section-icon">📑</span>
          <span class="section-label">章节</span>
          <span class="section-arrow">{{ expanded === 'chapters' ? '▼' : '▶' }}</span>
        </div>
        <div class="section-content panel-chapters" v-show="expanded === 'chapters'">
          <ChapterListPanel :book="book" @select-chapter="ch => $emit('select-chapter', ch)" />
        </div>
      </div>

      <!-- 大纲 -->
      <div class="sidebar-section">
        <div class="section-header" :class="{ active: expanded === 'outline' }" @click="toggle('outline')">
          <span class="section-icon">🗂</span>
          <span class="section-label">大纲</span>
          <span class="section-arrow">{{ expanded === 'outline' ? '▼' : '▶' }}</span>
        </div>
        <div class="section-content panel-outline" v-show="expanded === 'outline'">
          <OutlinePanel :book="book" />
        </div>
      </div>

      <!-- 人物 -->
      <div class="sidebar-section">
        <div class="section-header" :class="{ active: expanded === 'chars' }" @click="toggle('chars')">
          <span class="section-icon">👤</span>
          <span class="section-label">人物</span>
          <span class="section-arrow">{{ expanded === 'chars' ? '▼' : '▶' }}</span>
        </div>
        <div class="section-content panel-chars" v-show="expanded === 'chars'">
          <CharsPanel :book="book" />
        </div>
      </div>

      <!-- AI -->
      <div class="sidebar-section">
        <div class="section-header" :class="{ active: expanded === 'ai' }" @click="toggle('ai')">
          <span class="section-icon">🤖</span>
          <span class="section-label">AI</span>
          <span class="section-arrow">{{ expanded === 'ai' ? '▼' : '▶' }}</span>
        </div>
        <div class="section-content panel-ai" v-show="expanded === 'ai'">
          <AIPanel v-if="book" :book-id="book.id" />
          <div v-else class="ai-no-book">请先打开一本书</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useNovel } from '../../composables/useNovel'
import InfoPanel from './InfoPanel.vue'
import ChapterListPanel from './ChapterListPanel.vue'
import OutlinePanel from './OutlinePanel.vue'
import CharsPanel from './CharsPanel.vue'
import AIPanel from './AIPanel.vue'
import type { Book } from '../../types'

const props = defineProps<{ book: Book | null }>()
defineEmits(['back', 'select-chapter'])
const { updateBook } = useNovel()

const expanded = ref<string>('info')
const editingTitle = ref(false)
const titleValue = ref('')

function toggle(id: string) {
  expanded.value = expanded.value === id ? '' : id
}

function startEditTitle() {
  titleValue.value = props.book?.name ?? ''
  editingTitle.value = true
}

async function saveTitle() {
  editingTitle.value = false
  if (titleValue.value.trim() && props.book) {
    await updateBook(props.book.id, titleValue.value.trim())
  }
}
</script>

<style scoped>
.sidebar-body {
  flex: 1;
  overflow-y: auto;
}

.sidebar-section {
  border-bottom: 1px solid var(--border);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  cursor: pointer;
  background: var(--surface);
  transition: background 0.15s;
  user-select: none;
}

.section-header:hover,
.section-header.active {
  background: var(--selected-bg);
  color: var(--selected-text);
}

.section-header.active .section-arrow {
  color: var(--selected-text);
}

.section-header.active .section-title {
  color: var(--selected-text);
}

.section-icon {
  font-size: 14px;
}

.section-label {
  flex: 1;
  font-size: 13px;
  color: var(--text);
}

.section-arrow {
  font-size: 9px;
  color: var(--muted);
}

.section-content {
  background: var(--bg);
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

/* 每个面板的内部滚动区域 */
.panel-info,
.panel-chapters,
.panel-outline,
.panel-chars,
.panel-ai {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}
.ai-no-book {
  padding: 20px;
  text-align: center;
  font-size: 12px;
  color: var(--muted);
}
</style>
