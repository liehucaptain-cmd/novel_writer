<template>
  <div class="book-card" :class="{ 'new-card': isNew }" @click="isNew ? $emit('new') : $emit('click')">
    <template v-if="!isNew && book">
      <div class="book-name">{{ book.name }}</div>
      <div class="book-date">{{ formatDate(book.created_at) }}</div>
      <div class="book-actions">
        <button class="icon-btn" @click.stop="$emit('move-top')" title="置顶">🔝</button>
        <button class="icon-btn" @click.stop="$emit('move-up')" title="上移">⬆️</button>
        <button class="icon-btn" @click.stop="$emit('move-down')" title="下移">⬇️</button>
        <button class="icon-btn" @click.stop="$emit('rename')" title="重命名">✏️</button>
        <button class="icon-btn" @click.stop="$emit('delete')" title="删除">🗑️</button>
      </div>
    </template>
    <template v-else-if="isNew">
      <span>＋</span><span>新建书籍</span>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { Book } from '../../types'

defineProps<{ book?: Book; isNew?: boolean }>()
defineEmits(['click', 'new', 'rename', 'delete', 'move-top', 'move-up', 'move-down'])

function formatDate(iso: string | undefined): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('zh-CN', { year: 'numeric', month: 'short', day: 'numeric' })
}
</script>
