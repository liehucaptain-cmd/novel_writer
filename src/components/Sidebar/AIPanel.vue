<template>
  <div class="ai-panel-root">
    <!-- Header -->
    <div class="ai-header">
      <span class="ai-title">🤖 AI 创作助手</span>
      <div class="ai-header-actions">
        <button
          class="icon-btn"
          :class="{ active: !config?.has_api_key }"
          @click="openSettings"
          title="AI 设置"
        >⚙️</button>
      </div>
    </div>

    <!-- Toast notification for background completions -->
    <div v-if="showToast" class="ai-toast" @click="showToast = false">
      {{ toastMsg }}
    </div>

    <!-- No API Key Warning -->
    <div v-if="!config?.has_api_key" class="ai-warning">
      ⚠️ 请先配置 API Key 才能使用 AI 功能
      <button class="btn-link" @click="openSettings">去设置 →</button>
    </div>

    <!-- Category selector -->
    <div v-else class="ai-category-bar">
      <div class="category-tabs">
        <button
          v-for="cat in AI_CATEGORIES"
          :key="cat.name"
          class="category-tab"
          :class="{ active: selectedCategory === cat.name }"
          @click="selectedCategory = cat.name"
        >
          {{ cat.emoji }} {{ cat.name }}
        </button>
      </div>
    </div>

    <!-- Feature list -->
    <div class="ai-features">
      <button
        v-for="feature in currentFeatures"
        :key="feature.id"
        class="feature-btn"
        @click="openFeature(feature)"
      >
        <span class="feature-emoji">{{ feature.emoji }}</span>
        <span class="feature-name">{{ feature.name }}</span>
      </button>
    </div>

    <!-- Loading (fixed at top, always visible) -->
    <div v-if="loading" class="ai-loading">
      <span class="spinner"></span>
      <span>AI 创作中{{ generatingFeature ? `:${generatingFeature}` : '' }}...</span>
      <button class="btn-cancel-ai" @click="cancelAI">取消</button>
    </div>

    <!-- Output area -->
    <div v-if="output" class="ai-output-area">
      <div class="output-header">
        <span class="output-label">生成结果</span>
        <button class="copy-btn" @click="copyOutput">📋 复制</button>
      </div>
      <pre class="output-text">{{ output }}</pre>
    </div>

    <!-- Error -->
    <div v-if="errorMsg" class="ai-error">{{ errorMsg }}</div>

    <!-- Feature Modal -->
    <AIFeatureModal
      v-if="activeFeature"
      v-model="showFeatureModal"
      :feature="activeFeature"
      @result="onResult"
    />

    <!-- Settings Modal -->
    <AISettingsModal
      v-model="showSettings"
      :config="config"
      @saved="onSettingsSaved"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useAI, AI_CATEGORIES, type AIFeature } from '../../composables/useAI'
import AISettingsModal from '../shared/AISettingsModal.vue'
import AIFeatureModal from '../shared/AIFeatureModal.vue'

const props = defineProps<{ bookId?: string }>()

const { config, loading, generatingFeature, error, pendingResult, pendingError, clearPending, loadConfig, invokeAI, cancelAI } = useAI()

const showSettings = ref(false)
const showFeatureModal = ref(false)
const selectedCategory = ref(AI_CATEGORIES[0].name)
const activeFeature = ref<AIFeature | null>(null)
const output = ref('')
const errorMsg = ref('')
const showToast = ref(false)
const toastMsg = ref('')

onMounted(() => loadConfig())

watch([pendingResult, pendingError, error], ([res, err, errVal]) => {
  if (res) {
    showToast.value = true
    toastMsg.value = `✅ ${res.feature} 生成完成`
    output.value = res.result
    errorMsg.value = ''
    setTimeout(() => { showToast.value = false }, 3000)
  } else if (err) {
    showToast.value = true
    toastMsg.value = `❌ ${err.feature} 生成失败`
    errorMsg.value = err.error
    setTimeout(() => { showToast.value = false }, 3000)
  }
})

const currentFeatures = computed(() => {
  return AI_CATEGORIES.find(c => c.name === selectedCategory.value)?.features ?? []
})

function openSettings() {
  showSettings.value = true
}

