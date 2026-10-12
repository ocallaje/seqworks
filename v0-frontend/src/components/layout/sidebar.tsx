import { NavLink } from 'react-router-dom'
import { LayoutDashboard, LogOut, Moon, Sun } from 'lucide-react'
import { Logo } from '@/components/logo'
import { pipelines } from '@/pipelines/registry'
import { useAuth } from '@/lib/auth'
import { useRuns } from '@/lib/api'
import { useTheme } from '@/lib/theme'
import { cn } from '@/lib/utils'

const navItem =
  'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground'
const navActive = 'bg-card text-foreground shadow-sm ring-1 ring-border hover:bg-card'

export function Sidebar({ onNavigate }: { onNavigate?: () => void }) {
  const { session, logout } = useAuth()
  const { runs } = useRuns()
  const { theme, toggle } = useTheme()

  const activeByPipeline = runs.reduce<Record<string, number>>((acc, r) => {
    if (r.phase === 'Running' || r.phase === 'Pending') acc[r.pipelineId] = (acc[r.pipelineId] ?? 0) + 1
    return acc
  }, {})

  const initials = (session?.user.name ?? '?')
    .split(/[\s._-]+/)
    .map((p) => p[0])
    .join('')
    .slice(0, 2)
    .toUpperCase()

  return (
    <div className="flex h-full flex-col bg-sidebar">
      <div className="flex h-16 items-center px-5">
        <Logo />
      </div>

      <nav className="flex flex-1 flex-col gap-6 overflow-y-auto px-3 py-2" aria-label="Main">
        <NavLink to="/" end onClick={onNavigate} className={({ isActive }) => cn(navItem, isActive && navActive)}>
          <LayoutDashboard className="size-4" aria-hidden />
          Dashboard
        </NavLink>

        <div className="flex flex-col gap-1">
          <h2 className="px-3 pb-1 text-xs font-medium tracking-wide text-muted-foreground uppercase">Pipelines</h2>
          {pipelines.map((p) => {
            const active = activeByPipeline[p.id]
            return (
              <NavLink
                key={p.id}
                to={`/pipelines/${p.id}`}
                onClick={onNavigate}
                className={({ isActive }) => cn(navItem, 'items-start py-2.5', isActive && navActive)}
              >
                {({ isActive }) => (
                  <>
                    <span
                      className={cn(
                        'mt-1.5 size-1.5 shrink-0 rounded-full',
                        isActive ? 'bg-primary' : 'bg-muted-foreground/30',
                      )}
                      aria-hidden
                    />
                    <span className="flex min-w-0 flex-1 flex-col gap-0.5">
                      <span className="truncate">{p.name}</span>
                      <span className="truncate font-mono text-xs font-normal text-muted-foreground">
                        {p.workflow}@{p.version}
                      </span>
                    </span>
                    {active ? (
                      <span className="rounded-full bg-primary/10 px-1.5 py-0.5 text-xs font-medium tabular-nums text-primary">
                        {active}
                        <span className="sr-only"> active runs</span>
                      </span>
                    ) : null}
                  </>
                )}
              </NavLink>
            )
          })}
        </div>
      </nav>

      <div className="flex items-center gap-3 border-t px-4 py-4">
        <span className="flex size-8 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-semibold text-foreground">
          {initials}
        </span>
        <div className="flex min-w-0 flex-1 flex-col">
          <span className="truncate text-sm font-medium text-foreground">{session?.user.name}</span>
          <span className="truncate text-xs text-muted-foreground">
            {session?.method === 'oidc' ? 'GeneLink SSO' : 'Service account'}
          </span>
        </div>
        <button
          type="button"
          onClick={toggle}
          className="rounded-md p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
          aria-label={theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'}
        >
          {theme === 'dark' ? <Sun className="size-4" /> : <Moon className="size-4" />}
        </button>
        <button
          type="button"
          onClick={() => void logout()}
          className="rounded-md p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
          aria-label="Sign out"
        >
          <LogOut className="size-4" />
        </button>
      </div>
    </div>
  )
}
