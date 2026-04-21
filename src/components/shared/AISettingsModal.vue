<template>
  <teleport to="body">
    <div class="modal-overlay" :class="{ show: modelValue }" @click.self="close">
      <div class="modal-content ai-settings-modal" @click.stop>
        <div class="modal-header">
          <div class="modal-title">🤖 AI 设置</div>
          <button class="close-btn" @click="close">×</button>
        </div>

        <div class="settings-body">
          <!-- API 配置 -->
          <div class="settings-section">
            <div class="settings-section-title">API 配置</div>
            <div class="form-group">
              <label>AI 厂商</label>
              <select v-model="form.provider" class="form-input">
                <option value="minimax">MiniMax（推荐，支持中文小说优化）</option>
                <option value="openai">OpenAI（GPT 系列）</option>
              </select>
            </div>
            <div class="form-group">
              <label>{{ form.provider === 'openai' ? 'OpenAI' : 'MiniMax' }} API Key</label>
              <div class="api-key-row">
                <input
                  :type="showKey ? 'text' : 'password'"
                  v-model="form.api_key"
                  placeholder="输入你的 MiniMax API Key"
                  class="form-input"
                />
                <button class="toggle-vis-btn" @click="showKey = !showKey">
                  {{ showKey ? '🙈' : '👁️' }}
                </button>
              </div>
              <div class="form-hint">
                {{ config?.has_api_key ? '✅ 已配置 API Key' : '❌ 尚未配置 API Key' }}
              </div>
            </div>
            <div class="form-group">
              <label>模型</label>
              <input type="text" v-model="form.model" :placeholder="form.provider === 'openai' ? 'gpt-4o' : 'MiniMax-M2.7'" class="form-input" />
              <div class="form-hint">{{ form.provider === 'openai' ? 'OpenAI 模型，如 gpt-4o、gpt-3.5-turbo' : 'MiniMax-M2.7 支持中文小说写作优化' }}</div>
            </div>
          </div>

          <!-- Context Settings -->
          <div class="settings-section">
            <div class="settings-section-title">上下文记忆</div>
            <div class="form-group">
              <label>参考最近几章</label>
              <input
                type="number"
                v-model.number="form.context_chapters"
                min="0"
                max="20"
                class="form-input"
              />
              <div class="form-hint">AI 生成时会参考你小说的最近 N 章内容</div>
            </div>

            <div class="form-group">
              <label>关联人物</label>
              <div class="multi-select">
                <div class="multi-select-tags">
                  <span
                    v-for="charId in form.context_characters"
                    :key="charId"
                    class="tag"
                  >
                    {{ getCharName(charId) }}
                    <button @click="removeChar(charId)">×</button>
                  </span>
                </div>
                <select
                  v-if="availableChars.length > 0"
                  @change="addChar(($event.target as HTMLSelectElement).value)"
                  class="form-input"
                >
                  <option value="">+ 添加人物...</option>
                  <option v-for="c in availableChars" :key="c.id" :value="c.id">{{ c.name }}</option>
                </select>
              </div>
            </div>

            <div class="form-group">
              <label>关联情节线</label>
              <div class="multi-select">
                <div class="multi-select-tags">
                  <span
                    v-for="plotId in form.context_plotlines"
                    :key="plotId"
                    class="tag"
                    :style="{ borderColor: getPlotColor(plotId) }"
                  >
                    {{ getPlotName(plotId) }}
                    <button @click="removePlot(plotId)">×</button>
                  </span>
                </div>
                <select
                  v-if="availablePlots.length > 0"
                  @change="addPlot(($event.target as HTMLSelectElement).value)"
                  class="form-input"
                >
                  <option value="">+ 添加情节线...</option>
                  <option v-for="p in availablePlots" :key="p.id" :value="p.id">{{ p.name }}</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <div v-if="errorMsg" class="save-error">{{ errorMsg }}</div>
        <div class="modal-footer">
          <button class="btn-cancel" @click="close">取消</button>
          <button class="btn-save" :disabled="saving" @click="save">
            <span v-if="saving" class="spinner-sm"></span>
            {{ saving ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useAI } from '../../composables/useAI'
import { useNovel } from '../../composables/useNovel'
import type { AIConfigOutput } from '../../composables/useAI'

const props = defineProps<{
  modelValue: boolean
  config: AIConfigOutput | null
}>()

const emit = defineEmits(['update:modelValue', 'saved'])
const { saveConfig } = useAI()
const { currentBook } = useNovel()

const saving = ref(false)
const showKey = ref(false)
const errorMsg = ref('')

const form = ref({
  api_key: '',
  model: 'MiniMax-M2.7',
  provider: 'minimax',
  context_chapters: 5,
  context_characters: [] as string[],
  context_plotlines: [] as string[],
})

watch(() => props.modelValue, (val) => {
  if (val && props.config) {
    form.value = {
      api_key: props.config.has_api_key ? '••••••••' : '',
      model: props.config.model,
      provider: props.config.provider || 'minimax',
      context_chapters: props.config.context_chapters,
      context_characters: [...props.config.context_characters],
      context_plotlines: [...props.config.context_plotlines],
    }
  }
})

const availableChars = computed(() => {
  if (!currentBook.value) return []
  return currentBook.value.characters.filter(c => !form.value.context_characters.includes(c.id))
})

const availablePlots = computed(() => {
  if (!currentBook.value) return []
  return currentBook.value.storylines.filter(p => !form.value.context_plotlines.includes(p.id))
})

function getCharName(id: string) {
  return currentBook.value?.characters.find(c => c.id === id)?.name ?? id
}

function getPlotName(id: string) {
  return currentBook.value?.storylines.find(p => p.id === id)?.name ?? id
}

function getPlotColor(id: string) {
  return currentBook.value?.storylines.find(p => p.id === id)?.color ?? '#888'
}

function addChar(id: string) {
  if (id && !form.value.context_characters.includes(id)) {
    form.value.context_characters.push(id)
  }
}

function removeChar(id: string) {
  form.value.context_characters = form.value.context_characters.filter(c => c !== id)
}

function addPlot(id: string) {
  if (id && !form.value.context_plotlines.includes(id)) {
    form.value.context_plotlines.push(id)
  }
}

function removePlot(id: string) {
  form.value.context_plotlines = form.value.context_plotlines.filter(p => p !== id)
}

const MASK = '••••••••'
async function save() {
  errorMsg.value = ''
  saving.value = true
  try {
    if (!props.config) {
      errorMsg.value = 'config 未加载，请稍等几秒后重试或重启应用'
      saving.value = false
      return
    }
    const opts: Parameters<typeof saveConfig>[0] = {
      model: form.value.model,
      provider: form.value.provider,
      context_chapters: form.value.context_chapters,
      context_characters: form.value.context_characters,
      context_plotlines: form.value.context_plotlines,
    }
    console.log('[save] form.api_key =', JSON.stringify(form.value.api_key))
    // 只有用户实际输入了新值才发 api_key；掩码字符表示「不修改」
    if (form.value.api_key.trim() && form.value.api_key.trim() !== MASK) {
      opts.api_key = form.value.api_key.trim()
      console.log('[save] will send api_key:', opts.api_key?.slice(0, 5) + '...')
    } else {
      console.log('[save] NOT sending api_key, trim=', JSON.stringify(form.value.api_key?.trim()), 'isMask=', form.value.api_key?.trim() === MASK)
    }
    await saveConfig(opts)
    emit('saved')
    close()
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e)
    errorMsg.value = msg
    console.error('[AISettings] save error:', e)
  } finally {
    saving.value = false
  }
}

