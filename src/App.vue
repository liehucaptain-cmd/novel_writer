<template>
  <div class="app" :data-theme="theme">
    <header class="app-header">
      <div class="header-logo" @click="currentView = 'home'">
        ✒️ 小说家 <span>Powered by MiniMax M2.7</span>
      </div>
      <div class="header-status">
        <span class="status-dot"></span>
        <span class="status-text">就绪</span>
        <button class="theme-btn" @click="toggleTheme" title="切换主题">
          {{ theme === 'dark' ? '🌙' : theme === 'light' ? '☀️' : '🔅' }}
        </button>
      </div>
    </header>

    <HomeView v-if="currentView === 'home'" @open-book="openBook" />
    <EditorView v-else-if="currentView === 'editor'" :book="currentBook" @back="goHome" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTheme } from './composables/useTheme'
import { useNovel } from './composables/useNovel'
import HomeView from './views/HomeView.vue'
import EditorView from './views/EditorView.vue'
import './assets/styles/base.css'

const { theme, toggleTheme } = useTheme()
const { loadBooks, loadBook, currentBook } = useNovel()

const currentView = ref<'home' | 'editor'>('home')

async function openBook(book: { id: string }) {
  await loadBook(book.id)
  currentView.value = 'editor'
}

function goHome() {
  currentView.value = 'home'
}

onMounted(() => loadBooks())
</script>
