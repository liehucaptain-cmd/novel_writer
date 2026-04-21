<template>
  <div class="char-snapshot-panel" :class="{ open: modelIsOpen }">
    <div class="char-snapshot-inner">
      <div class="char-snapshot-header">
        <div class="char-snapshot-title">👤 人物状态快照</div>
        <button class="icon-btn" @click="$emit('update:isOpen', false)">✕</button>
      </div>
      <div class="char-snapshot-list">
        <div v-if="!snapshotChars.length" style="color:var(--muted);font-size:12px;text-align:center;padding:10px">
          当前章节暂无出场人物
        </div>
        <div v-for="char in snapshotChars" :key="char.id" class="char-snapshot-item">
          <span class="snap-name">{{ char.name }}</span>
          <template v-if="editingId === char.id">
            <input
              class="snap-edit-input"
              v-model="editingStatus"
              @blur="saveEdit(char)"
              @keydown.enter="saveEdit(char)"
              autofocus
            />
          </template>
          <template v-else>
            <span class="snap-status">{{ (char as any).status_summary || '暂无快照' }}</span>
            <button class="snap-edit" @click="startEdit(char)">✏️</button>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Book, Chapter, Character } from '../../types'

const props = defineProps<{ chapter: Chapter | null; book: Book | null; isOpen: boolean }>()
const emit = defineEmits(['update:isOpen'])

const modelIsOpen = computed(() => props.isOpen)
const editingId = ref<string | null>(null)
const editingStatus = ref('')

const snapshotChars = computed<Character[]>(() => {
  if (!props.chapter || !props.book) return []
  const ids = props.chapter.characters_appeared ?? []
  return props.book.characters?.filter(c => ids.includes(c.id)) ?? []
})

function startEdit(char: Character) {
  editingId.value = char.id
  editingStatus.value = (char as any).status_summary ?? ''
}

function saveEdit(char: Character) {
  (char as any).status_summary = editingStatus.value
  editingId.value = null
}
</script>
