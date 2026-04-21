<template>
  <div class="chars-panel">
    <!-- 角色列表区 -->
    <div class="chars-list-area">
      <div class="chars-toolbar">
        <select v-model="filterStatus" class="chars-filter">
          <option value="">全部</option>
          <option value="待出场">待出场</option>
          <option value="已出场">已出场</option>
          <option value="重要">重要</option>
          <option value="已下线">已下线</option>
        </select>
        <div class="chars-count">{{ filteredChars.length }} 人</div>
        <button class="btn btn-gold btn-sm" @click="openAddChar">＋ 人物</button>
      </div>

      <!-- 角色卡片网格 -->
      <div class="chars-grid" v-if="filteredChars.length">
        <div
          v-for="(char, idx) in filteredChars"
          :key="char.id"
          class="char-card-grid"
          :class="{ selected: selectedCharId === char.id }"
          draggable="true"
          @click="selectChar(char)"
          @dragstart="onDragStart(idx, $event)"
          @dragend="onDragEnd"
        >
          <div class="char-avatar" :style="{ background: getCharColor(char.id) }">
            {{ char.name.slice(0, 1) }}
          </div>
          <div class="char-info">
            <div class="char-name-row">
              <span class="char-name-text">{{ char.name }}</span>
              <span class="char-status-dot" :class="char.status" :title="char.status">●</span>
            </div>
            <div class="char-desc-text">{{ char.description || '暂无描述' }}</div>
          </div>
          <div class="char-actions">
            <button @click.stop="editChar(char)">✏️</button>
            <button @click.stop="deleteCharById(char.id)">🗑️</button>
          </div>
        </div>
      </div>
      <div v-else class="chars-empty">
        {{ filterStatus ? '该状态下暂无人物' : '暂无人物，点击添加' }}
      </div>
    </div>

    <!-- 人物图谱区 -->
    <div class="graph-area" @click="openGraphModal = true">
      <div class="graph-label">👤 人物关系图 — 点击放大</div>
      <svg ref="svgEl" class="graph-svg">
        <g v-for="edge in edges" :key="edge.id">
          <line class="graph-edge" :x1="edge.x1" :y1="edge.y1" :x2="edge.x2" :y2="edge.y2" :stroke="edge.color" />
          <text :x="edge.mx" :y="edge.my - 6" fill="#888" font-size="9" text-anchor="middle">{{ edge.type }}</text>
        </g>
        <g class="graph-node" v-for="node in graphNodes" :key="node.id">
          <circle :cx="node.x" :cy="node.y" r="20" :fill="node.color" :stroke="node.selected ? '#c9a96e' : 'transparent'" stroke-width="2" />
          <text :x="node.x" :y="node.y" :fill="node.textColor" font-size="10" text-anchor="middle" dominant-baseline="central" font-weight="600">{{ node.fullName }}</text>
        </g>
      </svg>
    </div>

    <!-- 人物图谱全屏弹窗 -->
    <div class="modal-overlay" :class="{ show: openGraphModal }" @click.self="closeGraphModal">
      <div class="graph-modal">
        <div class="graph-modal-header">
          <span>👤 人物关系图</span>
          <div class="graph-header-actions">
            <select v-model="graphFilterChar" class="graph-filter-select">
              <option value="">全部人物</option>
              <option v-for="c in allChars" :key="c.id" :value="c.id">{{ c.name }}</option>
            </select>
            <button class="btn-sm" @click="openRelModal">＋ 关系</button>
            <button class="close-btn" @click="closeGraphModal">✕</button>
          </div>
        </div>
        <!-- 操作提示栏 -->
        <div class="graph-hint-bar">
          <span v-if="selectedNodesForRel.length === 0">点击两个人物节点建立关系</span>
          <span v-else-if="selectedNodesForRel.length === 1">已选 1 人，再点击第二人</span>
          <span v-else-if="selectedNodesForRel.length === 2 && pendingRel">
            {{ allChars.find(c => c.id === selectedNodesForRel[0])?.name }} →
            <select v-model="pendingRel.type" class="rel-quick-select">
              <option value="父子">父子</option><option value="母女">母女</option><option value="兄妹">兄妹</option>
              <option value="兄弟">兄弟</option><option value="姐妹">姐妹</option><option value="恋人">恋人</option>
              <option value="夫妻">夫妻</option><option value="师徒">师徒</option><option value="朋友">朋友</option>
              <option value="主仆">主仆</option><option value="仇敌">仇敌</option><option value="对手">对手</option>
              <option value="其他">其他</option>
            </select> → {{ allChars.find(c => c.id === selectedNodesForRel[1])?.name }}
            <button class="btn-sm" @click="confirmPendingRel">✓ 建立</button>
            <button class="btn-sm" @click="cancelPendingRel">取消</button>
          </span>
          <span v-if="selectedEdgeForEdit">
            选中关系：{{ selectedEdgeForEdit.fromName }} - {{ selectedEdgeForEdit.type }} - {{ selectedEdgeForEdit.toName }}
            <select v-model="selectedEdgeForEdit.type" class="rel-quick-select" @change="updateEdgeType">
              <option value="父子">父子</option><option value="母女">母女</option><option value="兄妹">兄妹</option>
              <option value="兄弟">兄弟</option><option value="姐妹">姐妹</option><option value="恋人">恋人</option>
              <option value="夫妻">夫妻</option><option value="师徒">师徒</option><option value="朋友">朋友</option>
              <option value="主仆">主仆</option><option value="仇敌">仇敌</option><option value="对手">对手</option>
              <option value="其他">其他</option>
            </select>
            <button class="btn-sm btn-danger" @click="deleteSelectedEdge">🗑 删除</button>
            <button class="btn-sm" @click="selectedEdgeForEdit = null">取消</button>
          </span>
        </div>
        <svg ref="svgModalEl" class="graph-modal-svg" style="width:100%;min-height:300px"
             @mousedown="onSvgPanStart" @mousemove="onSvgPanMove" @mouseup="onSvgMouseUp" @mouseleave="onSvgMouseUp">
          <g :transform="'translate(' + panX + ',' + panY + ')'">
          <!-- Transparent bg rect for click-to-deselect and pan start -->
          <rect x="0" y="0" width="100%" height="100%" fill="transparent" style="cursor:grab" />
          <!-- Pending relationship dashed line -->
          <line v-if="pendingRelLine" class="graph-edge-pending" :x1="pendingRelLine.x1" :y1="pendingRelLine.y1" :x2="pendingRelLine.x2" :y2="pendingRelLine.y2" stroke="#c9a96e" stroke-width="2" stroke-dasharray="6,3" />
          <!-- Existing edges -->
          <g v-for="edge in filteredModalNodes.edges" :key="edge.id">
            <line class="graph-edge modal-edge" :x1="edge.x1" :y1="edge.y1" :x2="edge.x2" :y2="edge.y2"
              :stroke="selectedEdgeForEdit?.edgeId === edge.id ? '#c9a96e' : edge.color"
              :stroke-width="selectedEdgeForEdit?.edgeId === edge.id ? 3 : 2"
              stroke-dasharray="none" style="cursor:pointer" @click="selectEdgeForEdit(edge)" />
            <g v-if="edge.type" style="pointer-events:none">
              <rect :x="edge.mx - 28" :y="edge.my - 10" width="56" height="20" rx="10" fill="rgba(20,16,10,0.8)" />
              <text :x="edge.mx" :y="edge.my" fill="#c9a96e" font-size="10" text-anchor="middle" dominant-baseline="central" font-weight="700" style="pointer-events:none">{{ edge.type }}</text>
            </g>
          </g>
          <!-- Nodes -->
          <g class="graph-node" v-for="node in filteredModalNodes.nodes" :key="node.id"
             style="cursor:grab" @mousedown.stop="startNodeDrag($event, node)" @click.stop="toggleNodeForRel(node)">
            <circle :cx="node.x" :cy="node.y" r="30"
              :fill="node.color"
              :stroke="selectedNodesForRel.includes(node.id) ? '#c9a96e' : node.selected ? '#c9a96e' : 'transparent'"
              stroke-width="3" />
            <text :x="node.x" :y="node.y" :fill="node.textColor" font-size="10" text-anchor="middle" dominant-baseline="central" font-weight="700" style="pointer-events:none">{{ node.fullName }}</text>
          </g>
          </g>
        </svg>
        <!-- Click on SVG background = deselect all -->
        <div class="graph-deselect-hint" v-if="selectedNodesForRel.length" @click.stop="deselectAll">点击空白处取消选择</div>
      </div>
    </div>

    <!-- 添加/编辑人物弹窗 -->
    <Modal v-model="showAddChar" :title="editingChar ? '编辑人物' : '添加人物'">
      <!-- 标签页 -->
      <div class="char-form-tabs">
        <button v-for="t in charTabs" :key="t" :class="{ active: activeTab === t }" @click="activeTab = t">{{ t }}</button>
      </div>

      <!-- 基本信息 -->
      <div v-show="activeTab === '基本信息'">
        <div class="form-group">
          <label>姓名</label>
          <input v-model="charForm.name" placeholder="人物姓名" />
        </div>
        <div class="form-group">
          <label>状态</label>
          <select v-model="charForm.status">
            <option>待出场</option>
            <option>已出场</option>
            <option>重要</option>
            <option>已下线</option>
          </select>
        </div>
        <div class="form-group">
          <label>分组/阵营</label>
          <input v-model="charForm.group" placeholder="所属阵营/群体" />
        </div>
        <div class="form-group">
          <label>简介</label>
          <textarea v-model="charForm.description" placeholder="人物简介…" rows="3" />
        </div>
      </div>

      <!-- 外观 -->
      <div v-show="activeTab === '外观'">
        <div class="form-group">
          <label>年龄</label>
          <input v-model="charForm.age" placeholder="如：28岁" />
        </div>
        <div class="form-group">
          <label>外貌</label>
          <textarea v-model="charForm.appearance" placeholder="身高、体型、五官特征…" rows="4" />
        </div>
      </div>

      <!-- 性格 -->
      <div v-show="activeTab === '性格'">
        <div class="form-group">
          <label>性格</label>
          <textarea v-model="charForm.personality" placeholder="人物性格特点…" rows="4" />
        </div>
        <div class="form-group">
          <label>背景</label>
          <textarea v-model="charForm.background" placeholder="人物来历、经历…" rows="4" />
        </div>
      </div>

      <!-- 关系 -->
      <div v-show="activeTab === '关系'">
        <div class="form-group">
          <label>关系列表</label>
          <div v-if="!charForm.relationships?.length" class="rel-empty">暂无关系</div>
          <div v-for="(rel, idx) in charForm.relationships" :key="idx" class="rel-item">
            <select v-model="rel.rel_type" class="rel-type-select">
              <option value="父子">父子</option>
              <option value="母女">母女</option>
              <option value="兄妹">兄妹</option>
              <option value="兄弟">兄弟</option>
              <option value="姐妹">姐妹</option>
              <option value="恋人">恋人</option>
              <option value="夫妻">夫妻</option>
              <option value="师徒">师徒</option>
              <option value="朋友">朋友</option>
              <option value="主仆">主仆</option>
              <option value="仇敌">仇敌</option>
              <option value="对手">对手</option>
              <option value="其他">其他</option>
            </select>
            <input v-model="rel.target_id" placeholder="目标人物ID" class="rel-target-input" />
            <button class="rel-del" @click="removeRelForm(idx)">✕</button>
          </div>
          <button class="btn-sm" @click="addRelForm">＋ 添加关系</button>
        </div>
      </div>

      <div class="modal-actions">
        <button class="btn" @click="showAddChar = false">取消</button>
        <button class="btn btn-gold" @click="saveChar">保存</button>
      </div>
    </Modal>

    <!-- 添加关系弹窗（图谱模式） -->
    <Modal v-model="showRelModal" title="添加人物关系">
      <div class="form-group">
        <label>人物 A</label>
        <select v-model="relForm.fromId">
          <option value="">选择人物…</option>
          <option v-for="c in allChars" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
      </div>
      <div class="form-group">
        <label>关系类型</label>
        <select v-model="relForm.type">
          <option value="父子">父子</option>
          <option value="母女">母女</option>
          <option value="兄妹">兄妹</option>
          <option value="兄弟">兄弟</option>
          <option value="恋人">恋人</option>
          <option value="夫妻">夫妻</option>
          <option value="师徒">师徒</option>
          <option value="朋友">朋友</option>
          <option value="主仆">主仆</option>
          <option value="仇敌">仇敌</option>
          <option value="对手">对手</option>
          <option value="其他">其他</option>
        </select>
      </div>
      <div class="form-group">
        <label>人物 B</label>
        <select v-model="relForm.toId">
          <option value="">选择人物…</option>
          <option v-for="c in allChars" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="showRelModal = false">取消</button>
        <button class="btn btn-gold" @click="addRelGraph">添加</button>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useNovel } from '../../composables/useNovel'
