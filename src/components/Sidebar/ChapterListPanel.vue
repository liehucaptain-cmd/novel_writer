<template>
  <div id="panel-chapters" class="sidebar-panel">
    <div class="chapter-list">
      <div v-if="!book?.chapters?.length" class="chapter-empty">暂无章节</div>
      <div
        v-for="(ch, idx) in (book?.chapters ?? [])"
        :key="ch.id"
        class="chapter-item"
        :class="{ active: selectedChapterId === ch.id }"
        @click="selectChapter(ch)"
      >
        <div class="ch-plotlines">
          <span
            v-for="plId in (ch.plotline_ids ?? [])"
            :key="plId"
            class="ch-plot-dot"
            :style="{ background: getPlotlineColor(plId) }"
          />
        </div>
        <span class="ch-title">{{ ch.title || '无标题' }}</span>
        <span class="ch-word-count">{{ getChapterWordCount(ch.content) }}</span>
        <button v-if="idx > 0" class="ch-sort-btn" @click.stop="moveUp(idx)" title="上移">▲</button>
        <button v-if="idx < ((book?.chapters?.length ?? 1) - 1)" class="ch-sort-btn" @click.stop="moveDown(idx)" title="下移">▼</button>
        <button class="ch-del" @click.stop="removeChapter(ch.id)" title="删除">✕</button>
      </div>
    </div>
    <div class="sidebar-footer">
      <button class="add-chapter-btn" @click="addChapter">＋ 添加章节</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useNovel } from '../../composables/useNovel'
import type { Chapter } from '../../types'

const emit = defineEmits(['select-chapter'])
const props = defineProps<{ book: any }>()
const { createChapter, deleteChapter, reorderChapters } = useNovel()

const book = computed(() => props.book)

const selectedChapterId = ref<string | null>(null)

function getChapterWordCount(content: string | undefined): string {
  const n = (content ?? '').replace(/\s/g, '').length
  if (n === 0) return ''
  return n >= 1000 ? `${(n/1000).toFixed(1)}k` : `${n}`
}

function getPlotlineColor(id: string): string {
  return book.value?.storylines?.find(p => p.id === id)?.color ?? '#888'
}

function selectChapter(ch: Chapter) {
  selectedChapterId.value = ch.id
  emit('select-chapter', ch)
}

async function addChapter() {
  if (!book.value) return
  const title = `第${book.value.chapters.length + 1}章`
  const ch = await createChapter(book.value.id, title)
  if (ch) selectChapter(ch)
}

async function removeChapter(chapterId: string) {
  if (!book.value) return
  await deleteChapter(book.value.id, chapterId)
  if (selectedChapterId.value === chapterId) selectedChapterId.value = null
}

async function moveUp(idx: number) {
  if (!book.value || idx <= 0) return
  const ids = [...book.value.chapters.map(c => c.id)]
  ;[ids[idx - 1], ids[idx]] = [ids[idx], ids[idx - 1]]
  await reorderChapters(book.value.id, ids)
}

async function moveDown(idx: number) {
  if (!book.value || idx >= (book.value.chapters.length - 1)) return
  const ids = [...book.value.chapters.map(c => c.id)]
  ;[ids[idx], ids[idx + 1]] = [ids[idx + 1], ids[idx]]
  await reorderChapters(book.value.id, ids)
}
</script>
