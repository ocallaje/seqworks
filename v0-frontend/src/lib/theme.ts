import { useCallback, useSyncExternalStore } from 'react'

type Theme = 'light' | 'dark'
const KEY = 'genelink.theme'

function getTheme(): Theme {
  return document.documentElement.classList.contains('dark') ? 'dark' : 'light'
}

const listeners = new Set<() => void>()

function subscribe(cb: () => void) {
  listeners.add(cb)
  return () => listeners.delete(cb)
}

export function useTheme() {
  const theme = useSyncExternalStore(subscribe, getTheme, () => 'light' as Theme)
  const setTheme = useCallback((next: Theme) => {
    document.documentElement.classList.toggle('dark', next === 'dark')
    try {
      localStorage.setItem(KEY, next)
    } catch {
      // storage unavailable (private mode)
    }
    for (const l of listeners) l()
  }, [])
  const toggle = useCallback(() => setTheme(getTheme() === 'dark' ? 'light' : 'dark'), [setTheme])
  return { theme, setTheme, toggle }
}