import Modal from '../shared/Modal.vue'
import type { Book, Character, CharRelationship } from '../../types'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { appDataDir, join } from '@tauri-apps/api/path'

const CHAR_COLORS = ['#2d3a5a', '#1a4a3a', '#3a2a5a', '#5a2a2a', '#2a4a5a', '#4a3a1a', '#3a5a2a', '#5a1a3a']
const charTabs = ['基本信息', '外观', '性格', '关系']

interface GraphNode { id: string; x: number; y: number; label: string; fullName: string; color: string; textColor: string; selected: boolean }
interface GraphEdge { id: string; x1: number; y1: number; x2: number; y2: number; mx: number; my: number; color: string; type: string; fromId: string; toId: string }

const props = defineProps<{ book: Book | null }>()
const { createCharacter, updateCharacter, deleteCharacter, reorderCharacters } = useNovel()
const book = computed(() => props.book)
const allChars = computed(() => book.value?.characters ?? [])

const filterStatus = ref('')
const showAddChar = ref(false)
const editingChar = ref<Character | null>(null)
const selectedCharId = ref<string | null>(null)
const activeTab = ref('基本信息')

// Relationship graph interaction
const selectedNodesForRel = ref<string[]>([])
const pendingRel = ref<{ fromId: string; toId: string; type: string } | null>(null)
const pendingRelLine = ref<{ x1: number; y1: number; x2: number; y2: number } | null>(null)
const selectedEdgeForEdit = ref<{ edgeId: string; fromName: string; toName: string; type: string; charId: string; relIdx: number; targetId: string } | null>(null)

