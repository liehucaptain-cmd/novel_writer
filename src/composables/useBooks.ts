import { ref } from 'vue'
import { appDataDir } from '@tauri-apps/api/path'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'

// Self-contained old-schema types (for legacy components)
export interface OldBook {
  id: string
  title: string
  description: string
  createdAt: number
  updatedAt: number
  chapters: OldChapter[]
}

export interface OldChapter {
  id: string
  bookId: string
  title: string
  content: string
  order: number
  createdAt: number
  updatedAt: number
  charIds: string[]
  plotlineIds: string[]
}

export { type OldBook as Book, type OldChapter as Chapter }

// ── 单例状态（模块级别，所有 useBooks() 调用共享同一份） ──
const books = ref<OldBook[]>([])
const currentBook = ref<OldBook | null>(null)
const currentChapter = ref<OldChapter | null>(null)

export function useBooks() {

  async function getDataFile() {
    return `${await appDataDir()}novel-data.json`
  }

  async function loadBooks() {
    try {
      const raw = await readTextFile(await getDataFile())
      books.value = JSON.parse(raw)
    } catch {
      books.value = []
    }
  }

  async function saveBooks() {
    await writeTextFile(await getDataFile(), JSON.stringify(books.value, null, 2))
  }

  async function createBook(title: string) {
    const book: OldBook = {
      id: crypto.randomUUID(),
      title,
      description: '',
      createdAt: Date.now(),
      updatedAt: Date.now(),
      chapters: [],
    }
    books.value.push(book)
    await saveBooks()
    return book
  }

  async function deleteBook(bookId: string) {
    books.value = books.value.filter(b => b.id !== bookId)
    if (currentBook.value?.id === bookId) {
      currentBook.value = null
      currentChapter.value = null
    }
    await saveBooks()
  }

  async function createChapter(bookId: string, title: string) {
    const book = books.value.find(b => b.id === bookId)
    if (!book) return null
    const chapter: OldChapter = {
      id: crypto.randomUUID(),
      bookId,
      title,
      content: '',
      order: book.chapters.length,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      charIds: [],
      plotlineIds: [],
    }
    book.chapters.push(chapter)
    book.updatedAt = Date.now()
    await saveBooks()
    return chapter
  }

  async function deleteChapter(bookId: string, chapterId: string) {
    const book = books.value.find(b => b.id === bookId)
    if (!book) return
    book.chapters = book.chapters.filter(c => c.id !== chapterId)
    book.updatedAt = Date.now()
    if (currentChapter.value?.id === chapterId) {
      currentChapter.value = null
    }
    await saveBooks()
  }

  async function selectChapter(bookId: string, chapterId: string) {
    const book = books.value.find(b => b.id === bookId)
    if (!book) return
    currentBook.value = book
    currentChapter.value = book.chapters.find(c => c.id === chapterId) ?? null
  }

  async function updateChapter(chapter: OldChapter) {
    const book = books.value.find(b => b.id === chapter.bookId)
    if (!book) return
    const idx = book.chapters.findIndex(c => c.id === chapter.id)
    if (idx === -1) return
    book.chapters[idx] = { ...chapter, updatedAt: Date.now() }
    book.updatedAt = Date.now()
    currentChapter.value = book.chapters[idx]
    await saveBooks()
  }

  return {
    books,
    currentBook,
    currentChapter,
    loadBooks,
    saveBooks,
    createBook,
    deleteBook,
    createChapter,
    deleteChapter,
    selectChapter,
    updateChapter,
  }
}
