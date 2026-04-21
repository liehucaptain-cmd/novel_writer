<template>
  <teleport to="body">
    <div class="modal-overlay" :class="{ show: modelValue }" @click.self="close">
      <div class="modal-content feature-modal" @click.stop>
        <div class="modal-header">
          <div class="modal-title">
            <span>{{ feature.emoji }}</span>
            <span>{{ feature.name }}</span>
          </div>
          <button class="close-btn" @click="close">×</button>
        </div>

        <div class="feature-description">{{ feature.description }}</div>

        <!-- Input fields -->
    <div class="feature-form">
          <div
            v-for="field in feature.inputs"
            :key="field.key"
            class="form-group"
          >
            <label>{{ field.label }}</label>

            <!-- Textarea -->
            <textarea
              v-if="field.type === 'textarea'"
              v-model="form[field.key]"
              :placeholder="field.placeholder"
              class="form-input form-textarea"
              rows="3"
            />

            <!-- Select -->
            <select
              v-else-if="field.type === 'select'"
              v-model="form[field.key]"
              class="form-input"
            >
              <option
                v-for="opt in field.options"
                :key="opt.value"
                :value="opt.value"
              >{{ opt.label }}</option>
            </select>

            <!-- Number -->
            <input
              v-else-if="field.type === 'number'"
              type="number"
              v-model="form[field.key]"
              :placeholder="field.placeholder"
              class="form-input"
            />

            <!-- Default text -->
            <input
              v-else
              type="text"
              v-model="form[field.key]"
              :placeholder="field.placeholder"
              class="form-input"
            />
          </div>
        </div>

        <!-- Generate button -->
        <div class="feature-actions">
          <button class="btn-generate" @click="generate" :disabled="loading || !hasInput" type="button">
            <span v-if="loading" class="spinner-sm"></span>
            {{ loading ? '生成中...' : '✨ 生成' }}
          </button>
          <button v-if="loading" class="btn-cancel" @click="cancel">取消</button>
        </div>

        <!-- Output -->
        <div v-if="result" class="feature-result">
          <div class="result-header">
            <span class="result-label">生成结果</span>
            <button class="copy-btn" @click="copy">📋 复制</button>
          </div>
          <pre class="result-text">{{ result }}</pre>
        </div>

        <!-- Error -->
        <div v-if="errorMsg" class="feature-error">{{ errorMsg }}</div>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useAI, type AIFeature } from '../../composables/useAI'

const props = defineProps<{
  modelValue: boolean
  feature: AIFeature
}>()

const emit = defineEmits(['update:modelValue', 'result'])
const { invokeAI, loading, cancelAI } = useAI()

const result = ref('')
const errorMsg = ref('')

// Initialize form with defaults
const form = ref<Record<string, string>>({})

watch(() => props.feature, (f) => {
  const init: Record<string, string> = {}
  for (const field of f.inputs) {
    init[field.key] = field.default ?? ''
  }
  form.value = init
  result.value = ''
  errorMsg.value = ''
}, { immediate: true })

watch(() => props.modelValue, (v) => {
  if (v) {
    const init: Record<string, string> = {}
    for (const field of props.feature.inputs) {
      init[field.key] = field.default ?? ''
    }
    form.value = init
    result.value = ''
    errorMsg.value = ''
  }
})

const hasInput = computed(() => {
  return props.feature.inputs.some(f => {
    const val = form.value[f.key]?.trim()
    return val && val.length > 0
  })
})

async function generate() {
  errorMsg.value = ''
  result.value = ''
  const params: Record<string, string> = {}
  for (const key of Object.keys(form.value)) {
    if (form.value[key].trim()) {
      params[key] = form.value[key].trim()
    }
  }
  const text = await invokeAI(props.feature.id, params)
  result.value = text
  emit('result', text)
}

function copy() {
  if (result.value) {
    navigator.clipboard.writeText(result.value).catch(() => {})
  }
}

function cancel() {
  cancelAI()
}

function close() {
  emit('update:modelValue', false)
}
</script>

<style scoped>
.feature-modal {
  width: 520px;
  max-width: 95vw;
}
.feature-description {
  font-size: 12px;
  color: var(--muted);
  margin-bottom: 16px;
  padding: 8px 10px;
  background: var(--surface);
  border-radius: 6px;
  border-left: 3px solid var(--gold);
}
.feature-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}
.form-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.form-group label {
  font-size: 12px;
  color: var(--muted);
  font-weight: 500;
}
.form-textarea {
  resize: vertical;
  min-height: 60px;
}
.feature-actions {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 14px;
}
.btn-generate {
  background: var(--gold);
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 9px 24px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: opacity 0.15s;
}
.btn-generate:hover { opacity: 0.85; }
.btn-generate:disabled { opacity: 0.4; cursor: not-allowed; }
.btn-cancel {
  padding: 8px 16px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--surface);
  color: var(--text);
  cursor: pointer;
  font-size: 14px;
}
.feature-result {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}
.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 10px;
  background: var(--surface);
  border-bottom: 1px solid var(--border);
}
.result-label {
  font-size: 11px;
  color: var(--muted);
}
.copy-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 12px;
  color: var(--gold);
}
.result-text {
  padding: 12px;
  font-size: 12px;
  color: var(--text);
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
  margin: 0;
  max-height: 300px;
  overflow-y: auto;
  font-family: inherit;
}
.feature-error {
  margin-top: 10px;
  font-size: 11px;
  color: #e57373;
  padding: 8px;
  background: rgba(229, 115, 115, 0.1);
  border-radius: 6px;
  border: 1px solid rgba(229, 115, 115, 0.3);
}
.spinner-sm {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}
@keyframes spin { to { transform: rotate(360deg); } }
</style>