function toggleNodeForRel(node: GraphNode) {
  if (node.id === justDraggedId) return
  selectedEdgeForEdit.value = null
  const idx = selectedNodesForRel.value.indexOf(node.id)
  if (idx !== -1) {
    selectedNodesForRel.value.splice(idx, 1)
    pendingRel.value = null
    pendingRelLine.value = null
  } else {
    if (selectedNodesForRel.value.length < 2) {
      selectedNodesForRel.value.push(node.id)
      if (selectedNodesForRel.value.length === 2) {
        pendingRel.value = { fromId: selectedNodesForRel.value[0], toId: selectedNodesForRel.value[1], type: '朋友' }
        const n1 = modalGraphNodes.value.find(n => n.id === selectedNodesForRel.value[0])
        const n2 = modalGraphNodes.value.find(n => n.id === selectedNodesForRel.value[1])
        if (n1 && n2) {
          pendingRelLine.value = { x1: n1.x, y1: n1.y, x2: n2.x, y2: n2.y }
        }
      }
    }
  }
}

function selectEdgeForEdit(edge: GraphEdge) {
  selectedNodesForRel.value = []
  pendingRel.value = null
  pendingRelLine.value = null
  const rel = graphRels.value.find(r => r.id === edge.id)
  if (rel) {
    selectedEdgeForEdit.value = { ...rel, edgeId: edge.id, targetId: rel.rel.target_id }
  }
}

// Direct file write to bypass invoke hang
// appDataDir() returns com.novelwriter.app but data lives under novel-writer-tauri, so use fixed path
async function directUpdateChar(bookId: string, updatedChar: Character) {
  try {
    const fixedPath = '/Users/wuxiansheng/Library/Application Support/novel-writer-tauri/books/' + bookId + '.json'
    console.log('[directUpdateChar] reading from:', fixedPath)
    const content = await readTextFile(fixedPath)
    const bookData = JSON.parse(content)
    const idx = bookData.characters.findIndex((c: Character) => c.id === updatedChar.id)
    if (idx !== -1) {
      // Normalize TS->Rust: rel_type becomes type in JSON for Rust
      const charForFile = JSON.parse(JSON.stringify(updatedChar))
      for (const rel of charForFile.relationships || []) {
        if ('rel_type' in rel) { rel['type'] = rel['rel_type']; delete rel['rel_type'] }
      }
      bookData.characters[idx] = charForFile
    }
    bookData.updated_at = new Date().toISOString()
    await writeTextFile(fixedPath, JSON.stringify(bookData, null, 2))
    console.log('[directUpdateChar] SUCCESS')
    return updatedChar
  } catch(e) {
    console.error('[directUpdateChar] ERROR:', e)
    return null
  }
}

