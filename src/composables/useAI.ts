import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useNovel } from './useNovel'

export interface AIConfigOutput {
  has_api_key: boolean
  model: string
  provider: string
  context_chapters: number
  context_characters: string[]
  context_plotlines: string[]
}

export interface AIContextConfig {
  context_chapters?: number
  context_characters?: string[]
  context_plotlines?: string[]
}

export interface AIFeature {
  id: string
  name: string
  emoji: string
  category: string
  description: string
  inputs: AIInputField[]
}

export interface AIInputField {
  key: string
  label: string
  type: 'text' | 'textarea' | 'select' | 'number'
  placeholder?: string
  options?: { label: string; value: string }[]
  default?: string
}

export const AI_FEATURES: AIFeature[] = [
  // 创意孵化
  {
    id: 'brainstorm',
    name: '头脑风暴',
    emoji: '💡',
    category: '创意孵化',
    description: '从一句话灵感生成10个故事方向',
    inputs: [
      { key: 'idea', label: '你的想法', type: 'textarea', placeholder: '一句话描述你的故事灵感...' }
    ]
  },
  {
    id: 'trailer',
    name: '梗概与预告片',
    emoji: '🎬',
    category: '创意孵化',
    description: '生成封面故事和悬念预告文案',
    inputs: [
      { key: 'theme', label: '故事主题', type: 'textarea', placeholder: '描述你的故事主题...' }
    ]
  },
  // 框架构建
  {
    id: 'outline',
    name: '大纲生成',
    emoji: '📋',
    category: '框架构建',
    description: '根据主题生成完整分章大纲',
    inputs: [
      { key: 'theme', label: '故事主题', type: 'textarea', placeholder: '描述你想写什么样的故事...' },
      { key: 'structure', label: '结构类型', type: 'select', options: [
        { label: '三幕式', value: '三幕式' },
        { label: '英雄之旅', value: '英雄之旅' },
        { label: '起承转合', value: '起承转合' },
      ], default: '三幕式' }
    ]
  },
  {
    id: 'character_profile',
    name: '角色人设卡',
    emoji: '👤',
    category: '框架构建',
    description: '根据类型和背景生成详细人物设定',
    inputs: [
      { key: 'type', label: '角色类型', type: 'select', options: [
        { label: '主角', value: '主角' },
        { label: '反派', value: '反派' },
        { label: '导师', value: '导师' },
        { label: '盟友', value: '盟友' },
        { label: '神秘人', value: '神秘人' },
      ], default: '主角' },
      { key: 'background', label: '背景设定', type: 'textarea', placeholder: '描述角色的背景...' }
    ]
  },
  {
    id: 'world_building',
    name: '世界观扩充',
    emoji: '🌍',
    category: '框架构建',
    description: '扩展故事背景设定，增加细节和深度',
    inputs: [
      { key: 'aspect', label: '扩展方面', type: 'select', options: [
        { label: '地理环境', value: '地理环境' },
        { label: '社会结构', value: '社会结构' },
        { label: '历史背景', value: '历史背景' },
        { label: '文化风俗', value: '文化风俗' },
      ], default: '地理环境' },
      { key: 'base', label: '基础设定', type: 'textarea', placeholder: '描述已有的基础设定...' }
    ]
  },
  // 内容生成
  {
    id: 'chapter_content',
    name: '章节内容生成',
    emoji: '✍️',
    category: '内容生成',
    description: '根据大纲和上下文生成章节正文',
    inputs: [
      { key: 'chapter_title', label: '章节标题', type: 'textarea', placeholder: '输入章节标题或概要...' },
      { key: 'key_events', label: '关键情节点', type: 'textarea', placeholder: '描述本章关键事件...' }
    ]
  },
  {
    id: 'dialogue',
    name: '对话生成',
    emoji: '💬',
    category: '内容生成',
    description: '根据场景和人物生成自然对话',
    inputs: [
      { key: 'scene', label: '场景描述', type: 'textarea', placeholder: '描述场景和氛围...' },
      { key: 'characters', label: '涉及人物', type: 'textarea', placeholder: '输入人物名字，用逗号分隔...' }
    ]
  },
  {
    id: 'scene_desc',
    name: '场景描写',
    emoji: '🎬',
    category: '内容生成',
    description: '生成生动的场景描写文字',
    inputs: [
      { key: 'setting', label: '场景设定', type: 'textarea', placeholder: '描述场景的时间地点环境...' },
      { key: 'mood', label: '氛围', type: 'select', options: [
        { label: '紧张', value: '紧张' },
        { label: '温馨', value: '温馨' },
        { label: '悬疑', value: '悬疑' },
        { label: '欢快', value: '欢快' },
        { label: '悲伤', value: '悲伤' },
      ], default: '紧张' }
    ]
  },
  // 修改打磨
  {
    id: 'polish_prose',
    name: '文字润色',
    emoji: '✨',
    category: '修改打磨',
    description: '对已有文字进行风格润色和优化',
    inputs: [
      { key: 'text', label: '原始文字', type: 'textarea', placeholder: '输入需要润色的文字...' },
      { key: 'style', label: '风格要求', type: 'select', options: [
        { label: '文学化', value: '文学化' },
        { label: '口语化', value: '口语化' },
        { label: '简洁', value: '简洁' },
        { label: '华丽', value: '华丽' },
      ], default: '文学化' }
    ]
  },
  // 数据辅助
  {
    id: 'name_gen',
    name: '名字生成',
    emoji: '🏷️',
    category: '数据辅助',
    description: '根据类型和时代生成合适的人名',
    inputs: [
      { key: 'role', label: '角色类型', type: 'select', options: [
        { label: '主角', value: '主角' },
        { label: '配角', value: '配角' },
        { label: '反派', value: '反派' },
      ], default: '主角' },
      { key: 'era', label: '时代背景', type: 'select', options: [
        { label: '古代', value: '古代' },
        { label: '近代', value: '近代' },
        { label: '现代', value: '现代' },
        { label: '未来', value: '未来' },
      ], default: '现代' }
    ]
  },
]

