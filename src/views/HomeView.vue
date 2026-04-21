<template>
  <div class="view home-view">
    <div class="view-header">
      <div class="view-title">📚 我的书籍 <span style="font-size:12px;color:var(--muted);font-weight:400">({{ books.length }} 本)</span></div>
      <button class="btn btn-gold" @click="showNewModal = true">+ 新建书籍</button>
    </div>

    <div v-if="loading" style="text-align:center;padding:40px;color:var(--muted)">加载中...</div>
    <div v-else-if="books.length === 0" style="text-align:center;padding:40px;color:var(--muted)">
      暂无书籍，点击下方按钮创建第一本
    </div>
    <div class="book-grid" v-else>
      <BookCard
        v-for="(book, idx) in books"
        :key="book.id"
        :book="book"
        @click="openBook(book)"
        @rename="startRename(book)"
        @delete="confirmDelete(book)"
        @move-top="moveBookTop(idx)"
        @move-up="moveBookUp(idx)"
        @move-down="moveBookDown(idx)"
      />
      <BookCard :is-new="true" @new="showNewModal = true" />
    </div>

    <!-- 新建书籍弹窗 -->
    <Modal v-model="showNewModal" title="新建书籍">
      <div class="form-group">
        <label>书名</label>
        <input v-model="newBookName" placeholder="请输入书名" @keydown.enter="doCreateBook" />
      </div>
      <div class="modal-actions">
        <button class="btn" @click="showNewModal = false">取消</button>
        <button class="btn btn-gold" @click="doCreateBook">创建</button>
      </div>
    </Modal>

    <!-- 删除确认弹窗 -->
    <Modal v-model="showDeleteModal" title="删除书籍">
      <p style="color: var(--text); font-size: 14px; margin-bottom: 16px;">
        确定要删除「{{ bookToDelete?.name }}」吗？此操作不可恢复。
      </p>
      <div class="modal-actions">
        <button class="btn" @click="showDeleteModal = false">取消</button>
        <button class="btn btn-danger" @click="doDeleteBook">删除</button>
      </div>
    </Modal>

    <!-- 重命名弹窗 -->
    <Modal v-if="bookToRename" v-model="showRenameModal" title="重命名书籍">
      <div class="form-group">
        <label>书名</label>
        <input v-model="renameValue" @keydown.enter="doRenameBook" />
      </div>
      <div class="modal-actions">
        <button class="btn" @click="showRenameModal = false">取消</button>
        <button class="btn btn-gold" @click="doRenameBook">保存</button>
      </div>
    </Modal>

    <!-- Toast -->
    <Teleport to="body">
      <div class="toast" :class="{ show: toast.visible }">{{ toast.message }}</div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useNovel } from '../composables/useNovel'
import BookCard from '../components/shared/BookCard.vue'
import Modal from '../components/shared/Modal.vue'
import type { Book } from '../types'

const emit = defineEmits(['open-book'])
const { books, loading, createBook, updateBook, deleteBook, reorderBooks, loadBooks } = useNovel()

onMounted(() => {
  loadBooks()
})

const showNewModal = ref(false)
const showDeleteModal = ref(false)
const showRenameModal = ref(false)
const newBookName = ref('')
const renameValue = ref('')
const bookToDelete = ref<Book | null>(null)
const bookToRename = ref<Book | null>(null)

// Move book helpers
function moveBookUp(idx: number) {
  if (idx <= 0) return
  const newBooks = [...books.value]
  ;[newBooks[idx - 1], newBooks[idx]] = [newBooks[idx], newBooks[idx - 1]]
  books.value = newBooks
  reorderBooks(newBooks.map(b => b.id))
}

function moveBookDown(idx: number) {
  if (idx >= books.value.length - 1) return
  const newBooks = [...books.value]
  ;[newBooks[idx], newBooks[idx + 1]] = [newBooks[idx + 1], newBooks[idx]]
  books.value = newBooks
  reorderBooks(newBooks.map(b => b.id))
}

function moveBookTop(idx: number) {
  if (idx <= 0) return
  const newBooks = [...books.value]
  const [moved] = newBooks.splice(idx, 1)
  newBooks.unshift(moved)
  books.value = newBooks
  reorderBooks(newBooks.map(b => b.id))
}

const toast = reactive({ visible: false, message: '' })

function showToast(msg: string) {
  toast.message = msg
  toast.visible = true
  setTimeout(() => { toast.visible = false }, 3000)
}

async function doCreateBook() {
  if (!newBookName.value.trim()) return
  await createBook(newBookName.value.trim())
  newBookName.value = ''
  showNewModal.value = false
  showToast('书籍创建成功')
}

function startRename(book: Book) {
  bookToRename.value = book
  renameValue.value = book.name
  showRenameModal.value = true
}

async function doRenameBook() {
  if (!renameValue.value.trim() || !bookToRename.value) return
  await updateBook(bookToRename.value.id, renameValue.value.trim())
  showRenameModal.value = false
  bookToRename.value = null
  showToast('书名已更新')
}

function confirmDelete(book: Book) {
  bookToDelete.value = book
  showDeleteModal.value = true
}

async function doDeleteBook() {
  if (!bookToDelete.value) return
  await deleteBook(bookToDelete.value.id)
  showDeleteModal.value = false
  bookToDelete.value = null
  showToast('已删除')
}

function openBook(book: Book) {
  console.log('[openBook] emitting open-book for:', book.name)
  emit('open-book', book)
}
</script>

<style scoped>
.book-grid :deep(.book-card) {
  transition: opacity 0.2s, transform 0.2s;
}
.book-grid :deep(.book-card.dragging) {
  opacity: 0.4;
  transform: scale(0.95);
}
.book-grid :deep(.book-card.drag-over) {
  outline: 2px dashed var(--gold);
  outline-offset: 4px;
}
</style>
