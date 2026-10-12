import type { Run } from '@/lib/argo'
import { getPipeline } from '@/pipelines/registry'
import { cn, formatDuration, formatRelative } from '@/lib/utils'
import { PhaseBadge, StepTrack } from './status'

interface RunsTableProps {
  runs: Run[]
  now: number
  selected?: string | null
  onSelect: (name: string) => void
}

export function runDuration(run: Run, now: number) {
  if (!run.startedAt) return '—'
  const end = run.finishedAt ? new Date(run.finishedAt).getTime() : now
  return formatDuration(end - new Date(run.startedAt).getTime())
}

export function RunsTable({ runs, now, selected, onSelect }: RunsTableProps) {
  return (
    <div className="overflow-x-auto rounded-lg border bg-card">
      <table className="w-full min-w-[820px] text-left text-sm whitespace-nowrap">
        <thead className="border-b bg-muted/40 text-xs text-muted-foreground">
          <tr>
            <th scope="col" className="px-4 py-2.5 font-medium">Run</th>
            <th scope="col" className="px-4 py-2.5 font-medium">Status</th>
            <th scope="col" className="w-56 px-4 py-2.5 font-medium">Progress</th>
            <th scope="col" className="px-4 py-2.5 font-medium">Submitted by</th>
            <th scope="col" className="px-4 py-2.5 font-medium">Started</th>
            <th scope="col" className="px-4 py-2.5 text-right font-medium">Duration</th>
          </tr>
        </thead>
        <tbody className="divide-y">
          {runs.map((run) => {
            const pipeline = getPipeline(run.pipelineId)
            const isSelected = selected === run.name
            return (
              <tr
                key={run.uid}
                onClick={() => onSelect(run.name)}
                className={cn('cursor-pointer transition-colors hover:bg-muted/50', isSelected && 'bg-primary/5 hover:bg-primary/5')}
              >
                <td className="px-4 py-3">
                  <button
                    type="button"
                    onClick={(e) => {
                      e.stopPropagation()
                      onSelect(run.name)
                    }}
                    className="flex flex-col items-start gap-0.5 text-left"
                    aria-pressed={isSelected}
                  >
                    <span className="font-mono text-sm text-foreground">{run.name}</span>
                    <span className="text-xs text-muted-foreground">{pipeline?.name ?? run.pipelineId}</span>
                  </button>
                </td>
                <td className="px-4 py-3">
                  <PhaseBadge phase={run.phase} />
                </td>
                <td className="px-4 py-3">
                  <div className="flex w-48 items-center gap-3">
                    <StepTrack steps={run.steps} className="flex-1" />
                    <span className="w-9 text-right font-mono text-xs tabular-nums text-muted-foreground">
                      {run.progress.done}/{run.progress.total}
                    </span>
                  </div>
                </td>
                <td className="px-4 py-3 text-muted-foreground">{run.submittedBy}</td>
                <td className="px-4 py-3 whitespace-nowrap text-muted-foreground">
                  {formatRelative(run.startedAt ?? run.createdAt, now)}
                </td>
                <td className="px-4 py-3 text-right font-mono text-xs tabular-nums whitespace-nowrap text-foreground">
                  {runDuration(run, now)}
                </td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )
}