async function confirmPendingRel() {
  const pr = pendingRel.value
  if (!pr || !book.value) return
  const char = allChars.value.find(c => c.id === pr.fromId)
  if (!char) return
  const newRel: CharRelationship = { rel_type: pr.type, target_id: pr.toId }
  const updatedChar: Character = { ...char, relationships: [...(char.relationships ?? []), newRel] }
  console.log('[confirmPendingRel] START, char:', char.name, 'newRel:', newRel)

  // Try invoke first (3s timeout)
  const timer = setTimeout(() => console.log('[confirmPendingRel] invoke still hanging after 3s, using direct write...'), 3000)
  let success = false
  try {
    await Promise.race([
      invoke('update_character', { bookId: book.value.id, character: updatedChar }),
      new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), 3000))
    ])
    clearTimeout(timer)
    console.log('[confirmPendingRel] invoke SUCCESS')
    success = true
  } catch {
    clearTimeout(timer)
    console.warn('[confirmPendingRel] invoke failed/timeout, trying direct fs write')
    const saved = await directUpdateChar(book.value.id, updatedChar)
    if (saved) success = true
  }
  if (success) {
    const charIdx = (book.value as Book).characters.findIndex(c => c.id === updatedChar.id)
    if (charIdx !== -1) (book.value as Book).characters.splice(charIdx, 1, updatedChar)
  }
  cancelPendingRel()
  rebuildModalEdges()
}

function cancelPendingRel() {
  selectedNodesForRel.value = []
  pendingRel.value = null
  pendingRelLine.value = null
}

async function updateEdgeType() {
  const edge = selectedEdgeForEdit.value
  if (!edge || !book.value) return
  const char = allChars.value.find(c => c.id === edge.charId)
  if (!char) return
  const newRels = [...(char.relationships ?? [])]
  newRels[edge.relIdx] = { rel_type: edge.type, target_id: edge.targetId }
  const updatedChar = { ...char, relationships: newRels }
  const timer = setTimeout(() => console.log('[updateEdgeType] invoke hanging...'), 3000)
  let success = false
  try {
    await Promise.race([
      invoke('update_character', { bookId: book.value.id, character: updatedChar }),
      new Promise((_, reject) => setTimeout(() => reject(new Error()), 3000))
    ])
    clearTimeout(timer)
    success = true
  } catch {
    clearTimeout(timer)
    const saved = await directUpdateChar(book.value.id, updatedChar)
    if (saved) success = true
  }
  if (success) {
    const idx = (book.value as Book).characters.findIndex(c => c.id === updatedChar.id)
    if (idx !== -1) (book.value as Book).characters.splice(idx, 1, updatedChar)
  }
  rebuildModalEdges()
}

async function deleteSelectedEdge() {
  const edge = selectedEdgeForEdit.value
  if (!edge || !book.value) return
  const char = allChars.value.find(c => c.id === edge.charId)
  if (!char) return
  const newRels = [...(char.relationships ?? [])]
  newRels.splice(edge.relIdx, 1)
  const updatedChar = { ...char, relationships: newRels }
  const timer = setTimeout(() => console.log('[deleteSelectedEdge] invoke hanging...'), 3000)
  let success = false
  try {
    await Promise.race([
      invoke('update_character', { bookId: book.value.id, character: updatedChar }),
      new Promise((_, reject) => setTimeout(() => reject(new Error()), 3000))
    ])
    clearTimeout(timer)
    success = true
  } catch {
    clearTimeout(timer)
    const saved = await directUpdateChar(book.value.id, updatedChar)
    if (saved) success = true
  }
  if (success) {
    const idx = (book.value as Book).characters.findIndex(c => c.id === updatedChar.id)
    if (idx !== -1) (book.value as Book).characters.splice(idx, 1, updatedChar)
  }
  selectedEdgeForEdit.value = null
  rebuildModalEdges()
}
const showRelModal = ref(false)

// Drag state
const dragIdx = ref<number | null>(null)
const dragOverIdx = ref<number | null>(null)

const charForm = ref({ name: '', status: '待出场', group: '', description: '', age: '', appearance: '', personality: '', background: '', relationships: [] as CharRelationship[] })

const relForm = ref({ fromId: '', toId: '', type: '朋友' })

const filteredChars = computed<Character[]>(() => {
  const chars = allChars.value
  if (!filterStatus.value) return chars
  return chars.filter(c => c.status === filterStatus.value)
})

const graphNodes = ref<GraphNode[]>([])
const modalGraphNodes = ref<GraphNode[]>([])
const edges = ref<GraphEdge[]>([])
const modalEdges = ref<GraphEdge[]>([])

// Flattened relationships for graph display
const graphRels = computed(() => {
  const result: { id: string; from: string; type: string; to: string; charId: string; relIdx: number; rel: CharRelationship }[] = []
  for (const c of allChars.value) {
    for (let i = 0; i < (c.relationships ?? []).length; i++) {
      const rel = c.relationships![i]
      const target = allChars.value.find(x => x.id === rel.target_id)
      if (target) {
        result.push({ id: `${c.id}-${rel.target_id}`, from: c.name, type: rel.rel_type ?? (rel as any).type, to: target.name, charId: c.id, relIdx: i, rel })
      }
    }
  }
  return result
})

const svgEl = ref<SVGElement | null>(null)
const svgModalEl = ref<SVGElement | null>(null)
const openGraphModal = ref(false)

// Graph filter
const graphFilterChar = ref<string | null>(null)

