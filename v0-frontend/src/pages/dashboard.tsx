import { useMemo, useState } from 'react'
import { Link, useSearchParams } from 'react-router-dom'
import { AlertTriangle, Plus, RefreshCw, Search } from 'lucide-react'
import { useRuns } from '@/lib/api'
import type { Run, WorkflowPhase } from '@/lib/argo'
import { pipelines } from '@/pipelines/registry'
import { cn, useNow } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Input, Select } from '@/components/ui/controls'
import { LiveIndicator } from '@/components/runs/live-indicator'
import { RunsTable } from '@/components/runs/runs-table'
import { RunDrawer } from '@/components/runs/run-drawer'

type Filter = 'all' | 'Running' | 'Pending' | 'Succeeded' | 'failed'

const DAY = 86_400_000

function matchesFilter(run: Run, filter: Filter) {
  if (filter === 'all') return true
  if (filter === 'failed') return run.phase === 'Failed' || run.phase === 'Error'
  return run.phase === filter
}

export function DashboardPage() {
  const { runs, error, isLoading, mutate } = useRuns()
  const now = useNow()
  const [params, setParams] = useSearchParams()
  const [filter, setFilter] = useState<Filter>('all')
  const [query, setQuery] = useState('')
  const [pipelineId, setPipelineId] = useState('all')
  const selected = params.get('run')

  const counts = useMemo(() => {
    const recent = (r: Run) => now - new Date(r.createdAt).getTime() < DAY
    const is = (...phases: WorkflowPhase[]) => (r: Run) => phases.includes(r.phase)
    return {
      running: runs.filter(is('Running')).length,
      queued: runs.filter(is('Pending')).length,
      succeeded: runs.filter((r) => is('Succeeded')(r) && recent(r)).length,
      failed: runs.filter((r) => is('Failed', 'Error')(r) && recent(r)).length,
    }
  }, [runs, now])

  const visible = useMemo(() => {
    const q = query.trim().toLowerCase()
    return runs.filter(
      (r) =>
        matchesFilter(r, filter) &&
        (pipelineId === 'all' || r.pipelineId === pipelineId) &&
        (!q || r.name.toLowerCase().includes(q) || r.submittedBy.toLowerCase().includes(q)),
    )
  }, [runs, filter, query, pipelineId])

  const selectedRun = runs.find((r) => r.name === selected)

  const select = (name: string | null) => {
    const next = new URLSearchParams(params)
    if (name) next.set('run', name)
    else next.delete('run')
    setParams(next, { replace: true })
  }

  const summary: { key: Filter; label: string; value: number; tone: string }[] = [
    { key: 'Running', label: 'Running', value: counts.running, tone: 'text-primary' },
    { key: 'Pending', label: 'Queued', value: counts.queued, tone: 'text-foreground' },
    { key: 'Succeeded', label: 'Succeeded · 24h', value: counts.succeeded, tone: 'text-success' },
    { key: 'failed', label: 'Failed · 24h', value: counts.failed, tone: 'text-danger' },
  ]

  return (
    <div className="flex min-h-full">
      <div className="flex min-w-0 flex-1 flex-col gap-6 px-4 py-6 md:px-8 md:py-8">
        <div className="flex flex-wrap items-end justify-between gap-4">
          <div className="flex flex-col gap-1">
            <h1 className="text-2xl font-semibold tracking-tight text-foreground text-balance">Pipeline runs</h1>
            <p className="text-sm text-muted-foreground">Workflows submitted to the GeneLink orchestrator.</p>
          </div>
          <div className="flex items-center gap-2">
            <LiveIndicator />
            <Button variant="outline" size="icon" onClick={() => void mutate()} aria-label="Refresh runs">
              <RefreshCw className={cn(isLoading && 'animate-spin')} />
            </Button>
            <Link
              to={`/pipelines/${pipelines[0].id}`}
              className="inline-flex h-9 items-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-sm hover:bg-primary/90"
            >
              <Plus className="size-4" aria-hidden />
              New run
            </Link>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-px overflow-hidden rounded-lg border bg-border md:grid-cols-4">
          {summary.map((s) => {
            const active = filter === s.key
            return (
              <button
                key={s.label}
                type="button"
                onClick={() => setFilter(active ? 'all' : s.key)}
                aria-pressed={active}
                className={cn(
                  'flex flex-col items-start gap-1 bg-card px-5 py-4 text-left transition-colors hover:bg-muted/50',
                  active && 'bg-muted/70',
                )}
              >
                <span className="text-xs font-medium text-muted-foreground">{s.label}</span>
                <span className={cn('text-2xl font-semibold tabular-nums', s.tone)}>{s.value}</span>
              </button>
            )
          })}
        </div>

        <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
          <div className="relative flex-1">
            <Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" aria-hidden />
            <label htmlFor="run-search" className="sr-only">
              Search runs
            </label>
            <Input
              id="run-search"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search by run name or user"
              className="pl-9"
            />
          </div>
          <div className="sm:w-60">
            <label htmlFor="pipeline-filter" className="sr-only">
              Filter by pipeline
            </label>
            <Select
              id="pipeline-filter"
              value={pipelineId}
              onChange={(e) => setPipelineId(e.target.value)}
              options={[{ value: 'all', label: 'All pipelines' }, ...pipelines.map((p) => ({ value: p.id, label: p.name }))]}
            />
          </div>
        </div>

        {error ? (
          <div className="flex items-start gap-3 rounded-lg border border-danger/30 bg-danger/10 px-4 py-3 text-sm text-danger" role="alert">
            <AlertTriangle className="mt-0.5 size-4 shrink-0" aria-hidden />
            <div className="flex flex-col gap-1">
              <p className="font-medium">Could not load workflows from Argo.</p>
              <p className="leading-relaxed">{error.message}</p>
            </div>
          </div>
        ) : null}

        {isLoading && runs.length === 0 ? (
          <div className="flex flex-col gap-2 rounded-lg border bg-card p-4" aria-busy="true">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="h-10 animate-pulse rounded-md bg-muted" />
            ))}
          </div>
        ) : visible.length === 0 ? (
          <div className="flex flex-col items-center gap-2 rounded-lg border border-dashed bg-card px-6 py-16 text-center">
            <p className="text-sm font-medium text-foreground">No runs match these filters</p>
            <p className="text-sm text-muted-foreground">Try clearing the search or pick another status.</p>
          </div>
        ) : (
          <RunsTable runs={visible} now={now} selected={selected} onSelect={(name) => select(name)} />
        )}
      </div>

      {selectedRun ? (
        <div className="fixed inset-0 z-30 2xl:sticky 2xl:top-0 2xl:z-auto 2xl:h-dvh 2xl:w-[420px] 2xl:shrink-0">
          <button
            type="button"
            className="absolute inset-0 bg-foreground/30 2xl:hidden"
            aria-label="Close run details"
            onClick={() => select(null)}
          />
          <div className="absolute inset-y-0 right-0 w-full max-w-md shadow-xl 2xl:static 2xl:h-full 2xl:max-w-none 2xl:shadow-none">
            <RunDrawer run={selectedRun} now={now} onClose={() => select(null)} onChanged={() => void mutate()} />
          </div>
        </div>
      ) : null}
    </div>
  )
}
