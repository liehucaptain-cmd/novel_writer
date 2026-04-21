import { ref } from 'vue'

export type ThemeMode = 'dark' | 'light' | 'eye'

export function useTheme() {
  // 初始值从 localStorage 读取，默认 dark
  const theme = ref<ThemeMode>(
    (localStorage.getItem('theme-mode') as ThemeMode) || 'dark'
  )

  const applyTheme = (mode: ThemeMode) => {
    // 直接设置 data-theme 属性，让 CSS 的 [data-theme="xxx"] 规则生效
    // 不再用 inline style，覆盖 CSS 变量
    document.documentElement.setAttribute('data-theme', mode)
  }

  const setTheme = (mode: ThemeMode) => {
    theme.value = mode
    localStorage.setItem('theme-mode', mode)
    applyTheme(mode)
  }

  const toggleTheme = () => {
    const modes: ThemeMode[] = ['dark', 'light', 'eye']
    const idx = modes.indexOf(theme.value)
    setTheme(modes[(idx + 1) % modes.length])
  }

  // 初始化
  applyTheme(theme.value)

  return { theme, setTheme, toggleTheme }
}