// Filtered graph data with centered layout
const filteredModalNodes = computed(() => {
  const filter = graphFilterChar.value
  const allNodes = modalGraphNodes.value
  const allEdges = modalEdges.value
  if (!filter) return { nodes: allNodes, edges: allEdges }

  const selectedNode = allNodes.find(n => n.id === filter)
  if (!selectedNode) return { nodes: [], edges: [] }

  const svg = svgModalEl.value
  const rect = svg ? svg.getBoundingClientRect() : null
  const cx = rect ? rect.width / 2 : 400
  const cy = rect ? rect.height / 2 : 300

  const relatedIds = new Set<string>()
  allEdges.forEach(e => {
    if (e.fromId === filter || e.toId === filter) {
      relatedIds.add(e.fromId)
      relatedIds.add(e.toId)
    }
  })

  const visibleEdges = allEdges.filter(e => relatedIds.has(e.fromId) && relatedIds.has(e.toId))
  const otherNodes = allNodes.filter(n => n.id !== filter && relatedIds.has(n.id))
  const r = Math.max(100, otherNodes.length === 1 ? 120 : Math.round(45 * Math.sqrt(otherNodes.length)))
  const positioned: GraphNode[] = [{ ...selectedNode, x: cx, y: cy }]
  otherNodes.forEach((n, i) => {
    const angle = otherNodes.length === 1 ? 0 : (2 * Math.PI * i) / otherNodes.length - Math.PI / 2
    positioned.push({ ...n, x: cx + r * Math.cos(angle), y: cy + r * Math.sin(angle) })
  })

  const edgeMap: Record<string, GraphNode> = {}
  positioned.forEach(n => { edgeMap[n.id] = n })
  const rebuiltEdges: GraphEdge[] = visibleEdges.map(e => {
    const src = edgeMap[e.fromId]
    const tgt = edgeMap[e.toId]
    if (!src || !tgt) return null
    return { ...e, x1: src.x, y1: src.y, x2: tgt.x, y2: tgt.y, mx: (src.x + tgt.x) / 2, my: (src.y + tgt.y) / 2 }
  }).filter(Boolean) as GraphEdge[]

  return { nodes: positioned, edges: rebuiltEdges }
})

// Watch filter changes → center view on selected node
watch(graphFilterChar, (charId) => {
  if (!svgModalEl.value) return
  nextTick().then(() => {
    if (!charId) { panX.value = 0; panY.value = 0; return }
    const svg = svgModalEl.value
    const rect = svg.getBoundingClientRect()
    const cx = rect.width / 2
    const cy = rect.height / 2
    panX.value = rect.width / 2 - cx
    panY.value = rect.height / 2 - cy
  })
})

// Drag state
const draggingNodeId = ref<string | null>(null)
const dragOffX = ref(0)
const dragOffY = ref(0)
let didDrag = false
let dragStartX = 0
let dragStartY = 0
let justDraggedId: string | null = null

// Canvas pan
const panX = ref(0)
const panY = ref(0)
let isPanning = false
let isDraggingActive = false
let panStartX = 0
let panStartY = 0

function deselectAll() {
  selectedNodesForRel.value = []
  pendingRel.value = null
  pendingRelLine.value = null
  selectedEdgeForEdit.value = null
}

function startNodeDrag(e: MouseEvent, node: GraphNode) {
  e.stopPropagation()
  didDrag = false
  isDraggingActive = true
  console.log("[startNodeDrag] node:", node.id, "fullName:", node.fullName)
  draggingNodeId.value = node.id
  const rect = svgModalEl.value!.getBoundingClientRect()
  dragOffX.value = e.clientX - rect.left - node.x
  dragOffY.value = e.clientY - rect.top - node.y
  dragStartX = e.clientX
  dragStartY = e.clientY
  if (!selectedNodesForRel.value.includes(node.id)) {
    toggleNodeForRel(node)
  }
}

function rebuildModalEdges() {
  const svg = svgModalEl.value
  if (!svg) return
  const chars = allChars.value
  const nodes = modalGraphNodes.value
  modalEdges.value = []
  chars.forEach(c => {
    (c.relationships ?? []).forEach(rel => {
      const src = nodes.find(n => n.id === c.id)
      const tgt = nodes.find(n => n.id === rel.target_id)
      if (src && tgt) {
        modalEdges.value.push({
          id: `${c.id}-${rel.target_id}`,
          x1: src.x, y1: src.y, x2: tgt.x, y2: tgt.y,
          mx: (src.x + tgt.x) / 2, my: (src.y + tgt.y) / 2,
          color: '#4a5568',
          fromId: c.id,
          toId: rel.target_id,
          type: rel.rel_type ?? (rel as any).type
        })
      }
    })
  })
}

function onSvgMouseUp() {
  const wasDragging = !!draggingNodeId.value
  if (draggingNodeId.value) {
    justDraggedId = draggingNodeId.value
    saveGraphLayout()
  }
  draggingNodeId.value = null
  didDrag = false
  isPanning = false
  isDraggingActive = false
  if (!wasDragging) {
    selectedNodesForRel.value = []
    pendingRel.value = null
    pendingRelLine.value = null
  }
  setTimeout(() => { justDraggedId = null }, 0)
}

function onSvgPanStart(e: MouseEvent) {
  // Only start pan when clicking on background rect, not on nodes/edges
  const target = e.target as SVGElement
  if (target.tagName === 'rect' && target.getAttribute('fill') === 'transparent') {
    isPanning = true
    panStartX = e.clientX - panX.value
    panStartY = e.clientY - panY.value
    e.stopPropagation()
  }
}

function onSvgPanMove(e: MouseEvent) {
  // Handle node drag (highest priority)
  if (draggingNodeId.value && svgModalEl.value) {
    if (!didDrag) {
      if (Math.abs(e.clientX - dragStartX) > 4 || Math.abs(e.clientY - dragStartY) > 4) didDrag = true
    }
    const rect = svgModalEl.value.getBoundingClientRect()
    const nx = e.clientX - rect.left - dragOffX.value
    const ny = e.clientY - rect.top - dragOffY.value
    const node = modalGraphNodes.value.find(n => n.id === draggingNodeId.value)
    if (node) {
      node.x = nx; node.y = ny
      rebuildModalEdges()
      saveGraphLayout()
    }
  }
  // Handle canvas pan
  if (isPanning) {
    panX.value = e.clientX - panStartX
    panY.value = e.clientY - panStartY
  }
}

