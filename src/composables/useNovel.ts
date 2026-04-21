import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Book, Chapter, Character, Plotline, Outline, BookIndex } from '../types'

export function useNovel() {
  const books = ref<Book[]>([])
  const currentBook = ref<Book | null>(null)
  const currentChapter = ref<Chapter | null>(null)
  const currentCharacter = ref<Character | null>(null)
  const currentPlotline = ref<Plotline | null>(null)
  const currentOutline = ref<Outline | null>(null)
  const loading = ref(false)

  async function loadBooks() {
    loading.value = true
    try {
      // Try debug_books first to verify backend data
      try {
        const debug: string = await invoke('debug_books')
        console.log('[Novel] debug_books:', debug)
      } catch (e) {
        console.error('[Novel] debug_books failed:', String(e))
      }
      const index: BookIndex = await invoke('list_books')
      console.log('[Novel] list_books count:', index.books.length)
      const loaded: Book[] = []
      for (const meta of index.books) {
        try {
          const book = await invoke<Book>('get_book', { bookId: meta.id })
          loaded.push(book)
        } catch (e) {
          console.error('[Novel] get_book failed for', meta.id, String(e))
        }
      }
      books.value = loaded
      console.log('[Novel] books.value set, count:', books.value.length)
    } catch (e) {
      console.error('[Novel] loadBooks error:', String(e))
    } finally {
      loading.value = false
    }
  }

  async function loadBook(bookId: string) {
    loading.value = true
    try {
      const bookData: any = await invoke('get_book', { bookId })
      console.log('[loadBook] id:', bookData?.id, 'chapters:', bookData?.chapters?.length, 'chars:', bookData?.characters?.length, 'isRef:', bookData && bookData.__v !== undefined)
      currentBook.value = bookData
      console.log('[loadBook] currentBook.value.id:', currentBook.value?.id, 'chapters:', currentBook.value?.chapters?.length)
    } catch (e) {
      console.error('loadBook error:', e)
    } finally {
      loading.value = false
    }
  }

  async function createBook(name: string) {
    const book: Book = await invoke('create_book', { name })
    books.value.push(book)
    return book
  }

  async function updateBook(bookId: string, name: string) {
    const book: Book = await invoke('update_book', { bookId, name })
    const idx = books.value.findIndex(b => b.id === bookId)
    if (idx !== -1) books.value[idx] = book
    if (currentBook.value?.id === bookId) currentBook.value = book
    return book
  }

  async function reorderBooks(bookIds: string[]) {
    await invoke('reorder_books', { bookIds })
    // Reorder the local books array to match
    const reordered = bookIds.map(id => books.value.find(b => b.id === id)).filter(Boolean) as Book[]
    // Append any not in bookIds
    for (const b of books.value) {
      if (!bookIds.includes(b.id)) reordered.push(b)
    }
    books.value = reordered
  }

  async function deleteBook(bookId: string) {
    await invoke('delete_book', { bookId })
    books.value = books.value.filter(b => b.id !== bookId)
    if (currentBook.value?.id === bookId) {
      currentBook.value = null
      currentChapter.value = null
    }
  }

  async function createChapter(bookId: string, title: string) {
    const chapter: Chapter = await invoke('create_chapter', { bookId, title })
    if (currentBook.value?.id === bookId) {
      currentBook.value.chapters.push(chapter)
    }
    // reload to ensure parent components get fresh data
    if (currentBook.value?.id === bookId) {
      currentBook.value = await invoke('get_book', { bookId })
    }
    return chapter
  }

  async function updateChapter(bookId: string, chapter: Chapter) {
    const updated: Chapter = await invoke('update_chapter', { bookId, chapter })
    if (currentBook.value?.id === bookId) {
      const idx = currentBook.value.chapters.findIndex(c => c.id === chapter.id)
      if (idx !== -1) currentBook.value.chapters[idx] = updated
    }
    if (currentChapter.value?.id === chapter.id) {
      currentChapter.value = updated
    }
    return updated
  }

  async function deleteChapter(bookId: string, chapterId: string) {
    await invoke('delete_chapter', { bookId, chapterId })
    if (currentBook.value?.id === bookId) {
      currentBook.value.chapters = currentBook.value.chapters.filter(c => c.id !== chapterId)
    }
    if (currentChapter.value?.id === chapterId) {
      currentChapter.value = null
    }
  }

  async function reorderChapters(bookId: string, chapterIds: string[]) {
    await invoke('reorder_chapters', { bookId, chapterIds })
    if (currentBook.value?.id === bookId) {
      const reordered: Chapter[] = []
      for (const id of chapterIds) {
        const ch = currentBook.value.chapters.find(c => c.id === id)
        if (ch) reordered.push(ch)
      }
      currentBook.value.chapters = reordered
    }
  }

  async function loadPlotlines(bookId: string) {
    try {
      return await invoke<Plotline[]>('get_plotlines', { bookId })
    } catch (e) {
      console.error('loadPlotlines error:', e)
      return []
    }
  }

  async function savePlotlines(bookId: string, plotlines: Plotline[]) {
    await invoke('save_plotlines', { bookId, plotlines })
    if (currentBook.value?.id === bookId) {
      currentBook.value.storylines = plotlines
    }
  }

  async function loadOutline(bookId: string) {
    try {
      currentOutline.value = await invoke('get_outline', { bookId })
      return currentOutline.value
    } catch (e) {
      console.error('loadOutline error:', e)
      return null
    }
  }

  async function saveOutline(bookId: string, outline: Outline) {
    await invoke('save_outline', { bookId, outline })
    currentOutline.value = outline
  }

  // Character helpers (operate on currentBook)
  async function createCharacter(bookId: string, character: Character) {
    const created: Character = await invoke('create_character', { bookId, character })
    if (currentBook.value?.id === bookId) {
      currentBook.value.characters.push(created)
    }
    return created
  }

  async function updateCharacter(bookId: string, character: Character) {
    const updated: Character = await invoke('update_character', { bookId, character })
    if (currentBook.value?.id === bookId) {
      const idx = currentBook.value.characters.findIndex(c => c.id === character.id)
      if (idx !== -1) currentBook.value.characters[idx] = updated
    }
    if (currentCharacter.value?.id === character.id) {
      currentCharacter.value = updated
    }
    return updated
  }

  async function deleteCharacter(bookId: string, characterId: string) {
    await invoke('delete_character', { bookId, characterId })
    if (currentBook.value?.id === bookId) {
      currentBook.value.characters = currentBook.value.characters.filter(c => c.id !== characterId)
    }
    if (currentCharacter.value?.id === characterId) {
      currentCharacter.value = null
    }
  }

  async function reorderCharacters(bookId: string, charIds: string[]) {
    await invoke('reorder_characters', { bookId, charIds })
    if (currentBook.value?.id === bookId) {
      const reordered = charIds.map(id => currentBook.value!.characters.find(c => c.id === id)).filter(Boolean) as Character[]
      for (const c of currentBook.value!.characters) {
        if (!charIds.includes(c.id)) reordered.push(c)
      }
      currentBook.value.characters = reordered
    }
  }

  // Watch currentBook to verify reactivity
  watch(currentBook, (b) => {
    console.log('[currentBook] watch triggered, id:', b?.id, 'chapters:', b?.chapters?.length)
  }, { deep: true })

  return {
    books,
    currentBook,
    currentChapter,
    currentCharacter,
    currentPlotline,
    currentOutline,
    loading,
    loadBooks,
    loadBook,
    createBook,
    updateBook,
    deleteBook,
    reorderBooks,
    createChapter,
    updateChapter,
    deleteChapter,
    reorderChapters,
    loadPlotlines,
    savePlotlines,
    loadOutline,
    saveOutline,
    createCharacter,
    updateCharacter,
    deleteCharacter,
    reorderCharacters,
  }
}