function openFeature(feature: AIFeature) {
  clearPending()
  activeFeature.value = feature
  showFeatureModal.value = true
}

async function onResult(result: string) {
  output.value = result
  errorMsg.value = ''
}

function copyOutput() {
  if (output.value) {
    navigator.clipboard.writeText(output.value).catch(() => {})
  }
}

function onSettingsSaved() {
  loadConfig()
}
</script>

<style scoped>
.ai-panel-root {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  height: 100%;
  overflow-y: auto;
}
.ai-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border);
}
.ai-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text);
}
.ai-header-actions {
  display: flex;
  gap: 4px;
}
.icon-btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background: transparent;
  cursor: pointer;
  font-size: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s;
}
.icon-btn:hover {
  background: var(--surface);
}
.icon-btn.active {
  background: rgba(255, 152, 0, 0.15);
}
.ai-warning {
  font-size: 12px;
  color: #e57373;
  padding: 8px;
  background: rgba(229, 115, 115, 0.1);
  border-radius: 6px;
  border: 1px solid rgba(229, 115, 115, 0.3);
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.btn-link {
  background: none;
  border: none;
  color: var(--gold);
  font-size: 12px;
  cursor: pointer;
  text-align: left;
  padding: 0;
}
.btn-link:hover {
  text-decoration: underline;
}
.ai-category-bar {
  margin-bottom: 4px;
}
.category-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}
.category-tab {
  padding: 6px 14px;
  border: 1px solid var(--border);
  border-radius: 16px;
  background: transparent;
  color: var(--muted);
  font-size: 13px;
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.15s;
}
.category-tab:hover {
  border-color: var(--gold);
  color: var(--gold);
}
.category-tab.active {
  background: var(--gold);
  border-color: var(--gold);
  color: #fff;
}
.ai-features {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 6px;
}
.feature-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 8px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--surface);
  cursor: pointer;
  text-align: left;
  transition: all 0.15s;
  color: var(--text);
  font-size: 12px;
}
.feature-btn:hover {
  border-color: var(--gold);
  background: rgba(255, 200, 0, 0.05);
}
.feature-emoji {
  font-size: 18px;
  flex-shrink: 0;
}
.feature-name {
  font-size: 12px;
  font-weight: 500;
  line-height: 1.3;
}
.ai-output-area {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}
.output-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  border-bottom: 1px solid var(--border);
  background: rgba(255,255,255,0.02);
}
.output-label {
  font-size: 11px;
  color: var(--muted);
  font-weight: 500;
}
.copy-btn {
  padding: 2px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: transparent;
  color: var(--muted);
  font-size: 11px;
  cursor: pointer;
}
.copy-btn:hover {
  border-color: var(--gold);
  color: var(--gold);
}
.output-text {
  padding: 10px;
  font-size: 12px;
  color: var(--text);
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 200px;
  overflow-y: auto;
  margin: 0;
  line-height: 1.6;
}
.ai-loading {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 500;
  color: #fff;
  padding: 10px 14px;
  background: linear-gradient(135deg, #e67e22, #f39c12);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(230, 126, 34, 0.3);
}
.btn-cancel-ai {
  margin-left: auto;
  padding: 3px 10px;
  border: 1px solid rgba(255,255,255,0.6);
  border-radius: 4px;
  background: rgba(255,255,255,0.15);
  color: #fff;
  font-size: 11px;
  cursor: pointer;
}
.btn-cancel-ai:hover {
  background: rgba(255,255,255,0.25);
}
.ai-error {
  font-size: 11px;
  color: #e57373;
  padding: 8px;
  background: rgba(229, 115, 115, 0.1);
  border-radius: 6px;
  border: 1px solid rgba(229, 115, 115, 0.3);
}
.ai-toast {
  position: fixed;
  bottom: 80px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 10px 18px;
  font-size: 13px;
  color: var(--text);
  box-shadow: 0 4px 16px rgba(0,0,0,0.15);
  cursor: pointer;
  z-index: 9999;
  animation: toast-in 0.2s ease;
}
@keyframes toast-in {
  from { opacity: 0; transform: translateX(-50%) translateY(10px); }
  to { opacity: 1; transform: translateX(-50%) translateY(0); }
}
.spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid var(--border);
  border-top-color: var(--gold);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
  flex-shrink: 0;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