function getCharColor(id: string): string {
  let hash = 0
  for (let i = 0; i < id.length; i++) hash = id.charCodeAt(i) + ((hash << 5) - hash)
  return CHAR_COLORS[Math.abs(hash) % CHAR_COLORS.length]
}

function computeGraph(W: number, H: number): { nodes: GraphNode[], edges: GraphEdge[] } {
  const chars = allChars.value
  if (!chars.length || !W || !H) return { nodes: [], edges: [] }
  const cx = W / 2, cy = H / 2, r = Math.min(W, H) * 0.4
  const nodes: GraphNode[] = chars.map((c, i) => {
    const angle = chars.length === 1 ? 0 : (2 * Math.PI * i) / chars.length - Math.PI / 2
    return { id: c.id, x: cx + r * Math.cos(angle), y: cy + r * Math.sin(angle), label: c.name.slice(0, 2), fullName: c.name, color: getCharColor(c.id), textColor: '#e8e8e8', selected: false }
  })
  const edgeList: GraphEdge[] = []
  chars.forEach(c => {
    (c.relationships ?? []).forEach((rel) => {
      const src = nodes.find(n => n.id === c.id)
      const tgt = nodes.find(n => n.id === rel.target_id)
      if (src && tgt) edgeList.push({ id: `${c.id}-${rel.target_id}`, x1: src.x, y1: src.y, x2: tgt.x, y2: tgt.y, mx: (src.x + tgt.x) / 2, my: (src.y + tgt.y) / 2, color: '#4a5568', type: rel.rel_type ?? (rel as any).type, fromId: c.id, toId: rel.target_id })
    })
  })
  return { nodes, edges: edgeList }
}

function updateGraph() {
  const el = svgEl.value; if (!el) return
  const rect = el.getBoundingClientRect(); if (!rect.width) return
  const result = computeGraph(rect.width, rect.height)
  graphNodes.value = result.nodes; edges.value = result.edges
}

function saveGraphLayout() {
  if (!book.value) return
  const key = `graph_layout_${book.value.id}`
  const positions: Record<string, {x: number, y: number}> = {}
  modalGraphNodes.value.forEach(n => { positions[n.id] = { x: n.x, y: n.y } })
  localStorage.setItem(key, JSON.stringify(positions))
}

function updateModalGraph() {
  const el = svgModalEl.value; if (!el) return
  const rect = el.getBoundingClientRect()
  if (!rect.width || !rect.height) {
    setTimeout(updateModalGraph, 200)
    return
  }
  const result = computeGraph(rect.width, rect.height)
  // Apply saved positions directly to result.nodes (avoids relying on modalGraphNodes state)
  if (book.value) {
    const saved = localStorage.getItem(`graph_layout_${book.value.id}`)
    if (saved) {
      try {
        const positions: Record<string, {x: number, y: number}> = JSON.parse(saved)
        result.nodes.forEach(n => {
          if (positions[n.id]) { n.x = positions[n.id].x; n.y = positions[n.id].y }
        })
      } catch {}
    }
  }
  modalGraphNodes.value = result.nodes
  rebuildModalEdges()
}

watch(openGraphModal, (open) => {
  if (open) {
    nextTick().then(() => {
      // Give modal time to render and ResizeObserver to fire
      setTimeout(updateModalGraph, 250)
    })
  } else {
    modalGraphNodes.value = []; modalEdges.value = []
  }
})

function closeGraphModal() { openGraphModal.value = false; panX.value = 0; panY.value = 0 }

// Drag-and-drop
function onDragStart(idx: number, e: DragEvent) {
  dragIdx.value = idx
  if (e.dataTransfer) e.dataTransfer.effectAllowed = 'move'
}

function onDragOver(idx: number) { dragOverIdx.value = idx }

async function onDrop(dropIdx: number) {
  if (dragIdx.value === null || dragIdx.value === dropIdx) return
  const arr = [...filteredChars.value]
  const [moved] = arr.splice(dragIdx.value, 1)
  arr.splice(dropIdx, 0, moved)
  // Update local
  if (book.value) {
    const reordered = [...(book.value.characters ?? [])]
    const fromId = moved.id
    const realFromIdx = reordered.findIndex(c => c.id === fromId)
    if (realFromIdx !== -1) {
      reordered.splice(realFromIdx, 1)
      const targetId = arr[dropIdx].id
      const realToIdx = reordered.findIndex(c => c.id === targetId)
      reordered.splice(realToIdx !== -1 ? realToIdx : reordered.length, 0, moved)
      book.value.characters = reordered
    }
  }
  if (book.value) await reorderCharacters(book.value.id, allChars.value.map(c => c.id))
  dragIdx.value = null; dragOverIdx.value = null
  setTimeout(updateGraph, 80)
}

function onDragEnd() { dragIdx.value = null; dragOverIdx.value = null }

function selectChar(char: Character) {
  selectedCharId.value = char.id === selectedCharId.value ? null : char.id
  graphNodes.value.forEach(n => { n.selected = n.id === selectedCharId.value })
}

function openAddChar() {
  editingChar.value = null
  charForm.value = { name: '', status: '待出场', group: '', description: '', age: '', appearance: '', personality: '', background: '', relationships: [] }
  activeTab.value = '基本信息'
  showAddChar.value = true
}

