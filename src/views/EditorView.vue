<template>
  <div class="editor-view">
    <SidebarTabs :book="book" @back="$emit('back')" @select-chapter="selectChapter" />

    <div class="editor-area">
      <EditorToolbar
        :chapter="currentChapter"
        :book="book"
        :show-char-picker="showCharPicker"
        :show-plotline-picker="showPlotlinePicker"
        :saved="saved"
        :chapterWordCount="chapterWordCount"
        @update:title="onTitleChange"
        @toggle-char-picker="showCharPicker = !showCharPicker"
        @toggle-plotline-picker="showPlotlinePicker = !showPlotlinePicker"
        @toggle-ai="showAi = !showAi"
        @save="doSave"
      />

      <ChapterContent :chapter="currentChapter" @update:content="onContentChange" />

      <CharSnapshot :chapter="currentChapter" :book="book" :is-open="showCharSnapshot" @update:is-open="v => showCharSnapshot = v" />
      <AiPanel :is-open="showAi" :chapter="currentChapter" @close="showAi = false" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useNovel } from '../composables/useNovel'
import SidebarTabs from '../components/Sidebar/SidebarTabs.vue'
import EditorToolbar from '../components/Editor/EditorToolbar.vue'
import ChapterContent from '../components/Editor/ChapterContent.vue'
import CharSnapshot from '../components/Editor/CharSnapshot.vue'
import AiPanel from '../components/Editor/AiPanel.vue'
import type { Chapter } from '../types'

defineEmits(['back'])
const props = defineProps<{ book: any }>()
const { currentChapter, updateChapter } = useNovel()

// Use prop book directly for reactivity (book comes from parent App.vue's useNovel instance)
const book = computed(() => props.book)

watch(book, (b) => {
  console.log('[EditorView] book computed changed:', b?.chapters?.length, 'chars:', b?.characters?.length)
}, { deep: true })

onMounted(() => {
  console.log('[EditorView] mounted, book:', props.book?.chapters?.length, 'chars:', props.book?.characters?.length)
})

const showCharPicker = ref(false)
const showPlotlinePicker = ref(false)
const showAi = ref(false)
const showCharSnapshot = ref(false)
const saved = ref(true)

function selectChapter(ch: Chapter) {
  currentChapter.value = ch
  saved.value = true
}

let saveTimer: ReturnType<typeof setTimeout> | null = null

function onTitleChange(title: string) {
  if (!currentChapter.value) return
  currentChapter.value.title = title
  saved.value = false
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => doSave(), 1500)
}

function onContentChange(content: string) {
  if (!currentChapter.value) return
  currentChapter.value.content = content
  showCharSnapshot.value = true
  saved.value = false
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => doSave(), 1500)
}

async function doSave() {
  if (!currentChapter.value || !book.value) return
  saved.value = true
  await updateChapter(book.value.id, currentChapter.value)
}
</script>