function close() {
  emit('update:modelValue', false)
}
</script>

<style scoped>
.ai-settings-modal {
  width: 480px;
}
.settings-body {
  max-height: 60vh;
  overflow-y: auto;
}
.settings-section {
  margin-bottom: 20px;
}
.settings-section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--gold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 12px;
}
.api-key-row {
  display: flex;
  gap: 8px;
  align-items: center;
}
.toggle-vis-btn {
  background: none;
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 6px 10px;
  cursor: pointer;
  font-size: 14px;
  color: var(--muted);
}
.toggle-vis-btn:hover { color: var(--text); }
.form-hint {
  font-size: 11px;
  color: var(--muted);
  margin-top: 4px;
}
.multi-select {}
.multi-select-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 8px;
}
.tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  background: var(--selected-bg);
  color: var(--gold);
  border: 1px solid var(--gold);
  border-radius: 20px;
  padding: 2px 8px;
  font-size: 11px;
}
.tag button {
  background: none;
  border: none;
  color: var(--gold);
  cursor: pointer;
  padding: 0;
  font-size: 13px;
  line-height: 1;
}
.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--border);
}
.btn-cancel, .btn-save {
  padding: 8px 20px;
  border-radius: 8px;
  font-size: 13px;
  cursor: pointer;
  border: 1px solid var(--border);
  transition: all 0.15s;
}
.btn-cancel {
  background: transparent;
  color: var(--muted);
}
.btn-cancel:hover { color: var(--text); }
.btn-save {
  background: var(--gold);
  color: #fff;
  border-color: var(--gold);
}
.btn-save:hover { opacity: 0.85; }
.btn-save:disabled { opacity: 0.5; cursor: not-allowed; }
.save-error {
  margin-top: 10px;
  font-size: 12px;
  color: #e57373;
  padding: 8px 12px;
  background: rgba(229, 115, 115, 0.1);
  border: 1px solid rgba(229, 115, 115, 0.3);
  border-radius: 6px;
}
.spinner-sm {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
  margin-right: 6px;
  vertical-align: middle;
}
@keyframes spin { to { transform: rotate(360deg); } }
</style>
