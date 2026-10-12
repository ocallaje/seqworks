import { useEffect, useState } from 'react'
import { ExternalLink, RotateCcw, Square, X } from 'lucide-react'
import type { Run } from '@/lib/argo'
import { argoUiLink, retryRun, stopRun } from '@/lib/api'
import { useAuth } from '@/lib/auth'
import { getPipeline } from '@/pipelines/registry'
import { formatDateTime, formatDuration } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { PhaseBadge, StepTrack } from './status'
import { runDuration } from './runs-table'

interface RunDrawerProps {
  run: Run
  now: number
  onClose: () => void
  onChanged: () => void
}

export function RunDrawer({ run, now, onClose, onChanged }: RunDrawerProps) {
  const { session } = useAuth()
  const [pending, setPending] = useState<'stop' | 'retry' | null>(null)
  const [error, setError] = useState<string | null>(null)
  const pipeline = getPipeline(run.pipelineId)
  const link = argoUiLink(run.name)
  const isActive = run.phase === 'Running' || run.phase === 'Pending'

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => e.key === 'Escape' && onClose()
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [onClose])

  const act = async (action: 'stop' | 'retry') => {
    if (!session) return
    setPending(action)
    setError(null)
    try {
      await (action === 'stop' ? stopRun : retryRun)(session.accessToken, run.name)
      onChanged()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Action failed')
    } finally {
      setPending(null)
    }
  }

  return (
    <aside
      className="animate-drawer-in flex h-full flex-col border-l bg-card"
      aria-labelledby="run-drawer-title"
    >
      <div className="flex items-start justify-between gap-4 border-b px-5 py-4">
        <div className="flex min-w-0 flex-col gap-1.5">
          <p className="text-xs text-muted-foreground">{pipeline?.name ?? run.pipelineId}</p>
          <h2 id="run-drawer-title" className="truncate font-mono text-base font-medium text-foreground">
            {run.name}
          </h2>
          <div>
            <PhaseBadge phase={run.phase} />
          </div>
        </div>
        <Button variant="ghost" size="icon" onClick={onClose} aria-label="Close run details">
          <X />
        </Button>
      </div>

      <div className="flex flex-1 flex-col gap-6 overflow-y-auto px-5 py-5">
        <dl className="grid grid-cols-2 gap-x-4 gap-y-3 text-sm">
          <div className="flex flex-col gap-0.5">
            <dt className="text-xs text-muted-foreground">Started</dt>
            <dd className="text-foreground">{formatDateTime(run.startedAt)}</dd>
          </div>
          <div className="flex flex-col gap-0.5">
            <dt className="text-xs text-muted-foreground">Duration</dt>
            <dd className="font-mono tabular-nums text-foreground">{runDuration(run, now)}</dd>
          </div>
          <div className="flex flex-col gap-0.5">
            <dt className="text-xs text-muted-foreground">Submitted by</dt>
            <dd className="text-foreground">{run.submittedBy}</dd>
          </div>
          <div className="flex flex-col gap-0.5">
            <dt className="text-xs text-muted-foreground">Workflow</dt>
            <dd className="font-mono text-xs text-foreground">
              {pipeline ? `${pipeline.workflow}@${pipeline.version}` : '—'}
            </dd>
          </div>
        </dl>

        {run.message ? (
          <p className="rounded-md border border-danger/30 bg-danger/10 px-3 py-2 font-mono text-xs leading-relaxed text-danger">
            {run.message}
          </p>
        ) : null}

        <section className="flex flex-col gap-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold text-foreground">Steps</h3>
            <span className="font-mono text-xs tabular-nums text-muted-foreground">
              {run.progress.done}/{run.progress.total}
            </span>
          </div>
          <StepTrack steps={run.steps} />
          <ol className="flex flex-col divide-y rounded-lg border">
            {run.steps.map((step) => {
              const duration = step.startedAt
                ? formatDuration((step.finishedAt ? new Date(step.finishedAt).getTime() : now) - new Date(step.startedAt).getTime())
                : '—'
              return (
                <li key={step.id} className="flex items-center justify-between gap-3 px-3 py-2.5">
                  <span className="truncate text-sm text-foreground">{step.name}</span>
                  <span className="flex items-center gap-3">
                    <span className="font-mono text-xs tabular-nums text-muted-foreground">{duration}</span>
                    <PhaseBadge phase={step.phase} />
                  </span>
                </li>
              )
            })}
          </ol>
        </section>

        {run.parameters.length > 0 ? (
          <section className="flex flex-col gap-3">
            <h3 className="text-sm font-semibold text-foreground">Parameters</h3>
            <dl className="flex flex-col divide-y rounded-lg border bg-muted/30">
              {run.parameters.map((p) => (
                <div key={p.name} className="flex flex-col gap-0.5 px-3 py-2">
                  <dt className="font-mono text-xs text-muted-foreground">{p.name}</dt>
                  <dd className="font-mono text-xs break-all text-foreground">{p.value || '—'}</dd>
                </div>
              ))}
            </dl>
          </section>
        ) : null}
      </div>

      <div className="flex flex-col gap-2 border-t px-5 py-4">
        {error ? (
          <p className="text-sm text-danger" role="alert">
            {error}
          </p>
        ) : null}
        <div className="flex flex-wrap gap-2">
          {isActive ? (
            <Button variant="danger" onClick={() => void act('stop')} disabled={pending !== null}>
              <Square />
              {pending === 'stop' ? 'Stopping…' : 'Stop run'}
            </Button>
          ) : null}
          {run.phase === 'Failed' || run.phase === 'Error' ? (
            <Button variant="outline" onClick={() => void act('retry')} disabled={pending !== null}>
              <RotateCcw />
              {pending === 'retry' ? 'Retrying…' : 'Retry failed steps'}
            </Button>
          ) : null}
          {link ? (
            <a
              href={link}
              target="_blank"
              rel="noreferrer"
              className="ml-auto inline-flex h-9 items-center gap-2 rounded-md px-3 text-sm font-medium text-muted-foreground hover:bg-muted hover:text-foreground"
            >
              Open in Argo
              <ExternalLink className="size-4" aria-hidden />
            </a>
          ) : null}
        </div>
      </div>
    </aside>
  )
}
