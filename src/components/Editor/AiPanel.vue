<template>
  <div class="ai-editor-panel" :class="{ open: isOpen }">
    <div class="ai-editor-header">
      <span>🤖 AI 助手</span>
      <button class="close-btn" @click="$emit('close')">✕</button>
    </div>

    <div class="ai-editor-body">
      <!-- Quick features -->
      <div class="quick-features">
        <button
          v-for="cat in AI_CATEGORIES"
          :key="cat.name"
          class="quick-cat"
          :class="{ active: selectedCategory === cat.name }"
          @click="selectedCategory = cat.name"
        >{{ cat.name }}</button>
      </div>

      <div class="feature-grid-sm">
        <button
          v-for="f in currentFeatures"
          :key="f.id"
          class="feature-sm-btn"
          @click="openFeature(f)"
        >
          {{ f.emoji }} {{ f.name }}
        </button>
      </div>

      <div v-if="!config?.has_api_key" class="no-api-key">
        ⚠️ 请先在侧边栏 AI Tab 中配置 API Key
      </div>

      <div v-if="result" class="result-block">
        <div class="result-header">
          <span>结果</span>
          <button @click="copy">📋</button>
        </div>
        <pre class="result-text">{{ result }}</pre>
      </div>

      <div v-if="loading" class="loading-row">
        <span class="spinner"></span> AI 创作中...
      </div>

      <div v-if="errorMsg" class="error-msg">{{ errorMsg }}</div>
    </div>

    <AIFeatureModal
      v-if="activeFeature"
      v-model="showFeatureModal"
      :feature="activeFeature"
      @result="onResult"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAI, AI_CATEGORIES, type AIFeature } from '../../composables/useAI'
import AIFeatureModal from '../shared/AIFeatureModal.vue'
import type { Chapter } from '../../types'

const props = defineProps<{ isOpen: boolean; chapter: Chapter | null }>()
defineEmits(['close'])

const { config, loading, error, loadConfig, invokeAI } = useAI()
const selectedCategory = ref(AI_CATEGORIES[0].name)
const activeFeature = ref<AIFeature | null>(null)
const showFeatureModal = ref(false)
const result = ref('')
const errorMsg = ref('')

onMounted(() => loadConfig())

const currentFeatures = computed(() => {
  return AI_CATEGORIES.find(c => c.name === selectedCategory.value)?.features ?? []
})

function openFeature(f: AIFeature) {
  if (!config.value?.has_api_key) return
  activeFeature.value = f
  showFeatureModal.value = true
}

function onResult(r: string) {
  result.value = r
  errorMsg.value = ''
}

function copy() {
  if (result.value) navigator.clipboard.writeText(result.value).catch(() => {})
}
</script>

<style scoped>
.ai-editor-panel {
  position: fixed;
  right: -360px;
  top: 0;
  bottom: 0;
  width: 360px;
  background: var(--surface);
  border-left: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  transition: right 0.25s ease;
  z-index: 50;
}
.ai-editor-panel.open { right: 0; }
.ai-editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
  font-size: 13px;
  font-weight: 600;
  color: var(--gold);
}
.close-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--muted);
  font-size: 16px;
}
.close-btn:hover { color: var(--text); }
.ai-editor-body {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.quick-features {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}
.quick-cat {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 20px;
  padding: 2px 8px;
  font-size: 10px;
  color: var(--muted);
  cursor: pointer;
  transition: all 0.15s;
}
.quick-cat:hover { color: var(--text); }
.quick-cat.active {
  background: var(--gold);
  color: #fff;
  border-color: var(--gold);
}
.feature-grid-sm {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4px;
}
.feature-sm-btn {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 7px 6px;
  font-size: 11px;
  color: var(--text);
  cursor: pointer;
  text-align: left;
  transition: all 0.15s;
}
.feature-sm-btn:hover {
  border-color: var(--gold);
  background: rgba(201, 169, 110, 0.08);
}
.no-api-key {
  font-size: 11px;
  color: var(--gold);
  text-align: center;
  padding: 12px;
  background: rgba(201, 169, 110, 0.08);
  border-radius: 6px;
  border: 1px solid rgba(201, 169, 110, 0.3);
}
.result-block {
  border: 1px solid var(--border);
  border-radius: 6px;
  overflow: hidden;
}
.result-header {
  display: flex;
  justify-content: space-between;
  padding: 4px 8px;
  background: var(--bg);
  font-size: 11px;
  color: var(--muted);
  border-bottom: 1px solid var(--border);
}
.result-header button {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 11px;
  color: var(--gold);
}
.result-text {
  padding: 8px;
  font-size: 11px;
  color: var(--text);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
  margin: 0;
  max-height: 200px;
  overflow-y: auto;
  font-family: inherit;
}
.loading-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--muted);
}
.error-msg {
  font-size: 11px;
  color: #e57373;
  padding: 8px;
  background: rgba(229, 115, 115, 0.1);
  border-radius: 6px;
}
.spinner {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid var(--border);
  border-top-color: var(--gold);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}
@keyframes spin { to { transform: rotate(360deg); } }
</style>
