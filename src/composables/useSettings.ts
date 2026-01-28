import { ref, watch } from 'vue'

export type Theme = 'dark' | 'waffle'

const theme = ref<Theme>((localStorage.getItem('theme') as Theme) || 'dark')

watch(theme, (newTheme) => {
  localStorage.setItem('theme', newTheme)
  applyTheme(newTheme)
})

function applyTheme(t: Theme) {
  if (t === 'waffle') {
    document.documentElement.setAttribute('data-theme', 'waffle')
  } else {
    document.documentElement.removeAttribute('data-theme')
  }
}

applyTheme(theme.value)

export function useSettings() {
  function toggleTheme() {
    theme.value = theme.value === 'dark' ? 'waffle' : 'dark'
  }

  function setTheme(t: Theme) {
    theme.value = t
  }

  return {
    theme,
    toggleTheme,
    setTheme,
  }
}