export const AI_CATEGORIES = [
  { name: '创意孵化', features: AI_FEATURES.filter(f => f.category === '创意孵化') },
  { name: '框架构建', features: AI_FEATURES.filter(f => f.category === '框架构建') },
  { name: '内容生成', features: AI_FEATURES.filter(f => f.category === '内容生成') },
  { name: '修改打磨', features: AI_FEATURES.filter(f => f.category === '修改打磨') },
  { name: '数据辅助', features: AI_FEATURES.filter(f => f.category === '数据辅助') },
]

// ── Singleton state (module level — shared across all useAI() callers) ──────────
const config = ref<AIConfigOutput | null>(null)
const apiKey = ref<string>('')
const loading = ref(false)
const generatingFeature = ref<string>('')
const error = ref<string | null>(null)
const pendingResult = ref<{ feature: string; result: string } | null>(null)
const pendingError = ref<{ feature: string; error: string } | null>(null)
let currentController: AbortController | null = null

// ── useNovel singleton (called once, same reactive refs as components) ────────────
const { currentBook } = useNovel()

// ── Helper functions ───────────────────────────────────────────────────────────
function notifyResult(feature: string, result: string) {
  pendingResult.value = { feature, result }
  pendingError.value = null
}

function notifyError(feature: string, errorMsg: string) {
  pendingError.value = { feature, error: errorMsg }
  pendingResult.value = null
}

function clearPending() {
  pendingResult.value = null
  pendingError.value = null
}

async function loadConfig() {
  try {
    config.value = await invoke<AIConfigOutput>('get_ai_config')
    try {
      apiKey.value = await invoke<string>('get_api_key')
    } catch {
      apiKey.value = ''
    }
  } catch (e) {
    console.error('[AI] loadConfig error:', e)
    error.value = String(e)
  }
}

async function saveConfig(opts: {
  api_key?: string
  model?: string
  provider?: string
  context_chapters?: number
  context_characters?: string[]
  context_plotlines?: string[]
}) {
  try {
    await invoke('set_ai_config', opts)
    await loadConfig()
  } catch (e) {
    console.error('[AI] saveConfig error:', e)
    error.value = String(e)
    throw e
  }
}

async function buildContext(bookId: string, contextConfig?: AIContextConfig): Promise<string> {
  return await invoke<string>('build_context', { bookId, contextConfig })
}

async function invokeAI(
  feature: string,
  params: Record<string, string>,
  contextConfig?: AIContextConfig
): Promise<string> {
  loading.value = true
  generatingFeature.value = feature
  error.value = null
  currentController = new AbortController()
  try {
    await loadConfig()
    const cfg = config.value
    const key = apiKey.value || cfg?.api_key || ''
    if (!key) {
      throw new Error("API Key 未设置，请在设置中配置 API Key")
    }

    const [systemPrompt, userPrompt] = await invoke<[string, string]>('build_ai_prompt', {
      feature,
      params: JSON.parse(JSON.stringify(params)),
      contextConfig: contextConfig ?? null,
      bookId: currentBook.value?.id ?? null,
    })

    const apiBody = JSON.stringify({
      model: cfg?.model || 'MiniMax-M2.7',
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: userPrompt },
      ],
      temperature: 0.7,
    })

    let response: Response
    try {
      response = await fetch('https://api.minimax.chat/v1/text/chatcompletion_v2', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${key}`,
          'Content-Type': 'application/json',
        },
        body: apiBody,
        signal: currentController.signal,
      })
    } catch (e) {
      if (e instanceof Error && e.name === 'AbortError') {
        throw new Error('用户取消了请求')
      }
      throw new Error(`网络请求失败: ${e instanceof Error ? e.message : String(e)}`)
    } finally {
      currentController = null
    }

    const responseText = await response.text()
    const data = JSON.parse(responseText)

    if (data.base_resp && data.base_resp.status_code !== 0) {
      throw new Error(`API错误 (${data.base_resp.status_code}): ${data.base_resp.status_msg}`)
    }
    if (data.error) {
      throw new Error(`API错误: ${data.error.message || JSON.stringify(data.error)}`)
    }
    if (!data.choices || !data.choices[0]?.message?.content) {
      throw new Error(`API返回格式异常: ${responseText.slice(0, 100)}`)
    }

    const text = data.choices[0].message.content
    notifyResult(feature, text)
    return text
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e)
    console.error('[invokeAI] error:', msg)
    error.value = msg
    if (msg === '用户取消了请求') {
      pendingError.value = null
      return ''
    }
    notifyError(feature, msg)
    throw new Error(msg)
  } finally {
    loading.value = false
    generatingFeature.value = ''
  }
}

function cancelAI() {
  if (currentController) {
    currentController.abort()
    currentController = null
  }
  loading.value = false
  generatingFeature.value = ''
}

// ── Public API (factory — returns same singleton refs each call) ─────────────────
export function useAI() {
  return {
    config,
    loading,
    generatingFeature,
    error,
    pendingResult,
    pendingError,
    loadConfig,
    saveConfig,
    buildContext,
    invokeAI,
    cancelAI,
    clearPending,
  }
}