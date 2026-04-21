<template>
  <div id="panel-outline" class="sidebar-panel">
    <div class="panel-toolbar">
      <button class="btn-sm" @click="addAct">＋ 幕</button>
      <div class="spacer"></div>
      <button v-if="subTab === 'plotlines'" class="btn-sm" @click="openPlotModal = true" title="放大">⛶</button>
      <button v-if="subTab === 'overview'" class="btn-sm" @click="openOverviewModal = true" title="放大">⛶</button>
    </div>

    <div class="outline-subtabs">
      <div class="outline-subtab" :class="{ active: subTab === 'tree' }" @click="subTab = 'tree'">🌳 大纲树</div>
      <div class="outline-subtab" :class="{ active: subTab === 'overview' }" @click="subTab = 'overview'">📋 概览</div>
      <div class="outline-subtab" :class="{ active: subTab === 'plotlines' }" @click="subTab = 'plotlines'">🔗 情节线</div>
    </div>

    <!-- 🌳 大纲树 -->
    <div v-show="subTab === 'tree'" class="outline-scroll">
      <div v-if="!outline?.acts?.length" class="outline-empty">
        暂无大纲<br/>
        <button class="btn-sm" @click="addAct" style="margin-top:8px">＋ 添加第一幕</button>
      </div>
      <div v-for="act in outline?.acts ?? []" :key="act.id" class="outline-act">
        <div class="outline-act-header" :class="{ selected: selectedActId === act.id }" @click="selectAct(act)">
          <span class="outline-act-toggle" :class="{ collapsed: collapsedActs[act.id] }" @click.stop="toggleAct(act.id)">▼</span>
          <div class="outline-act-color" :style="{ background: act.color }"></div>
          <template v-if="editingActId === act.id">
            <input class="outline-act-name-input" v-model="editingActName" @blur="saveActName(act)" @keydown.enter="saveActName(act)" autofocus @click.stop />
          </template>
          <template v-else>
            <div class="outline-act-name" @click.stop="startEditAct(act)">{{ act.name }}</div>
          </template>
          <span class="outline-act-meta">{{ act.chapters?.length ?? 0 }}章</span>
          <div class="outline-act-actions">
            <button @click.stop="addChapterToAct(act)">＋</button>
            <button @click.stop="removeAct(act.id)">✕</button>
          </div>
        </div>
        <div class="outline-children" :class="{ collapsed: collapsedActs[act.id] }">
          <div v-for="och in act.chapters ?? []" :key="och.id" class="outline-chapter">
            <div class="outline-chapter-header" :class="{ selected: selectedChapterId === och.id }" @click="selectChapter(och)">
              <span class="outline-chapter-toggle" :class="{ collapsed: collapsedChapters[och.id] }" @click.stop="toggleChapter(och.id)">▼</span>
              <div class="outline-chapter-dot"></div>
              <template v-if="editingChapterId === och.id">
                <input class="outline-chapter-name-input" v-model="editingChapterName" @blur="saveChapterName(och)" @keydown.enter="saveChapterName(och)" autofocus @click.stop />
              </template>
              <template v-else>
                <div class="outline-chapter-name" @click.stop="startEditChapter(och)">{{ och.name }}</div>
              </template>
              <div class="outline-chapter-actions">
                <button @click.stop="addSceneToChapter(act.id, och.id)">＋</button>
                <button @click.stop="removeChapterFromAct(act.id, och.id)">✕</button>
              </div>
            </div>
            <div class="outline-children" :class="{ collapsed: collapsedChapters[och.id] }" style="padding-left:18px">
              <div v-for="scene in och.scenes ?? []" :key="scene.id" class="outline-scene">
                <div class="outline-scene-header" :class="{ selected: selectedSceneId === scene.id }" @click="selectScene(scene)">
                  <div class="outline-scene-tri" :class="{ expanded: !collapsedScenes[scene.id] }" @click.stop="toggleScene(scene.id)"></div>
                  <template v-if="editingSceneId === scene.id">
                    <textarea class="outline-scene-name-input" v-model="editingSceneName" @blur="saveSceneName(scene)" @keydown.enter.prevent="saveSceneName(scene)" @click.stop></textarea>
                  </template>
                  <template v-else>
                    <div class="outline-scene-name" @click.stop="startEditScene(scene)">{{ scene.name }}</div>
                  </template>
                  <span class="outline-scene-status" :class="scene.status" style="font-size:9px">{{ scene.status === '待写' ? '待填写' : scene.status }}</span>
                  <div class="outline-scene-actions">
                    <button @click.stop="cycleSceneStatus(scene)">↻</button>
                    <button @click.stop="removeScene(act.id, och.id, scene.id)">✕</button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 📋 概览 -->
    <div v-show="subTab === 'overview'" class="outline-scroll overview-panel">
      <div v-if="!outline?.acts?.length" class="outline-empty">暂无大纲</div>
      <div v-for="act in outline?.acts ?? []" :key="act.id" class="overview-act">
        <div class="overview-act-bar">
          <div class="overview-act-color" :style="{ background: act.color }"></div>
          <span class="overview-act-name">{{ act.name }}</span>
          <span class="overview-act-meta">{{ act.chapters?.length }}章 / {{ act.chapters?.reduce((s,c) => s + (c.scenes?.length ?? 0), 0) }}场景</span>
        </div>
        <div class="overview-chapters">
          <div v-for="(och, ci) in (act.chapters ?? [])" :key="och.id" class="overview-chapter">
            <span class="overview-ch-num">{{ ci + 1 }}</span>
            <span class="overview-ch-name">{{ och.name }}</span>
            <div class="overview-ch-scenes">
              <span
                v-for="sc in (och.scenes ?? [])"
                :key="sc.id"
                class="overview-scene-chip"
                :class="sc.status"
                :title="`${sc.name} (${sc.status})`"
              >{{ sc.name }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 🔗 情节线 -->
    <div v-show="subTab === 'plotlines'" class="outline-scroll">
      <div class="plot-add-wrap">
        <input v-model="newPlotName" placeholder="情节线名称…" @keydown.enter="addPlotline" />
        <div class="plot-add-color-btn" :style="{ background: newPlotColor }" @click="showPlotColorPicker = !showPlotColorPicker"></div>
        <button class="plot-add-btn" @click="addPlotline">添加</button>
      </div>
      <div v-if="showPlotColorPicker" class="plot-color-picker">
        <button v-for="c in plotColors" :key="c" :style="{ background: c }" @click="newPlotColor = c; showPlotColorPicker = false" />
      </div>
      <div v-if="!storylines.length" class="plot-empty">暂无情节线</div>
      <div v-for="pl in storylines" :key="pl.id" class="plot-line-card" :style="{ borderLeftColor: pl.color }">
        <div class="plot-line-card-header">
          <div class="plot-line-color-dot" :style="{ background: pl.color }"></div>
          <div class="plot-line-name">{{ pl.name }}</div>
          <div class="plot-line-badge" :class="pl.status">{{ pl.status === 'active' ? '进行中' : '已完成' }}</div>
          <div class="plot-line-actions">
            <button @click="removePlotline(pl.id)">✕</button>
          </div>
        </div>
        <div class="plot-line-desc">{{ pl.description || '暂无描述' }}</div>
        <div class="plot-line-chapters">
          <span v-for="chId in (pl.chapter_ids ?? [])" :key="chId" class="plot-chapter-tag">{{ getChapterName(chId) }}</span>
          <span v-if="!(pl.chapter_ids?.length)" class="plot-no-chapters">未关联章节</span>
        </div>
      </div>
    </div>

    <!-- 📋 概览全屏弹窗 -->
    <div class="modal-overlay" :class="{ show: openOverviewModal }" @click.self="openOverviewModal = false">
      <div class="overview-modal">
        <div class="modal-header">
          <span>📋 全局概览</span>
          <div class="modal-header-stats" v-if="outline?.acts?.length">
            {{ outline.acts.length }}幕 ·
            {{ outline.acts.reduce((s,a) => s + (a.chapters?.length ?? 0), 0) }}章 ·
            {{ outline.acts.reduce((s,a) => s + a.chapters?.reduce((sc,c) => sc + (c.scenes?.length ?? 0), 0), 0) }}场景
          </div>
          <button class="close-btn" @click="openOverviewModal = false">✕</button>
        </div>
        <div class="overview-modal-body">
          <div v-for="act in outline?.acts ?? []" :key="act.id" class="overview-modal-act">
            <div class="overview-modal-act-header">
              <div class="overview-act-color" :style="{ background: act.color }"></div>
              <span>{{ act.name }}</span>
              <span class="overview-act-meta">{{ act.chapters?.length }}章 · {{ act.chapters?.reduce((s,c) => s + (c.scenes?.length ?? 0), 0) }}场景</span>
            </div>
            <div class="overview-modal-chapters">
              <div v-for="(och, ci) in (act.chapters ?? [])" :key="och.id" class="overview-modal-chapter">
                <div class="omc-title">{{ ci + 1 }}. {{ och.name }}</div>
                <div class="omc-scenes">
                  <div v-for="sc in (och.scenes ?? [])" :key="sc.id" class="omc-scene" :class="sc.status">
                    <span class="omc-scene-name">{{ sc.name }}</span>
                    <span class="omc-scene-status">{{ sc.status }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-if="!outline?.acts?.length" class="overview-empty">暂无大纲</div>
        </div>
      </div>
    </div>

    <!-- 🔗 情节线全屏弹窗 -->
    <div class="modal-overlay" :class="{ show: openPlotModal }" @click.self="openPlotModal = false">
      <div class="plot-modal">
        <div class="modal-header">
          <span>🔗 情节线总览</span>
          <button class="close-btn" @click="openPlotModal = false">✕</button>
        </div>
        <div class="plot-modal-body">
          <div class="plot-add-section">
            <input v-model="newPlotName" placeholder="情节线名称…" @keydown.enter="addPlotline" />
            <div class="plot-add-color-btn" :style="{ background: newPlotColor }" @click="showPlotColorPicker = !showPlotColorPicker"></div>
            <button class="btn btn-gold btn-sm" @click="addPlotline">添加</button>
          </div>
          <div v-if="showPlotColorPicker" class="plot-color-picker">
            <button v-for="c in plotColors" :key="c" :style="{ background: c }" @click="newPlotColor = c; showPlotColorPicker = false" />
          </div>
          <div v-if="!storylines.length" class="plot-empty">暂无情节线</div>
          <div class="plot-grid">
            <div v-for="pl in storylines" :key="pl.id" class="plot-line-card-full" :style="{ borderLeftColor: pl.color }">
              <div class="plot-line-card-header">
                <div class="plot-line-color-dot" :style="{ background: pl.color }"></div>
                <div class="plot-line-name">{{ pl.name }}</div>
                <div class="plot-line-badge" :class="pl.status">{{ pl.status === 'active' ? '进行中' : '已完成' }}</div>
                <div class="plot-line-actions">
                  <button @click="removePlotline(pl.id)">✕</button>
                </div>
              </div>
              <div class="plot-line-desc">{{ pl.description || '暂无描述' }}</div>
              <div class="plot-line-chapters">
                <span v-for="chId in (pl.chapter_ids ?? [])" :key="chId" class="plot-chapter-tag">{{ getChapterName(chId) }}</span>
                <span v-if="!(pl.chapter_ids?.length)" class="plot-no-chapters">未关联章节</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useNovel } from '../../composables/useNovel'
import type { Book, Outline, Act, OutlineChapter, Scene } from '../../types'

const props = defineProps<{ book: Book | null }>()
const { currentOutline, loadOutline, saveOutline, savePlotlines } = useNovel()

const subTab = ref('tree')
const storylines = computed(() => props.book?.storylines ?? [])
const outline = ref<Outline>({ acts: [] })
const openOverviewModal = ref(false)
const openPlotModal = ref(false)

const collapsedActs = ref<Record<string, boolean>>({})
const collapsedChapters = ref<Record<string, boolean>>({})
const collapsedScenes = ref<Record<string, boolean>>({})
const editingActId = ref<string | null>(null)
const editingActName = ref('')
const editingChapterId = ref<string | null>(null)
const editingChapterName = ref('')
const editingSceneId = ref<string | null>(null)
const editingSceneName = ref('')
const selectedActId = ref<string | null>(null)
const selectedChapterId = ref<string | null>(null)
const selectedSceneId = ref<string | null>(null)

const newPlotName = ref('')
const newPlotColor = ref('#c9a96e')
const showPlotColorPicker = ref(false)
const plotColors = ['#c9a96e', '#4ade80', '#e53e3e', '#60a5fa', '#f59e0b', '#a78bfa', '#f472b6', '#34d399']

function selectAct(act: Act) { selectedActId.value = act.id === selectedActId.value ? null : act.id }
function selectChapter(och: OutlineChapter) { selectedChapterId.value = och.id === selectedChapterId.value ? null : och.id }
function selectScene(sc: Scene) { selectedSceneId.value = sc.id === selectedSceneId.value ? null : sc.id }

function toggleAct(id: string) { collapsedActs.value[id] = !collapsedActs.value[id] }
function toggleChapter(id: string) { collapsedChapters.value[id] = !collapsedChapters.value[id] }
function toggleScene(id: string) { collapsedScenes.value[id] = !collapsedScenes.value[id] }

function startEditAct(act: Act) { editingActId.value = act.id; editingActName.value = act.name }
async function saveActName(act: Act) {
  act.name = editingActName.value; editingActId.value = null
  if (props.book) await saveOutline(props.book.id, outline.value!)
}
function startEditChapter(och: OutlineChapter) { editingChapterId.value = och.id; editingChapterName.value = och.name }
async function saveChapterName(och: OutlineChapter) {
  och.name = editingChapterName.value; editingChapterId.value = null
  if (props.book) await saveOutline(props.book.id, outline.value!)
}
function startEditScene(sc: Scene) { editingSceneId.value = sc.id; editingSceneName.value = sc.name }
async function saveSceneName(sc: Scene) {
  sc.name = editingSceneName.value; editingSceneId.value = null
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function addAct() {
  if (!outline.value) outline.value = { acts: [] }
  outline.value.acts.push({
    id: crypto.randomUUID(),
    name: `第${outline.value.acts.length + 1}幕`,
    color: '#c9a96e',
    chapters: [],
  })
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function removeAct(actId: string) {
  if (!outline.value) return
  outline.value.acts = outline.value.acts.filter(a => a.id !== actId)
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function addChapterToAct(act: Act) {
  act.chapters.push({ id: crypto.randomUUID(), name: `章节${act.chapters.length + 1}`, scenes: [] })
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function removeChapterFromAct(actId: string, ochId: string) {
  const act = outline.value?.acts.find(a => a.id === actId)
  if (!act) return
  act.chapters = act.chapters.filter(c => c.id !== ochId)
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function addSceneToChapter(actId: string, ochId: string) {
  const act = outline.value?.acts.find(a => a.id === actId)
  if (!act) return
  const ch = act.chapters.find(c => c.id === ochId)
  if (!ch) return
  ch.scenes.push({ id: crypto.randomUUID(), name: `场景${ch.scenes.length + 1}`, status: '待写', content: '' })
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function removeScene(actId: string, ochId: string, sceneId: string) {
  const act = outline.value?.acts.find(a => a.id === actId)
  if (!act) return
  const ch = act.chapters.find(c => c.id === ochId)
  if (!ch) return
  ch.scenes = ch.scenes.filter(s => s.id !== sceneId)
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

async function cycleSceneStatus(sc: Scene) {
  const statuses: Scene['status'][] = ['待写', '进行中', '已完成']
  sc.status = statuses[(statuses.indexOf(sc.status) + 1) % statuses.length]
  if (props.book) await saveOutline(props.book.id, outline.value!)
}

// Plotlines
async function addPlotline() {
  if (!newPlotName.value.trim() || !props.book) return
  if (!props.book.storylines) props.book.storylines = []
  props.book.storylines.push({
    id: crypto.randomUUID(),
    name: newPlotName.value.trim(),
    color: newPlotColor.value,
    status: 'active',
    description: '',
    status_summary: '',
    chapter_ids: [],
  })
  await savePlotlines(props.book.id, props.book.storylines)
  newPlotName.value = ''
}

async function removePlotline(id: string) {
  if (!props.book) return
  props.book.storylines = props.book.storylines.filter(p => p.id !== id)
  await savePlotlines(props.book.id, props.book.storylines)
}

function getChapterName(chId: string): string {
  return props.book?.chapters?.find(c => c.id === chId)?.title ?? '未知章节'
}

watch(() => props.book?.id, async (id) => {
  if (!id) return
  const o = await loadOutline(id)
  outline.value = o ?? { acts: [] }
}, { immediate: true })
</script>

<style scoped>
.outline-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
  min-height: 0;
}

/* Overview panel */
.overview-panel {
  padding: 12px;
}

.overview-act {
  margin-bottom: 16px;
}

.overview-act-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  padding: 6px 8px;
  background: var(--surface);
  border-radius: 6px;
}

.overview-act-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
  flex-shrink: 0;
}

.overview-act-name {
  font-weight: 600;
  font-size: 13px;
  color: var(--text);
  flex: 1;
}

.overview-act-meta {
  font-size: 11px;
  color: var(--muted);
}

.overview-chapters {
  padding-left: 20px;
  border-left: 2px solid var(--border);
  margin-left: 5px;
}

.overview-chapter {
  margin-bottom: 6px;
  display: flex;
  align-items: flex-start;
  gap: 6px;
}

.overview-ch-num {
  font-size: 10px;
  color: var(--muted);
  width: 14px;
  flex-shrink: 0;
  margin-top: 2px;
}

.overview-ch-name {
  font-size: 12px;
  color: var(--text);
  font-weight: 500;
  flex: 1;
}

.overview-ch-scenes {
  display: flex;
  flex-wrap: wrap;
  gap: 3px;
  margin-top: 2px;
}

.overview-scene-chip {
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 3px;
  border: 1px solid var(--border);
  color: var(--muted);
  max-width: 80px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.overview-scene-chip.待写 { border-color: #666; color: #888; }
.overview-scene-chip.进行中 { border-color: var(--gold); color: var(--gold); }
.overview-scene-chip.已完成 { border-color: var(--green); color: var(--green); }

/* Plotline cards */
.plot-add-wrap {
  display: flex;
  gap: 6px;
  align-items: center;
  margin-bottom: 8px;
}

.plot-add-wrap input {
  flex: 1;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 5px;
  padding: 6px 10px;
  color: var(--text);
  font-size: 12px;
  outline: none;
}

.plot-add-color-btn {
  width: 26px;
  height: 26px;
  border-radius: 5px;
  cursor: pointer;
  border: 2px solid var(--border);
  flex-shrink: 0;
}

.plot-add-btn {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 5px;
  color: var(--gold);
  font-size: 12px;
  padding: 5px 10px;
  cursor: pointer;
}

.plot-color-picker {
  display: flex;
  gap: 5px;
  margin-bottom: 8px;
  flex-wrap: wrap;
}

.plot-color-picker button {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
}

.plot-color-picker button:hover {
  border-color: var(--text);
}

.plot-empty, .plot-no-chapters {
  text-align: center;
  color: var(--muted);
  font-size: 12px;
  padding: 20px 0;
}

.plot-line-card {
  border-left: 3px solid;
  padding: 8px 10px;
  margin-bottom: 8px;
  background: var(--surface);
  border-radius: 0 6px 6px 0;
}

.plot-line-card-header {
  display: flex;
  align-items: center;
  gap: 6px;
}

.plot-line-color-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.plot-line-name {
  flex: 1;
  font-size: 13px;
  font-weight: 600;
  color: var(--text);
}

.plot-line-badge {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 10px;
}

.plot-line-badge.active { background: rgba(74, 222, 128, 0.15); color: var(--green); }
.plot-line-badge.completed { background: rgba(229, 62, 62, 0.1); color: var(--red); }

.plot-line-actions button {
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
  font-size: 11px;
  padding: 2px 4px;
}

.plot-line-desc {
  font-size: 11px;
  color: var(--muted);
  margin-top: 4px;
  padding-left: 16px;
}

.plot-line-chapters {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-top: 6px;
  padding-left: 16px;
}

.plot-chapter-tag {
  font-size: 10px;
  padding: 2px 7px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  color: var(--muted);
}

/* Modal overlay */
.modal-overlay {
  display: none;
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.6);
  z-index: 1000;
  align-items: center;
  justify-content: center;
}

.modal-overlay.show { display: flex; }

.modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  font-size: 14px;
  font-weight: 600;
  color: var(--gold);
}

.modal-header-stats {
  flex: 1;
  font-size: 12px;
  color: var(--muted);
  font-weight: 400;
}

.close-btn {
  background: none;
  border: none;
  color: var(--muted);
  font-size: 16px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
}

.close-btn:hover { color: var(--text); }

/* Overview modal */
.overview-modal {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  width: 90vw;
  max-width: 800px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.overview-modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.overview-modal-act {
  margin-bottom: 20px;
}

.overview-modal-act-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  font-weight: 600;
  font-size: 14px;
  color: var(--text);
}

.overview-modal-chapters {
  padding-left: 20px;
  border-left: 2px solid var(--border);
  margin-left: 6px;
}

.overview-modal-chapter {
  margin-bottom: 10px;
}

.omc-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
  margin-bottom: 5px;
}

.omc-scenes {
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding-left: 16px;
}

.omc-scene {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.omc-scene-name { color: var(--muted); flex: 1; }
.omc-scene-status {
  font-size: 10px;
  padding: 1px 7px;
  border-radius: 10px;
}
.omc-scene.待写 .omc-scene-status { background: rgba(255,255,255,0.05); color: #888; }
.omc-scene.进行中 .omc-scene-status { background: rgba(201,169,110,0.15); color: var(--gold); }
.omc-scene.已完成 .omc-scene-status { background: rgba(74,222,128,0.12); color: var(--green); }

.overview-empty {
  text-align: center;
  color: var(--muted);
  padding: 40px 0;
}

/* Plot modal */
.plot-modal {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  width: 90vw;
  max-width: 800px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.plot-modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.plot-add-section {
  display: flex;
  gap: 8px;
  align-items: center;
  margin-bottom: 10px;
}

.plot-add-section input {
  flex: 1;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 5px;
  padding: 8px 12px;
  color: var(--text);
  font-size: 13px;
  outline: none;
}

.plot-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 12px;
  margin-top: 12px;
}

.plot-line-card-full {
  border-left: 3px solid;
  padding: 12px;
  background: var(--bg);
  border-radius: 0 8px 8px 0;
}
</style>
