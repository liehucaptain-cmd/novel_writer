import { test, expect, Page } from '@playwright/test'
import path from 'path'
import fs from 'fs'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// ─────────────────────────────────────────
// Screenshot helper — saves with a descriptive name
// ─────────────────────────────────────────
async function snap(page: Page, name: string) {
  const dir = path.join(__dirname, '../test-screenshots')
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true })
  const file = path.join(dir, `${name}.png`)
  await page.screenshot({ path: file, fullPage: false })
  console.log(`📸 截图: ${name}.png`)
}

// ─────────────────────────────────────────
// Mock TAURI invoke()
// ─────────────────────────────────────────
const mockBooks = [
  {
    id: 'test-book-1',
    name: '测试书籍A',
    created_at: '2026-04-01T10:00:00.000000',
    updated_at: '2026-04-01T10:00:00.000000',
    chapters: [],
    characters: [],
    plotlines: [],
  },
  {
    id: 'test-book-2',
    name: '测试书籍B',
    created_at: '2026-04-02T10:00:00.000000',
    updated_at: '2026-04-02T10:00:00.000000',
    chapters: [],
    characters: [],
    plotlines: [],
  },
]

let createdBooks = [...mockBooks]

test.beforeEach(async ({ page }) => {
  await page.addInitScript((books) => {
    ;(window as any).__TAURI__ = {
      invoke: async (cmd: string, args?: Record<string, unknown>) => {
        if (cmd === 'get_books') return books
        if (cmd === 'create_book') {
          const newBook = {
            id: `generated-${Date.now()}`,
            name: args?.name as string,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
            chapters: [],
            characters: [],
            plotlines: [],
          }
          books.push(newBook)
          return newBook
        }
        if (cmd === 'delete_book') {
          const idx = books.findIndex((b: any) => b.id === args?.bookId)
          if (idx !== -1) books.splice(idx, 1)
          return null
        }
        if (cmd === 'get_book') {
          return books.find((b: any) => b.id === args?.bookId) ?? null
        }
        throw new Error(`Unknown invoke: ${cmd}`)
      },
    }
  }, createdBooks)
})

// ─────────────────────────────────────────
// TEST SUITE: HomeView 书籍管理
// ─────────────────────────────────────────

test('H01-首页加载', async ({ page }) => {
  await page.goto('/')
  await snap(page, 'H01-home-loaded')
  await expect(page.getByText('📚 我的书籍')).toBeVisible()
})

test('H02-新建书籍弹窗', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: '+ 新建书籍' }).click()
  await snap(page, 'H02-new-book-modal-open')
  await expect(page.getByPlaceholder('请输入书名')).toBeVisible()
  await expect(page.getByRole('button', { name: '创建' })).toBeVisible()
  await expect(page.getByRole('button', { name: '取消' })).toBeVisible()
})

test('H03-输入书名', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: '+ 新建书籍' }).click()
  await page.getByPlaceholder('请输入书名').fill('星际迷途')
  await snap(page, 'H03-new-book-filled')
  // Verify the input has the value
  await expect(page.getByPlaceholder('请输入书名')).toHaveValue('星际迷途')
})

test('H04-取消关闭弹窗', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: '+ 新建书籍' }).click()
  await page.getByRole('button', { name: '取消' }).click()
  await snap(page, 'H04-modal-cancelled')
  await expect(page.getByPlaceholder('请输入书名')).not.toBeVisible()
})

test('H05-关闭按钮关闭弹窗', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: '+ 新建书籍' }).click()
  await page.getByRole('button', { name: '×' }).click()
  await snap(page, 'H05-modal-closed')
  await expect(page.getByPlaceholder('请输入书名')).not.toBeVisible()
})

test('H06-创建书籍成功', async ({ page }) => {
  await page.goto('/')
  const beforeCount = createdBooks.length

  await page.getByRole('button', { name: '+ 新建书籍' }).click()
  await page.getByPlaceholder('请输入书名').fill('新书名测试')
  await page.getByRole('button', { name: '创建' }).click()

  await snap(page, 'H06-after-create-book')
  await expect(page.getByPlaceholder('请输入书名')).not.toBeVisible()
  await expect(page.getByText('新书名测试')).toBeVisible()
  await expect(page.getByText(`(${beforeCount + 1} 本)`)).toBeVisible()
})

test('H07-深色模式切换', async ({ page }) => {
  await page.goto('/')
  await snap(page, 'H07-light-mode')
  const btn = page.getByRole('button', { name: '🌙' })
  await btn.click()
  await snap(page, 'H07-dark-mode')
  await expect(page.getByText('📚 我的书籍')).toBeVisible()
})

// ─────────────────────────────────────────
// 报告生成（测试全部结束后打印截图列表）
// ─────────────────────────────────────────
if (typeof afterAll === 'function') {
  afterAll(() => {
    const dir = path.join(__dirname, '../test-screenshots')
    if (!fs.existsSync(dir)) return
    const files = fs.readdirSync(dir).filter(f => f.endsWith('.png')).sort()
    console.log('\n📋 截图已保存至 test-screenshots/')
    files.forEach(f => console.log(`   - ${f}`))
  })
}
