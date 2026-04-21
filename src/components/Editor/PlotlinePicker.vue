<template>
  <div class="plotline-picker-bar">
    <span class="plotline-picker-label">情节线：</span>
    <div
      v-for="pl in book?.storylines ?? []"
      :key="pl.id"
      class="plotline-pick-chip"
      :class="{ active: isActive(pl.id) }"
      @click="togglePlotline(pl.id)"
    >
      <span class="plotline-pick-dot" :style="{ background: pl.color }"></span>
      {{ pl.name }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useNovel } from '../../composables/useNovel'
import type { Book, Chapter } from '../../types'

const props = defineProps<{ book: Book | null; chapter: Chapter | null }>()
defineEmits(['toggle'])
const { updateChapter } = useNovel()

function isActive(id: string) {
  return props.chapter?.plotline_ids?.includes(id)
}

async function togglePlotline(plId: string) {
  if (!props.chapter || !props.book) return
  const arr = [...(props.chapter.plotline_ids ?? [])]
  const idx = arr.indexOf(plId)
  if (idx >= 0) arr.splice(idx, 1)
  else arr.push(plId)
  await updateChapter(props.book.id, { ...props.chapter, plotline_ids: arr })
}
</script>