function editChar(char: Character) {
  editingChar.value = char
  charForm.value = {
    name: char.name, status: char.status,
    group: char.group ?? '', description: char.description,
    age: char.age ?? '', appearance: char.appearance ?? '',
    personality: char.personality ?? '', background: char.background ?? '',
    relationships: char.relationships ?? []
  }
  activeTab.value = '基本信息'
  showAddChar.value = true
}

async function saveChar() {
  if (!charForm.value.name.trim() || !book.value) return
  if (editingChar.value) {
    const merged: Character = { ...editingChar.value, name: charForm.value.name, status: charForm.value.status as any, group: charForm.value.group, description: charForm.value.description, age: charForm.value.age, appearance: charForm.value.appearance, personality: charForm.value.personality, background: charForm.value.background, relationships: charForm.value.relationships }
    await updateCharacter(book.value.id, merged)
  } else {
    await createCharacter(book.value.id, {
      id: crypto.randomUUID(), name: charForm.value.name,
      status: charForm.value.status as any, group: charForm.value.group,
      description: charForm.value.description, age: charForm.value.age,
      appearance: charForm.value.appearance, personality: charForm.value.personality,
      background: charForm.value.background, relationships: charForm.value.relationships,
      appear_chapters: []
    })
  }
  showAddChar.value = false; editingChar.value = null
  setTimeout(updateGraph, 80)
}

async function deleteCharById(id: string) {
  if (!book.value) return
  await deleteCharacter(book.value.id, id)
  if (selectedCharId.value === id) selectedCharId.value = null
  setTimeout(updateGraph, 80)
}

function addRelForm() {
  charForm.value.relationships.push({ rel_type: '朋友', target_id: '' })
}

function removeRelForm(idx: number) {
  charForm.value.relationships.splice(idx, 1)
}

function openRelModal() { showRelModal.value = true; relForm.value = { fromId: '', toId: '', type: '朋友' } }

async function addRelGraph() {
  if (!relForm.value.fromId || !relForm.value.toId || !book.value || relForm.value.fromId === relForm.value.toId) return
  const char = allChars.value.find(c => c.id === relForm.value.fromId)
  if (!char) return
  const newRel: CharRelationship = { rel_type: relForm.value.type, target_id: relForm.value.toId }
  const updated: Character = { ...char, relationships: [...(char.relationships ?? []), newRel] }
  await updateCharacter(book.value.id, updated)
  showRelModal.value = false
  setTimeout(updateModalGraph, 80)
}

async function removeRel(rel: { charId: string; relIdx: number }) {
  if (!book.value) return
  const char = allChars.value.find(c => c.id === rel.charId)
  if (!char) return
  const newRels = [...(char.relationships ?? [])]
  newRels.splice(rel.relIdx, 1)
  const updated: Character = { ...char, relationships: newRels }
  await updateCharacter(book.value.id, updated)
  setTimeout(updateModalGraph, 80)
}

async function changeRelFrom(rel: any, e: Event) {
  if (!book.value) return
  const newFromId = (e.target as HTMLSelectElement).value
  const oldChar = allChars.value.find(c => c.id === rel.charId)
  const newChar = allChars.value.find(c => c.id === newFromId)
  if (!oldChar || !newChar) return
  // Remove from old character
  const oldRels = [...(oldChar.relationships ?? [])]
  oldRels.splice(rel.relIdx, 1)
  await updateCharacter(book.value.id, { ...oldChar, relationships: oldRels })
  // Add to new character
  const newRels = [...(newChar.relationships ?? []), { rel_type: rel.rel.rel_type, target_id: rel.rel.target_id }]
  await updateCharacter(book.value.id, { ...newChar, relationships: newRels })
  setTimeout(updateModalGraph, 80)
}

async function changeRelType(rel: any, e: Event) {
  if (!book.value) return
  const newType = (e.target as HTMLSelectElement).value
  const char = allChars.value.find(c => c.id === rel.charId)
  if (!char) return
  const newRels = [...(char.relationships ?? [])]
  newRels[rel.relIdx] = { ...rel.rel, rel_type: newType }
  const updated: Character = { ...char, relationships: newRels }
  await updateCharacter(book.value.id, updated)
  setTimeout(updateModalGraph, 80)
}

async function changeRelTo(rel: any, e: Event) {
  if (!book.value) return
  const newToId = (e.target as HTMLSelectElement).value
  const char = allChars.value.find(c => c.id === rel.charId)
  if (!char) return
  const newRels = [...(char.relationships ?? [])]
  newRels[rel.relIdx] = { ...rel.rel, target_id: newToId }
  const updated: Character = { ...char, relationships: newRels }
  await updateCharacter(book.value.id, updated)
  setTimeout(updateModalGraph, 80)
}
</script>

