import { cn } from '@/lib/utils'
import type { NodePhase, RunStep } from '@/lib/argo'

const phaseStyles: Record<NodePhase, { label: string; className: string; dot: string }> = {
  Pending: { label: 'Queued', className: 'bg-muted text-muted-foreground', dot: 'bg-muted-foreground/60' },
  Running: { label: 'Running', className: 'bg-primary/10 text-primary', dot: 'bg-primary animate-pulse-dot' },
  Succeeded: { label: 'Succeeded', className: 'bg-success/10 text-success', dot: 'bg-success' },
  Failed: { label: 'Failed', className: 'bg-danger/10 text-danger', dot: 'bg-danger' },
  Error: { label: 'Error', className: 'bg-danger/10 text-danger', dot: 'bg-danger' },
  Skipped: { label: 'Skipped', className: 'bg-muted text-muted-foreground', dot: 'bg-muted-foreground/40' },
  Omitted: { label: 'Omitted', className: 'bg-muted text-muted-foreground', dot: 'bg-muted-foreground/40' },
}

export function PhaseBadge({ phase, className }: { phase: NodePhase; className?: string }) {
  const style = phaseStyles[phase]
  return (
    <span
      className={cn(
        'inline-flex h-6 items-center gap-1.5 rounded-full px-2.5 text-xs font-medium whitespace-nowrap',
        style.className,
        className,
      )}
    >
      <span className={cn('size-1.5 rounded-full', style.dot)} aria-hidden />
      {style.label}
    </span>
  )
}

const segmentColor: Record<NodePhase, string> = {
  Pending: 'bg-muted-foreground/15',
  Running: 'bg-primary animate-pulse-dot',
  Succeeded: 'bg-success',
  Failed: 'bg-danger',
  Error: 'bg-danger',
  Skipped: 'bg-muted-foreground/25',
  Omitted: 'bg-muted-foreground/10',
}

export function StepTrack({ steps, className }: { steps: RunStep[]; className?: string }) {
  if (steps.length === 0) return <div className={cn('h-1.5 rounded-full bg-muted', className)} />
  return (
    <div className={cn('flex gap-0.5', className)} aria-hidden>
      {steps.map((step) => (
        <span
          key={step.id}
          title={`${step.name} — ${phaseStyles[step.phase].label}`}
          className={cn('h-1.5 flex-1 rounded-full first:rounded-l-full', segmentColor[step.phase])}
        />
      ))}
    </div>
  )
}

export { phaseStyles }
