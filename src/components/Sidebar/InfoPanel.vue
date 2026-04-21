<template>
  <div id="panel-info" class="sidebar-panel active">
    <div class="info-field">
      <label>书名</label>
      <div class="info-value">{{ book?.name ?? '—' }}</div>
    </div>
    <div class="info-field">
      <label>创建时间</label>
      <div class="info-value">{{ book?.created_at ? formatDate(book.created_at) : '—' }}</div>
    </div>
    <div class="info-field">
      <label>最后更新</label>
      <div class="info-value">{{ book?.updated_at ? formatDate(book.updated_at) : '—' }}</div>
    </div>
    <div class="info-stat">
      <div class="info-stat-item">
        <div class="val">{{ book?.chapters?.length ?? 0 }}</div>
        <div class="lbl">章节</div>
      </div>
      <div class="info-stat-item">
        <div class="val">{{ book?.characters?.length ?? 0 }}</div>
        <div class="lbl">人物</div>
      </div>
      <div class="info-stat-item">
        <div class="val">{{ totalWordCount.toLocaleString() }}</div>
        <div class="lbl">总字数</div>
      </div>
    </div>

    <button class="btn" style="margin-top:10px;font-size:12px;padding:7px 12px" @click="$emit('rename')">
      ✏️ 修改书名
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'
import type { Book } from '../../types'

const props = defineProps<{ book: Book | null }>()
defineEmits(['rename'])

const totalWordCount = computed(() => {
  return (props.book?.chapters ?? []).reduce((sum, ch) => {
    return sum + (ch.content ?? '').replace(/\s/g, '').length
  }, 0)
})

watch(() => props.book, (b) => {
  console.log('[InfoPanel] book - name:', b?.name, 'chapters:', b?.chapters?.length, 'chars:', b?.characters?.length)
}, { immediate: true, deep: true })

function formatDate(iso: string): string {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('zh-CN', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit'
  })
}
</script>