<style scoped>
.chars-panel { display: flex; flex-direction: column; height: 100%; overflow: hidden; }
.chars-list-area { flex: 1; overflow-y: auto; min-height: 0; }
.chars-toolbar { display: flex; align-items: center; gap: 8px; padding: 8px 12px; border-bottom: 1px solid var(--border); flex-shrink: 0; }
.chars-filter { background: var(--surface); border: 1px solid var(--border); border-radius: 5px; color: var(--text); font-size: 12px; padding: 4px 8px; outline: none; }
.chars-count { flex: 1; font-size: 11px; color: var(--muted); text-align: right; }
.btn-sm { padding: 5px 10px; background: var(--surface); border: 1px solid var(--border); border-radius: 5px; color: var(--gold); font-size: 11px; cursor: pointer; white-space: nowrap; }
.chars-grid { display: flex; flex-direction: column; gap: 4px; padding: 8px 10px; }
.char-card-grid { display: flex; flex-direction: row; align-items: center; gap: 8px; padding: 6px 8px; background: var(--surface); border: 1px solid var(--border); border-radius: 6px; cursor: pointer; transition: border-color 0.2s; position: relative; }
.char-card-grid:hover { border-color: var(--gold); }
.char-card-grid.selected { border-color: var(--selected-text); background: var(--selected-bg); }
.char-avatar { width: 26px; height: 26px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 700; color: #fff; flex-shrink: 0; }
.char-info { flex: 1; display: flex; flex-direction: row; align-items: center; gap: 6px; min-width: 0; }
.char-name-row { display: flex; align-items: center; gap: 4px; flex-shrink: 0; }
.char-name-text { font-size: 11px; font-weight: 600; color: var(--text); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.char-status-dot { font-size: 8px; flex-shrink: 0; }
.char-status-dot.\待出场 { color: #666; }
.char-status-dot.\已出场 { color: var(--green); }
.char-status-dot.\重要 { color: var(--gold); }
.char-status-dot.\已下线 { color: var(--red); }
.char-desc-text { font-size: 9px; color: var(--muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; }
.char-actions { display: flex; gap: 4px; opacity: 0; transition: opacity 0.15s; position: absolute; top: 50%; right: 8px; transform: translateY(-50%); }
.char-card-grid:hover .char-actions { opacity: 1; }
.char-actions button { background: none; border: none; font-size: 11px; cursor: pointer; padding: 2px; }
.chars-empty { text-align: center; color: var(--muted); font-size: 12px; padding: 30px 0; }
.graph-area { flex-shrink: 0; height: 160px; border-top: 1px solid var(--border); background: var(--bg); cursor: pointer; position: relative; overflow: hidden; }
.graph-label { position: absolute; top: 6px; left: 10px; font-size: 10px; color: var(--muted); z-index: 1; pointer-events: none; }
.graph-svg { width: 100%; height: 100%; }
.graph-modal { background: var(--surface); border: 1px solid var(--border); border-radius: 12px; width: 90vw; height: 80vh; max-width: 900px; max-height: 700px; display: flex; flex-direction: column; overflow: hidden; position: relative; }
.graph-modal-header { display: flex; align-items: center; justify-content: space-between; padding: 14px 20px; border-bottom: 1px solid var(--border); font-size: 14px; font-weight: 600; color: var(--gold); flex-shrink: 0; }
.graph-deselect-hint { position: absolute; bottom: 16px; left: 50%; transform: translateX(-50%); background: rgba(20,16,10,0.8); color: #c9a96e; font-size: 11px; padding: 4px 12px; border-radius: 12px; pointer-events: none; z-index: 5; }
.graph-filter-select { background: var(--surface); border: 1px solid var(--border); border-radius: 4px; color: var(--text); font-size: 12px; padding: 3px 8px; outline: none; max-width: 120px; }
.graph-rels-bar { display: flex; flex-wrap: wrap; gap: 6px; padding: 8px 16px; border-bottom: 1px solid var(--border); flex-shrink: 0; max-height: 80px; overflow-y: auto; }
.graph-rel-chip { display: flex; align-items: center; gap: 4px; background: var(--bg); border: 1px solid var(--border); border-radius: 12px; padding: 3px 8px; font-size: 11px; }
.graph-rel-chip .rel-type { color: var(--gold); font-weight: 600; }
.graph-rel-chip button { background: none; border: none; font-size: 10px; cursor: pointer; color: var(--muted); padding: 0 2px; }
.graph-rels-empty { padding: 6px 16px; font-size: 11px; color: var(--muted); border-bottom: 1px solid var(--border); flex-shrink: 0; }
.rel-char-select { background: var(--surface); border: 1px solid var(--border); border-radius: 4px; color: var(--text); font-size: 11px; padding: 1px 4px; outline: none; max-width: 80px; }
.rel-type-select-inline { background: var(--surface); border: 1px solid var(--gold); border-radius: 4px; color: var(--gold); font-size: 11px; padding: 1px 4px; outline: none; }
.graph-hint-bar { display: flex; align-items: center; flex-wrap: wrap; gap: 8px; padding: 8px 16px; border-bottom: 1px solid var(--border); flex-shrink: 0; font-size: 12px; color: var(--muted); min-height: 36px; }
.graph-modal-svg { flex: 1; min-height: 0; background: var(--bg); display: block; }
.rel-quick-select { background: var(--surface); border: 1px solid var(--gold); border-radius: 4px; color: var(--gold); font-size: 12px; padding: 2px 6px; outline: none; }
.btn-danger { color: var(--red) !important; border-color: var(--red) !important; }
.modal-edge { cursor: pointer; }
.char-form-tabs { display: flex; gap: 4px; margin-bottom: 12px; border-bottom: 1px solid var(--border); padding-bottom: 8px; }
.char-form-tabs button { background: none; border: none; color: var(--muted); font-size: 12px; cursor: pointer; padding: 4px 8px; border-radius: 4px; transition: background 0.15s; }
.char-form-tabs button.active { background: var(--selected-bg); color: var(--gold); font-weight: 600; }
.rel-empty { color: var(--muted); font-size: 12px; text-align: center; padding: 8px 0; }
.rel-item { display: flex; align-items: center; gap: 6px; margin-bottom: 6px; }
.rel-type-select { flex: 0 0 80px; background: var(--surface); border: 1px solid var(--border); border-radius: 4px; color: var(--text); font-size: 12px; padding: 4px; outline: none; }
.rel-target-input { flex: 1; background: var(--surface); border: 1px solid var(--border); border-radius: 4px; color: var(--text); font-size: 12px; padding: 4px; outline: none; }
.rel-del { background: none; border: none; color: var(--muted); font-size: 11px; cursor: pointer; padding: 2px 4px; }
.rel-del:hover { color: var(--red); }
.close-btn { background: none; border: none; font-size: 18px; color: var(--muted); cursor: pointer; padding: 0 4px; line-height: 1; }
.close-btn:hover { color: var(--text); }
</style>
