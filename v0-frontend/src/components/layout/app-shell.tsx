import { useState } from 'react'
import { Outlet } from 'react-router-dom'
import { Menu, X } from 'lucide-react'
import { Sidebar } from './sidebar'
import { Logo } from '@/components/logo'
import { LiveUpdatesProvider } from '@/lib/live'
import { config } from '@/config'

export function AppShell() {
  const [open, setOpen] = useState(false)

  return (
    <LiveUpdatesProvider>
      <div className="flex min-h-dvh">
        <aside className="sticky top-0 hidden h-dvh w-72 shrink-0 border-r lg:block">
          <Sidebar />
        </aside>

        {open ? (
          <div className="fixed inset-0 z-40 lg:hidden" role="dialog" aria-modal="true" aria-label="Navigation">
            <button
              type="button"
              className="absolute inset-0 bg-foreground/30"
              aria-label="Close navigation"
              onClick={() => setOpen(false)}
            />
            <div className="animate-drawer-in absolute inset-y-0 left-0 w-72 border-r shadow-xl">
              <Sidebar onNavigate={() => setOpen(false)} />
            </div>
          </div>
        ) : null}

        <div className="flex min-w-0 flex-1 flex-col">
          {config.demoMode ? (
            <div className="border-b bg-warning/15 px-4 py-1.5 text-center text-xs font-medium text-foreground">
              Demo mode — not connected to an orchestrator. Set API_URL to connect.
            </div>
          ) : null}
          <header className="flex h-14 items-center justify-between border-b px-4 lg:hidden">
            <Logo />
            <button
              type="button"
              onClick={() => setOpen((o) => !o)}
              className="rounded-md p-2 text-muted-foreground hover:bg-muted"
              aria-label={open ? 'Close navigation' : 'Open navigation'}
            >
              {open ? <X className="size-5" /> : <Menu className="size-5" />}
            </button>
          </header>
          <main className="flex-1">
            <Outlet />
          </main>
        </div>
      </div>
    </LiveUpdatesProvider>
  )
}
