import { useLiveStatus } from '@/lib/live'
import { cn } from '@/lib/utils'

const labels = {
  connecting: 'Connecting…',
  live: 'Live',
  reconnecting: 'Reconnecting…',
  offline: 'Polling',
} as const

export function LiveIndicator() {
  const { status } = useLiveStatus()
  return (
    <span
      className="inline-flex h-7 items-center gap-2 rounded-full border bg-card px-3 text-xs font-medium text-muted-foreground"
      role="status"
      aria-live="polite"
    >
      <span
        className={cn(
          'size-2 rounded-full',
          status === 'live' && 'bg-success animate-pulse-dot',
          (status === 'connecting' || status === 'reconnecting') && 'bg-warning animate-pulse-dot',
          status === 'offline' && 'bg-muted-foreground/50',
        )}
        aria-hidden
      />
      {labels[status]}
    </span>
  )
}
