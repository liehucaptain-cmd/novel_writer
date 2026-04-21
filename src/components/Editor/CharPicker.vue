<template>
  <div class="chapter-char-picker">
    <span class="chapter-char-picker-label">出场人物：</span>
    <div
      v-for="char in book?.characters ?? []"
      :key="char.id"
      class="chapter-char-chip"
      :class="{ active: isActive(char.id) }"
      @click="toggleChar(char.id)"
    >{{ char.name }}</div>
  </div>
</template>

<script setup lang="ts">
import { useNovel } from '../../composables/useNovel'
import type { Book, Chapter } from '../../types'

const props = defineProps<{ book: Book | null; chapter: Chapter | null }>()
defineEmits(['toggle'])
const { updateChapter } = useNovel()

function isActive(id: string) {
  return props.chapter?.characters_appeared?.includes(id)
}

async function toggleChar(charId: string) {
  if (!props.chapter || !props.book) return
  const arr = [...(props.chapter.characters_appeared ?? [])]
  const idx = arr.indexOf(charId)
  if (idx >= 0) arr.splice(idx, 1)
  else arr.push(charId)
  await updateChapter(props.book.id, { ...props.chapter, characters_appeared: arr })
}
</script>
